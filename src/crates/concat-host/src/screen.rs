// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: recording the screen. The pure part.
//!
//! The screen is recorded by an ffmpeg program that runs on this machine
//! as a program of its own (see `concat/src/screen_record.rs`, which
//! starts and stops it). Nothing here starts a process or touches the
//! screen: this module only says what to ask ffmpeg for, what to call the
//! file, and how to read ffmpeg's list of sound devices, so all of it can
//! be tested without a screen.
//!
//! Windows' own grabber (`gdigrab`) takes the picture, and DirectShow
//! (`dshow`) takes the microphone. The screen's sound ("what you hear")
//! is not recorded, and a window the system protects (copy-protected
//! video) comes out black.

use std::path::{Path, PathBuf};

/// The frame rates the recorder offers.
pub const FRAME_RATES: [u32; 4] = [15, 24, 30, 60];

/// The frame rate when none was asked for.
pub const DEFAULT_FPS: u32 = 30;

/// The longest a recording may run, in seconds: four hours. A guard so a
/// recording that nobody can stop (the program that started it has gone)
/// ends by itself.
pub const DEFAULT_MAX_SECONDS: u32 = 4 * 60 * 60;

/// What is recorded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// The whole desktop, every screen together.
    FullScreen,
    /// One window, by the title it shows.
    Window(String),
    /// A rectangle of the desktop: its top-left corner and its size, in
    /// pixels.
    Region {
        /// The left edge.
        x: i32,
        /// The top edge.
        y: i32,
        /// The width.
        w: u32,
        /// The height.
        h: u32,
    },
}

/// How a recording is made.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// What is recorded.
    pub target: Target,
    /// Frames a second: one of [`FRAME_RATES`]; another is read as
    /// [`DEFAULT_FPS`].
    pub fps: u32,
    /// Whether the mouse pointer is drawn into the picture.
    pub cursor: bool,
    /// The DirectShow name of the microphone to record with, or None for
    /// a recording without sound.
    pub mic: Option<String>,
    /// The recording stops by itself after this many seconds.
    pub max_seconds: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            target: Target::FullScreen,
            fps: DEFAULT_FPS,
            cursor: true,
            mic: None,
            max_seconds: DEFAULT_MAX_SECONDS,
        }
    }
}

/// `fps` when the recorder offers it, [`DEFAULT_FPS`] otherwise.
pub fn clean_fps(fps: u32) -> u32 {
    if FRAME_RATES.contains(&fps) {
        fps
    } else {
        DEFAULT_FPS
    }
}

/// Appends each of `items` to `args`.
fn add(args: &mut Vec<String>, items: &[&str]) {
    for item in items {
        args.push((*item).to_owned());
    }
}

/// The arguments ffmpeg is run with to record `settings` into `output`.
///
/// The picture is made an even number of pixels wide and high, which the
/// H.264 encoder needs; the file is MP4 with its index moved to the front
/// so it opens at once.
pub fn ffmpeg_args(settings: &Settings, output: &Path) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    add(&mut args, &["-hide_banner", "-loglevel", "error", "-nostats"]);

    // The picture.
    add(&mut args, &["-f", "gdigrab", "-framerate"]);
    args.push(clean_fps(settings.fps).to_string());
    add(
        &mut args,
        &["-draw_mouse", if settings.cursor { "1" } else { "0" }],
    );
    match &settings.target {
        Target::FullScreen => add(&mut args, &["-i", "desktop"]),
        Target::Window(title) => {
            args.push("-i".to_owned());
            args.push(format!("title={title}"));
        }
        Target::Region { x, y, w, h } => {
            args.push("-offset_x".to_owned());
            args.push(x.to_string());
            args.push("-offset_y".to_owned());
            args.push(y.to_string());
            args.push("-video_size".to_owned());
            args.push(format!("{}x{}", (*w).max(2), (*h).max(2)));
            add(&mut args, &["-i", "desktop"]);
        }
    }

    // The microphone, as a second input.
    let mic = settings.mic.as_deref().filter(|name| !name.is_empty());
    if let Some(name) = mic {
        add(&mut args, &["-f", "dshow", "-i"]);
        args.push(format!("audio={name}"));
    }

    // What is made of it.
    add(
        &mut args,
        &[
            "-vf",
            "scale=trunc(iw/2)*2:trunc(ih/2)*2",
            "-c:v",
            "libx264",
            "-preset",
            "ultrafast",
            "-crf",
            "20",
            "-pix_fmt",
            "yuv420p",
        ],
    );
    if mic.is_some() {
        add(&mut args, &["-c:a", "aac", "-b:a", "160k"]);
    }
    add(&mut args, &["-movflags", "+faststart", "-t"]);
    args.push(settings.max_seconds.max(1).to_string());
    args.push("-y".to_owned());
    args.push(output.to_string_lossy().into_owned());
    args
}

