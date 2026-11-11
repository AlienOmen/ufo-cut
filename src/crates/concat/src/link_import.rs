// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: a video from a link.
//!
//! The Media page takes a web address and hands it to yt-dlp, which runs
//! on this machine as a program of its own: the video is fetched over the
//! person's own connection, lands in the project's `imports` folder, and
//! comes into the bin the way a file picked from the disk does.
//!
//! yt-dlp is not part of UFO Cut and does not ship with it. It is fetched
//! from its own releases the first time a link is used, and kept in the
//! app's data folder. What a person fetches with it is theirs to answer
//! for: the page says to use videos they own or may use.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crate::host::spawn_in_project;
use crate::i18n::{t, tf};
use crate::panes::Msg;
use crate::panes::media_bin::MediaMsg;
use crate::studio::Studio;

/// The program's file, as its releases name it for this system.
#[cfg(windows)]
const TOOL: &str = "yt-dlp.exe";
#[cfg(target_os = "macos")]
const TOOL: &str = "yt-dlp_macos";
#[cfg(all(not(windows), not(target_os = "macos")))]
const TOOL: &str = "yt-dlp_linux";

/// Where yt-dlp's newest release keeps its files.
const RELEASES: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download";

/// Fetches the video at `url` into the open project and imports it.
pub fn start(studio: &mut Studio, url: String, quality: i32) {
    let url = url.trim().to_owned();
    let Some(session) = studio.session.as_ref() else {
        studio.notify(&t("sounds.openProjectFirst"), true);
        return;
    };
    let folder = PathBuf::from(session.path()).join("imports");
    // A web address and nothing else: it is handed to another program, and
    // must not be able to read as one of that program's options.
    let web = url.starts_with("https://") || url.starts_with("http://");
    if !web || url.len() > 2000 || url.chars().any(char::is_whitespace) {
        studio.notify(&t("link.notALink"), true);
        return;
    }
    let tools = studio.host.dirs.data.join("tools");
    studio.notify(&t("link.getting"), false);
    spawn_in_project(
        move || fetch(&tools, &folder, &url, quality),
        |studio, _, _, result: Result<PathBuf, String>| match result {
            Ok(file) => studio.handle(Msg::Media(MediaMsg::Import(vec![file]))),
            Err(error) => studio.notify(&tf("link.failed", &[&error]), true),
        },
    );
}

/// Runs yt-dlp on `url` and answers with the file it saved in `folder`.
fn fetch(tools: &Path, folder: &Path, url: &str, quality: i32) -> Result<PathBuf, String> {
    let tool = tool(tools)?;
    // The tallest picture asked for; none is the best the site has.
    let height = [0, 2160, 1440, 1080, 720, 480, 360]
        .get(quality.max(0) as usize)
        .copied()
        .filter(|height| *height > 0);
    // Sites keep their larger pictures apart from the sound, and an
    // ffmpeg program joins the two. Without one, the largest file that
    // already holds both is taken instead.
    let joiner = joiner(tools).is_ok();
    // The last row of the page's list: the sound alone.
    let sound_only = quality == 7;
    let format = match (joiner, height) {
        _ if sound_only => "ba/b".to_owned(),
        (true, _) => "bv*+ba/b".to_owned(),
        (false, Some(height)) => format!("b[height<={height}]/b"),
        (false, None) => "b".to_owned(),
    };
    std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    let mut command = Command::new(&tool);
    command
        .args([
            "--no-playlist",
            "--no-warnings",
            "--no-progress",
            "--no-simulate",
            "--restrict-filenames",
            "--encoding",
            "utf-8",
            "--max-filesize",
            "2G",
            "-o",
            "%(title).80s-%(id)s.%(ext)s",
            "--print",
            "after_move:filepath",
            "-P",
        ])
        .arg(folder)
        .args(["-f", &format]);
    if joiner && sound_only {
        command.args(["-x", "--audio-format", "mp3", "--ffmpeg-location"]).arg(tools);
    } else if joiner {
        command.args(["--merge-output-format", "mp4", "--ffmpeg-location"]).arg(tools);
        if let Some(height) = height {
            command.args(["-S", &format!("res:{height}")]);
        }
    }
    command.arg("--").arg(url);
    quiet(&mut command);
    let output = command.output().map_err(|error| error.to_string())?;
    let last = |text: &[u8]| {
        String::from_utf8_lossy(text)
            .lines()
            .rev()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(str::to_owned)
    };
    match last(&output.stdout).map(PathBuf::from) {
        Some(file) if output.status.success() && file.is_file() => Ok(file),
        _ => Err(last(&output.stderr)
            .map(|line| line.trim_start_matches("ERROR: ").to_owned())
            .unwrap_or_else(|| "yt-dlp gave no answer".to_owned())),
    }
}

/// The program's path, fetched from its releases when it is not here yet.
fn tool(tools: &Path) -> Result<PathBuf, String> {
    let path = tools.join(TOOL);
    if path.is_file() {
        return Ok(path);
    }
    std::fs::create_dir_all(tools).map_err(|error| error.to_string())?;
    let partial = tools.join(format!("{TOOL}.partial"));
    let never = AtomicBool::new(false);
    concat_host::models::download(
        &format!("{RELEASES}/{TOOL}"),
        &partial,
        20_000_000,
        &never,
        "cancelled",
        &mut |_, _| {},
    )?;
    // A program that is about to be run is checked against the digest
    // its own release publishes; one that does not match is thrown away.
    if let Err(error) = published_digest().and_then(|digest| concat_host::models::verify(&partial, &digest)) {
        let _ = std::fs::remove_file(&partial);
        return Err(error);
    }
    std::fs::rename(&partial, &path).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755));
    }
    Ok(path)
}

/// The SHA-256 yt-dlp's release lists for this system's file.
fn published_digest() -> Result<String, String> {
    use std::io::Read;
    let response = concat_host::net::get(
        &format!("{RELEASES}/SHA2-256SUMS"),
        Duration::from_secs(15),
        Duration::from_secs(30),
        concat_host::net::Trust::Shipped,
        &|request| request,
    )
    .map_err(|failure| failure.cause.describe())?;
    let mut text = String::new();
    response
        .into_reader()
        .take(64 * 1024)
        .read_to_string(&mut text)
        .map_err(|error| error.to_string())?;
    text.lines()
        .find_map(|line| {
            let mut parts = line.split_whitespace();
            let digest = parts.next()?;
            let name = parts.next()?.trim_start_matches('*');
            (name == TOOL).then(|| digest.to_ascii_lowercase())
        })
        .ok_or_else(|| "the release lists no digest for yt-dlp".to_owned())
}

/// The ffmpeg program that joins picture and sound, fetched once. A
/// ready-made Windows build from the ffmpeg-static project.
fn joiner(tools: &Path) -> Result<PathBuf, String> {
    let path = tools.join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" });
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
