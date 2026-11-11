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
pub fn start(studio: &mut Studio, url: String) {
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
        move || fetch(&tools, &folder, &url),
        |studio, _, _, result: Result<PathBuf, String>| match result {
            Ok(file) => studio.handle(Msg::Media(MediaMsg::Import(vec![file]))),
            Err(error) => studio.notify(&tf("link.failed", &[&error]), true),
        },
    );
}

/// Runs yt-dlp on `url` and answers with the file it saved in `folder`.
fn fetch(tools: &Path, folder: &Path, url: &str) -> Result<PathBuf, String> {
    let tool = tool(tools)?;
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
            // One file that already holds picture and sound: joining
            // separate ones needs an ffmpeg program, which is not here.
            "-f",
            "b[ext=mp4]/b",
            "-o",
            "%(title).80s-%(id)s.%(ext)s",
            "--print",
            "after_move:filepath",
            "-P",
        ])
        .arg(folder)
        .arg("--")
        .arg(url);
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
    std::fs::rename(&partial, &path).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755));
    }
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