/// A calendar date and a time of day, for naming a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stamp {
    /// The year, four digits.
    pub year: i32,
    /// The month, 1 to 12.
    pub month: u32,
    /// The day of the month, 1 to 31.
    pub day: u32,
    /// The hour, 0 to 23.
    pub hour: u32,
    /// The minute, 0 to 59.
    pub minute: u32,
    /// The second, 0 to 59.
    pub second: u32,
}

impl Stamp {
    /// The date and time in UTC of `secs` seconds after 1 January 1970.
    pub fn from_unix_utc(secs: u64) -> Stamp {
        let days = (secs / 86_400) as i64;
        let rest = secs % 86_400;
        // The civil date of a day count, by Howard Hinnant's method.
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let day_of_era = z.rem_euclid(146_097);
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
        let month = if shifted_month < 10 {
            shifted_month + 3
        } else {
            shifted_month - 9
        };
        let year = if month <= 2 { year + 1 } else { year };
        Stamp {
            year: year as i32,
            month: month as u32,
            day: day as u32,
            hour: (rest / 3_600) as u32,
            minute: (rest % 3_600 / 60) as u32,
            second: (rest % 60) as u32,
        }
    }
}

/// The file a recording made at `now` goes to in `folder`:
/// `Screen_2026-11-14_09-30-05.mp4`. When that name is taken, as judged by
/// `exists`, `_2`, `_3` and so on are tried, so nothing is overwritten.
pub fn output_name_with(folder: &Path, now: &Stamp, exists: &dyn Fn(&Path) -> bool) -> PathBuf {
    let base = format!(
        "Screen_{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        now.year, now.month, now.day, now.hour, now.minute, now.second
    );
    let first = folder.join(format!("{base}.mp4"));
    if !exists(&first) {
        return first;
    }
    for number in 2..1000 {
        let next = folder.join(format!("{base}_{number}.mp4"));
        if !exists(&next) {
            return next;
        }
    }
    folder.join(format!("{base}_1000.mp4"))
}

/// [`output_name_with`] against the disk.
pub fn output_name(folder: &Path, now: &Stamp) -> PathBuf {
    output_name_with(folder, now, &|path: &Path| path.exists())
}

