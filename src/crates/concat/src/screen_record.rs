// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: recording the screen.
//!
//! The Record screen row (File menu) and button (Media page) start it. A
//! three-second countdown runs in the toasts, the window is minimised, and
//! an ffmpeg program records the screen into the project's `recordings`
//! folder until the person presses Ctrl+Shift+R - a hotkey that works over
//! any other program, and exists only while a recording is armed or
//! running - or chooses the same row again. The window then comes back,
//! and the file comes into the media bin like any other.
//!
//! ffmpeg is not part of UFO Cut's own build: it is the same program the
//! link downloader fetches once into the app's data folder, and it runs on
//! this machine as a program of its own. Nothing is recorded unless the
//! person chose it, nothing is recorded in the background, and nothing is
//! sent anywhere: the only network use is that one-time fetch.
//!
//! What is captured: the whole desktop, every screen together, with the
//! mouse pointer. Not captured: the sound the computer plays, and any
//! window the system protects, which comes out black.
//!
//! What the arguments are, and what the file is called, is the pure part
//! and lives in `concat_host::screen`, with its tests.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant};

use concat_host::screen::{self, Settings, Stamp};
use slint::ComponentHandle;

use crate::i18n::{t, tf};
use crate::panes::Msg;
use crate::panes::media_bin::MediaMsg;
use crate::studio::Studio;
use crate::ui::App;

/// What the Media page's Record screen button sends in place of a link's
/// quality: see the handler of `import-link` in lib.rs.
pub const RECORD_CODE: i32 = 101;

/// The frame rate of a recording.
// TODO(owner): read these four from the preferences once a settings page
// for the recorder exists; until then every recording is made with them.
const FPS: u32 = 30;
/// Whether the mouse pointer is drawn into a recording.
const CURSOR: bool = true;
/// The DirectShow name of the microphone, or none for a silent recording.
const MIC: Option<&str> = None;

/// The program's file name on this system.
#[cfg(windows)]
const RECORDER: &str = "ffmpeg.exe";
#[cfg(not(windows))]
const RECORDER: &str = "ffmpeg";

/// Nothing is happening.
const IDLE: u8 = 0;
/// The program is being fetched, the countdown is running, or the program
/// is starting.
const PREPARING: u8 = 1;
/// The program is recording.
const RECORDING: u8 = 2;
/// The program is being told to finish.
const STOPPING: u8 = 3;

/// Where a recording is: one of the four above.
static PHASE: AtomicU8 = AtomicU8::new(IDLE);
/// Set when the person asked to stop before the recording had begun.
static CANCEL: AtomicBool = AtomicBool::new(false);
/// The recording that is running.
static ACTIVE: Mutex<Option<Active>> = Mutex::new(None);

/// A recording that is running.
struct Active {
    /// ffmpeg, with its standard input open to be told `q`.
    child: Child,
    /// The file it is writing.
    file: PathBuf,
}

/// How the start went.
enum Started {
    /// ffmpeg is recording.
    Recording,
    /// The person stopped it during the countdown.
    Cancelled,
    /// There is no ffmpeg program and none could be fetched.
    NoRecorder(String),
    /// ffmpeg would not run.
    CannotStart(String),
}

/// What a recording is made with.
fn settings() -> Settings {
    Settings {
        target: screen::Target::FullScreen,
        fps: FPS,
        cursor: CURSOR,
        mic: MIC.map(str::to_owned),
        max_seconds: screen::DEFAULT_MAX_SECONDS,
    }
}

/// The Record screen row, the button and the hotkey: starts a recording
/// when none is going, stops the one that is.
pub fn toggle(studio: &mut Studio) {
    if !cfg!(windows) {
        studio.notify(&t("screen.windowsOnly"), true);
        return;
    }
    match PHASE.load(Ordering::SeqCst) {
        IDLE => begin(studio),
        PREPARING => CANCEL.store(true, Ordering::SeqCst),
        RECORDING => stop(),
        _ => {}
    }
}