/// The audio devices in what `ffmpeg -list_devices true -f dshow -i dummy`
/// prints, in the order it lists them, each once.
///
/// Two layouts exist. Newer ffmpeg lists every device once, with its kinds
/// in brackets after the name (`"Mic" (audio)`); older ffmpeg lists the
/// video devices and then the audio devices under a heading each, with the
/// names bare. Both are read, and the lines that only give a device's
/// alternative name are skipped.
pub fn dshow_devices_from_ffmpeg_output(text: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut in_audio = false;
    for line in text.lines() {
        if line.contains("DirectShow video devices") {
            in_audio = false;
            continue;
        }
        if line.contains("DirectShow audio devices") {
            in_audio = true;
            continue;
        }
        if line.contains("Alternative name") {
            continue;
        }
        let Some(open) = line.find('"') else {
            continue;
        };
        let after = &line[open + 1..];
        let Some(close) = after.find('"') else {
            continue;
        };
        let name = &after[..close];
        if name.is_empty() {
            continue;
        }
        let tail = after[close + 1..].trim();
        let audio = if tail.starts_with('(') {
            tail.contains("audio")
        } else {
            in_audio
        };
        if audio && !names.iter().any(|known| known == name) {
            names.push(name.to_owned());
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    fn joined(settings: &Settings) -> String {
        ffmpeg_args(settings, Path::new("out.mp4")).join(" ")
    }

    #[test]
    fn the_whole_screen_at_the_defaults() {
        assert_eq!(
            joined(&Settings::default()),
            "-hide_banner -loglevel error -nostats -f gdigrab -framerate 30 -draw_mouse 1 \
             -i desktop -vf scale=trunc(iw/2)*2:trunc(ih/2)*2 -c:v libx264 -preset ultrafast \
             -crf 20 -pix_fmt yuv420p -movflags +faststart -t 14400 -y out.mp4"
        );
    }

    #[test]
    fn a_window_is_named_by_its_title() {
        let settings = Settings {
            target: Target::Window("Untitled - Notepad".to_owned()),
            ..Settings::default()
        };
        let args = ffmpeg_args(&settings, Path::new("out.mp4"));
        let at = args.iter().position(|arg| arg == "-i").unwrap();
        assert_eq!(args[at + 1], "title=Untitled - Notepad");
        assert!(!args.iter().any(|arg| arg == "-video_size"));
    }

    #[test]
    fn a_region_gives_its_corner_and_size_before_the_input() {
        let settings = Settings {
            target: Target::Region {
                x: 100,
                y: 50,
                w: 801,
                h: 600,
            },
            fps: 60,
            cursor: false,
            ..Settings::default()
        };
        let line = joined(&settings);
        assert!(
            line.contains(
                "-framerate 60 -draw_mouse 0 -offset_x 100 -offset_y 50 -video_size 801x600 \
                 -i desktop -vf"
            ),
            "{line}"
        );
    }

    #[test]
    fn a_microphone_is_a_second_input_with_sound_encoded() {
        let settings = Settings {
            mic: Some("Microphone (USB)".to_owned()),
            ..Settings::default()
        };
        let args = ffmpeg_args(&settings, Path::new("out.mp4"));
        let line = args.join(" ");
        assert!(line.contains("-i desktop -f dshow -i audio=Microphone (USB) -vf"), "{line}");
        assert!(line.contains("-pix_fmt yuv420p -c:a aac -b:a 160k -movflags"), "{line}");
        // Without one there is no sound input and no sound encoder.
        let silent = joined(&Settings::default());
        assert!(!silent.contains("dshow"));
        assert!(!silent.contains("aac"));
        // An empty name is no microphone.
        let empty = Settings {
            mic: Some(String::new()),
            ..Settings::default()
        };
        assert_eq!(joined(&empty), silent);
    }

    #[test]
    fn a_frame_rate_the_recorder_does_not_offer_is_thirty() {
        assert_eq!(clean_fps(15), 15);
        assert_eq!(clean_fps(24), 24);
        assert_eq!(clean_fps(30), 30);
        assert_eq!(clean_fps(60), 60);
        assert_eq!(clean_fps(0), 30);
        assert_eq!(clean_fps(144), 30);
        let settings = Settings {
            fps: 144,
            ..Settings::default()
        };
        assert!(joined(&settings).contains("-framerate 30 "));
    }

    #[test]
    fn dates_are_read_from_seconds() {
        let epoch = Stamp::from_unix_utc(0);
        assert_eq!(
            (epoch.year, epoch.month, epoch.day, epoch.hour, epoch.minute, epoch.second),
            (1970, 1, 1, 0, 0, 0)
        );
        // One billion seconds: Sunday 9 September 2001, 01:46:40 UTC.
        let billion = Stamp::from_unix_utc(1_000_000_000);
        assert_eq!(
            (billion.year, billion.month, billion.day),
            (2001, 9, 9)
        );
        assert_eq!((billion.hour, billion.minute, billion.second), (1, 46, 40));
    }

    #[test]
    fn a_file_is_named_by_the_second_and_never_overwrites() {
        let now = Stamp {
            year: 2026,
            month: 11,
            day: 4,
            hour: 9,
            minute: 5,
            second: 7,
        };
        let folder = Path::new("rec");
        let free = output_name_with(folder, &now, &|_path: &Path| false);
        assert_eq!(free, folder.join("Screen_2026-11-04_09-05-07.mp4"));
        let first = folder.join("Screen_2026-11-04_09-05-07.mp4");
        let second = folder.join("Screen_2026-11-04_09-05-07_2.mp4");
        let taken_once = output_name_with(folder, &now, &|path: &Path| path == first);
        assert_eq!(taken_once, second);
        let taken_twice = output_name_with(folder, &now, &|path: &Path| path == first || path == second);
        assert_eq!(taken_twice, folder.join("Screen_2026-11-04_09-05-07_3.mp4"));
    }

    #[test]
    fn microphones_are_read_from_the_newer_layout() {
        let text = "\
[dshow @ 000001] DirectShow video devices (some may be both video and audio devices)
[dshow @ 000001]  \"Integrated Camera\" (video)
[dshow @ 000001]     Alternative name \"@device_pnp_\\\\?\\usb#vid_1\"
[dshow @ 000001] DirectShow audio devices (some may be both video and audio devices)
[dshow @ 000001]  \"Microphone (Realtek Audio)\" (audio)
[dshow @ 000001]     Alternative name \"@device_cm_{33D9A762}\\wave_{1}\"
[dshow @ 000001]  \"Headset (USB)\" (audio)
[dshow @ 000001]     Alternative name \"@device_cm_{33D9A762}\\wave_{2}\"
dummy: Immediate exit requested
";
        assert_eq!(
            dshow_devices_from_ffmpeg_output(text),
            vec!["Microphone (Realtek Audio)".to_owned(), "Headset (USB)".to_owned()]
        );
    }

    #[test]
    fn microphones_are_read_from_the_older_layout() {
        let text = "\
[dshow @ 000001] DirectShow video devices
[dshow @ 000001]  \"Integrated Camera\"
[dshow @ 000001]     Alternative name \"@device_pnp_x\"
[dshow @ 000001] DirectShow audio devices
[dshow @ 000001]  \"Line In (Realtek)\"
[dshow @ 000001]     Alternative name \"@device_cm_y\"
[dshow @ 000001]  \"Line In (Realtek)\"
";
        assert_eq!(
            dshow_devices_from_ffmpeg_output(text),
            vec!["Line In (Realtek)".to_owned()]
        );
    }

    #[test]
    fn a_device_that_is_both_counts_and_one_that_is_not_audio_does_not() {
        let text = "\
[dshow @ 1]  \"Capture Card\" (video, audio)
[dshow @ 1]  \"Webcam\" (video)
[dshow @ 1]  \"Other\" (none)
";
        assert_eq!(
            dshow_devices_from_ffmpeg_output(text),
            vec!["Capture Card".to_owned()]
        );
        assert!(dshow_devices_from_ffmpeg_output("").is_empty());
    }
}