/// Starts a recording of the screen into the open project's folder.
fn begin(studio: &mut Studio) {
    let Some(session) = studio.session.as_ref() else {
        studio.notify(&t("sounds.openProjectFirst"), true);
        return;
    };
    let folder = PathBuf::from(session.path()).join("recordings");
    let tools = studio.host.dirs.data.join("tools");
    if PHASE
        .compare_exchange(IDLE, PREPARING, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    CANCEL.store(false, Ordering::SeqCst);
    if !tools.join(RECORDER).is_file() {
        studio.notify(&t("screen.getting"), false);
    }
    crate::host::spawn(
        move || prepare(&tools, &folder),
        |studio, app, _models, started: Started| after_start(studio, app, started),
    );
}

/// Fetches the program if need be, counts down, and starts it. Runs on a
/// thread of its own.
fn prepare(tools: &Path, folder: &Path) -> Started {
    let program = match recorder(tools) {
        Ok(program) => program,
        Err(error) => return Started::NoRecorder(error),
    };
    // The key that stops it, before anything is hidden: without it the
    // window stays where it is, and the row stops the recording.
    let armed = hotkey::arm(fired).is_ok();
    if !armed {
        crate::host::on_ui(|studio, _, _| studio.notify(&t("screen.noHotkey"), true));
    }
    for number in [3_u32, 2, 1] {
        if CANCEL.load(Ordering::SeqCst) {
            return Started::Cancelled;
        }
        crate::host::on_ui(move |studio, _, _| {
            studio.notify(&tf("screen.countdown", &[&number]), false);
        });
        std::thread::sleep(Duration::from_secs(1));
    }
    if CANCEL.load(Ordering::SeqCst) {
        return Started::Cancelled;
    }
    if armed {
        crate::host::on_ui(|_, app, _| crate::platform::minimize(app.window()));
        // Long enough for the window to be gone from the picture.
        std::thread::sleep(Duration::from_millis(700));
    }
    if let Err(error) = std::fs::create_dir_all(folder) {
        return Started::CannotStart(error.to_string());
    }
    let file = screen::output_name(folder, &local_stamp());
    let log = tools.join("screen-recorder.log");
    let mut command = Command::new(&program);
    command.args(screen::ffmpeg_args(&settings(), &file));
    quiet(&mut command);
    let complaints = match File::create(&log) {
        Ok(handle) => Stdio::from(handle),
        Err(_) => Stdio::null(),
    };
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(complaints)
        .spawn();
    let mut child = match child {
        Ok(child) => child,
        Err(error) => return Started::CannotStart(error.to_string()),
    };
    // A program that is going to refuse - no such window, no such
    // microphone - does so at once.
    std::thread::sleep(Duration::from_millis(1500));
    if let Ok(Some(_)) = child.try_wait() {
        return Started::CannotStart(last_line(&log));
    }
    {
        let mut slot = ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *slot = Some(Active { child, file });
    }
    PHASE.store(RECORDING, Ordering::SeqCst);
    Started::Recording
}

/// What the start came to, on the window's thread.
fn after_start(studio: &mut Studio, app: &App, started: Started) {
    match started {
        Started::Recording => {
            studio.notify(&t("screen.recording"), false);
            // Stopped before it had begun.
            if CANCEL.swap(false, Ordering::SeqCst) {
                stop();
            }
        }
        Started::Cancelled => {
            reset(app);
            studio.notify(&t("screen.cancelled"), false);
        }
        Started::NoRecorder(error) => {
            reset(app);
            studio.notify(&tf("screen.noFfmpeg", &[&error]), true);
        }
        Started::CannotStart(error) => {
            reset(app);
            studio.notify(&tf("screen.cannotStart", &[&error]), true);
        }
    }
}

/// Back to nothing happening: the key given up and the window brought back.
fn reset(app: &App) {
    hotkey::disarm();
    PHASE.store(IDLE, Ordering::SeqCst);
    CANCEL.store(false, Ordering::SeqCst);
    restore(app);
}

/// Brings the window back from the taskbar.
fn restore(app: &App) {
    app.window().set_minimized(false);
}

/// Finishes the recording that is running.
fn stop() {
    if PHASE
        .compare_exchange(RECORDING, STOPPING, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    crate::host::spawn(
        end_recording,
        |studio, app, _models, saved: Option<PathBuf>| after_stop(studio, app, saved),
    );
}

/// Tells ffmpeg to finish, waits for it to close the file, and answers
/// with the file when something was recorded. Runs on a thread of its own.
fn end_recording() -> Option<PathBuf> {
    let active = {
        let mut slot = ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        slot.take()
    };
    hotkey::disarm();
    let mut active = active?;
    finish(&mut active, Duration::from_secs(8));
    let size = std::fs::metadata(&active.file)
        .map(|meta| meta.len())
        .unwrap_or(0);
    if size == 0 { None } else { Some(active.file) }
}

/// Asks ffmpeg to close its file with `q`, waits up to `wait` for it, and
/// kills it when it has not finished by then.
fn finish(active: &mut Active, wait: Duration) {
    // `q` is how ffmpeg is asked to close a file properly.
    if let Some(mut input) = active.child.stdin.take() {
        let _ = input.write_all(b"q\n");
        let _ = input.flush();
    }
    let deadline = Instant::now() + wait;
    let mut finished = false;
    while Instant::now() < deadline {
        match active.child.try_wait() {
            Ok(Some(_)) => {
                finished = true;
                break;
            }
            Ok(None) => {}
            Err(_) => break,
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    if !finished {
        let _ = active.child.kill();
        let _ = active.child.wait();
    }
}

/// UFO Cut is closing: a recording that is still running is finished here,
/// on the calling thread, so ffmpeg does not outlive the program. Called
/// once the window's loop has ended; touches no window.
pub fn stop_all_blocking() {
    CANCEL.store(true, Ordering::SeqCst);
    let active = {
        let mut slot = ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        slot.take()
    };
    hotkey::disarm();
    if let Some(mut active) = active {
        finish(&mut active, Duration::from_secs(4));
    }
}

/// The recording is closed: the window comes back and the file with it.
fn after_stop(studio: &mut Studio, app: &App, saved: Option<PathBuf>) {
    PHASE.store(IDLE, Ordering::SeqCst);
    CANCEL.store(false, Ordering::SeqCst);
    restore(app);
    match saved {
        Some(file) => {
            let name = file
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if studio.session.is_some() {
                studio.handle(Msg::Media(MediaMsg::Import(vec![file])));
            }
            studio.notify(&tf("screen.saved", &[&name]), false);
        }
        None => studio.notify(&t("screen.nothing"), true),
    }
}

/// The hotkey was pressed: the same as the row.
fn fired() {
    crate::host::on_ui(|studio, _, _| toggle(studio));
}

/// The last line ffmpeg wrote to its log, for saying why it stopped.
fn last_line(log: &Path) -> String {
    std::fs::read_to_string(log)
        .ok()
        .and_then(|text| {
            text.lines()
                .rev()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "ffmpeg gave no answer".to_owned())
}

/// The ffmpeg program, fetched once when it is not here yet: the same
/// ready-made Windows build, to the same place, as the link downloader's
/// (see `link_import::joiner`, which is private to that module).
fn recorder(tools: &Path) -> Result<PathBuf, String> {
    let path = tools.join(RECORDER);
    if path.is_file() {
        return Ok(path);
    }
    if !cfg!(windows) {
        return Err("no ffmpeg program".to_owned());
    }
    std::fs::create_dir_all(tools).map_err(|error| error.to_string())?;
    let partial = tools.join("ffmpeg.partial");
    let never = AtomicBool::new(false);
    concat_host::models::download(
        "https://github.com/eugeneware/ffmpeg-static/releases/latest/download/ffmpeg-win32-x64",
        &partial,
        83_000_000,
        &never,
        "cancelled",
        &mut |_, _| {},
    )?;
    std::fs::rename(&partial, &path).map_err(|error| error.to_string())?;
    Ok(path)
}

/// Keeps a console window from flashing up behind the editor on Windows.
fn quiet(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    let _ = command;
}

/// The local date and time, for the file's name.
#[cfg(windows)]
fn local_stamp() -> Stamp {
    /// Windows' SYSTEMTIME.
    #[repr(C)]
    struct SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLocalTime(time: *mut SystemTime);
    }
    let mut now = SystemTime {
        year: 1970,
        month: 1,
        day_of_week: 0,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        milliseconds: 0,
    };
    unsafe { GetLocalTime(&mut now) };
    Stamp {
        year: i32::from(now.year),
        month: u32::from(now.month),
        day: u32::from(now.day),
        hour: u32::from(now.hour),
        minute: u32::from(now.minute),
        second: u32::from(now.second),
    }
}

/// The date and time, in UTC, for the file's name.
#[cfg(not(windows))]
fn local_stamp() -> Stamp {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    Stamp::from_unix_utc(seconds)
}

/// The key that stops a recording while another program is in front:
/// Ctrl+Shift+R, registered with Windows for as long as a recording is
/// armed or running, and given up when it ends.
#[cfg(windows)]
mod hotkey {
    use std::sync::Mutex;
    use std::sync::mpsc;
    use std::thread::JoinHandle;
    use std::time::Duration;

    /// Windows' MOD_CONTROL.
    const MOD_CONTROL: u32 = 0x0002;
    /// Windows' MOD_SHIFT.
    const MOD_SHIFT: u32 = 0x0004;
    /// Windows' MOD_NOREPEAT: holding the keys down is one press.
    const MOD_NOREPEAT: u32 = 0x4000;
    /// The R key.
    const VK_R: u32 = 0x52;
    /// What Windows posts when the registered keys are pressed.
    const WM_HOTKEY: u32 = 0x0312;
    /// What ends a thread's message loop.
    const WM_QUIT: u32 = 0x0012;
    /// PeekMessageW's "leave the message where it is".
    const PM_NOREMOVE: u32 = 0;
    /// This registration's number, to tell it from another's.
    const HOTKEY_ID: i32 = 0x5546;

    /// Windows' POINT.
    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }

    /// Windows' MSG.
    #[repr(C)]
    struct Msg {
        hwnd: isize,
        message: u32,
        wparam: usize,
        lparam: isize,
        time: u32,
        pt: Point,
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn RegisterHotKey(hwnd: isize, id: i32, modifiers: u32, vk: u32) -> i32;
        fn UnregisterHotKey(hwnd: isize, id: i32) -> i32;
        fn GetMessageW(msg: *mut Msg, hwnd: isize, min: u32, max: u32) -> i32;
        fn PeekMessageW(msg: *mut Msg, hwnd: isize, min: u32, max: u32, remove: u32) -> i32;
        fn PostThreadMessageW(thread: u32, message: u32, wparam: usize, lparam: isize) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentThreadId() -> u32;
    }

    /// The thread that holds the key: its id, to be told to end, and its
    /// handle, to be waited for.
    static RUNNING: Mutex<Option<(u32, JoinHandle<()>)>> = Mutex::new(None);

    /// Registers the key and calls `fired` on a thread of its own each time
    /// it is pressed. Fails, saying why, when another program holds it.
    pub fn arm(fired: fn()) -> Result<(), String> {
        disarm();
        let (sender, receiver) = mpsc::channel::<Result<u32, String>>();
        let handle = std::thread::spawn(move || run(fired, sender));
        match receiver.recv_timeout(Duration::from_secs(3)) {
            Ok(Ok(thread)) => {
                let mut slot = RUNNING.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                *slot = Some((thread, handle));
                Ok(())
            }
            Ok(Err(error)) => {
                let _ = handle.join();
                Err(error)
            }
            Err(_) => Err("the key did not answer".to_owned()),
        }
    }

    /// Gives the key up. Nothing happens when it is not held.
    pub fn disarm() {
        let held = {
            let mut slot = RUNNING.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            slot.take()
        };
        if let Some((thread, handle)) = held {
            unsafe { PostThreadMessageW(thread, WM_QUIT, 0, 0) };
            let _ = handle.join();
        }
    }

    /// The key's thread: holds the registration, which belongs to the thread
    /// that made it, and waits for presses until told to end.
    fn run(fired: fn(), sender: mpsc::Sender<Result<u32, String>>) {
        let mut msg = Msg {
            hwnd: 0,
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            pt: Point { x: 0, y: 0 },
        };
        let held = unsafe {
            RegisterHotKey(0, HOTKEY_ID, MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT, VK_R)
        };
        if held == 0 {
            let _ = sender.send(Err("Ctrl+Shift+R is held by another program".to_owned()));
            return;
        }
        // A thread has no message queue until it asks for one; the ending
        // message is posted to it.
        unsafe { PeekMessageW(&mut msg, 0, 0, 0, PM_NOREMOVE) };
        let thread = unsafe { GetCurrentThreadId() };
        let _ = sender.send(Ok(thread));
        loop {
            let got = unsafe { GetMessageW(&mut msg, 0, 0, 0) };
            if got <= 0 {
                break;
            }
            if msg.message == WM_HOTKEY && msg.wparam == HOTKEY_ID as usize {
                fired();
            }
        }
        unsafe { UnregisterHotKey(0, HOTKEY_ID) };
    }
}

/// Windows only.
#[cfg(not(windows))]
mod hotkey {
    /// There is no key to hold.
    pub fn arm(_fired: fn()) -> Result<(), String> {
        Err("only on Windows".to_owned())
    }

    /// There is no key to give up.
    pub fn disarm() {}
}
