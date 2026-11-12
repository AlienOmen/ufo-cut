// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: the A.L.I.E.N. templates. Ready-made edits of A.L.I.E.N.'s
//! own - openers, lower thirds, reels, end cards - carried in the binary
//! and put in the user's template library the first time the app runs.
//! After that they are the user's: removed or kept, and never put back,
//! because a mark beside the library says it was done.

use std::path::Path;

/// The mark's name. A later set of templates takes a new number.
const MARK: &str = "alien-templates-1";

/// Each template's folder, its document and its poster.
const TEMPLATES: &[(&str, &str, &[u8])] = &[
    (
        "ALIEN-01-Title-Landing",
        include_str!("../templates/ALIEN-01-Title-Landing/template.json"),
        include_bytes!("../templates/ALIEN-01-Title-Landing/poster.jpg"),
    ),
    (
        "ALIEN-02-Glitch-Opener",
        include_str!("../templates/ALIEN-02-Glitch-Opener/template.json"),
        include_bytes!("../templates/ALIEN-02-Glitch-Opener/poster.jpg"),
    ),
    (
        "ALIEN-03-Lower-Third-Name",
        include_str!("../templates/ALIEN-03-Lower-Third-Name/template.json"),
        include_bytes!("../templates/ALIEN-03-Lower-Third-Name/poster.jpg"),
    ),
    (
        "ALIEN-04-Lower-Third-Plate",
        include_str!("../templates/ALIEN-04-Lower-Third-Plate/template.json"),
        include_bytes!("../templates/ALIEN-04-Lower-Third-Plate/poster.jpg"),
    ),
    (
        "ALIEN-05-Reel-Hook-3-Clips",
        include_str!("../templates/ALIEN-05-Reel-Hook-3-Clips/template.json"),
        include_bytes!("../templates/ALIEN-05-Reel-Hook-3-Clips/poster.jpg"),
    ),
    (
        "ALIEN-06-Quote",
        include_str!("../templates/ALIEN-06-Quote/template.json"),
        include_bytes!("../templates/ALIEN-06-Quote/poster.jpg"),
    ),
    (
        "ALIEN-07-Before-and-After",
        include_str!("../templates/ALIEN-07-Before-and-After/template.json"),
        include_bytes!("../templates/ALIEN-07-Before-and-After/poster.jpg"),
    ),
    (
        "ALIEN-08-Product-3-Beats",
        include_str!("../templates/ALIEN-08-Product-3-Beats/template.json"),
        include_bytes!("../templates/ALIEN-08-Product-3-Beats/poster.jpg"),
    ),
    (
        "ALIEN-09-End-Card",
        include_str!("../templates/ALIEN-09-End-Card/template.json"),
        include_bytes!("../templates/ALIEN-09-End-Card/poster.jpg"),
    ),
    (
        "ALIEN-10-End-Card-Vertical",
        include_str!("../templates/ALIEN-10-End-Card-Vertical/template.json"),
        include_bytes!("../templates/ALIEN-10-End-Card-Vertical/poster.jpg"),
    ),
    (
        "ALIEN-11-Countdown",
        include_str!("../templates/ALIEN-11-Countdown/template.json"),
        include_bytes!("../templates/ALIEN-11-Countdown/poster.jpg"),
    ),
    (
        "ALIEN-12-Picture-in-Picture",
        include_str!("../templates/ALIEN-12-Picture-in-Picture/template.json"),
        include_bytes!("../templates/ALIEN-12-Picture-in-Picture/poster.jpg"),
    ),
    (
        "ALIEN-13-Photo-Slideshow",
        include_str!("../templates/ALIEN-13-Photo-Slideshow/template.json"),
        include_bytes!("../templates/ALIEN-13-Photo-Slideshow/poster.jpg"),
    ),
    (
        "ALIEN-14-Podcast-Clip",
        include_str!("../templates/ALIEN-14-Podcast-Clip/template.json"),
        include_bytes!("../templates/ALIEN-14-Podcast-Clip/poster.jpg"),
    ),
    (
        "ALIEN-15-YouTube-Opener",
        include_str!("../templates/ALIEN-15-YouTube-Opener/template.json"),
        include_bytes!("../templates/ALIEN-15-YouTube-Opener/poster.jpg"),
    ),
    (
        "ALIEN-16-Offer",
        include_str!("../templates/ALIEN-16-Offer/template.json"),
        include_bytes!("../templates/ALIEN-16-Offer/poster.jpg"),
    ),
    (
        "ALIEN-17-Chapter-Card",
        include_str!("../templates/ALIEN-17-Chapter-Card/template.json"),
        include_bytes!("../templates/ALIEN-17-Chapter-Card/poster.jpg"),
    ),
    (
        "ALIEN-18-Split-Screen",
        include_str!("../templates/ALIEN-18-Split-Screen/template.json"),
        include_bytes!("../templates/ALIEN-18-Split-Screen/poster.jpg"),
    ),
];

/// Puts the templates into the library under `config`, once.
pub fn seed(config: &Path) {
    let library = concat_host::templates::templates_dir(config);
    let mark = config.join(MARK);
    if mark.exists() || std::fs::create_dir_all(&library).is_err() {
        return;
    }
    for (folder, document, poster) in TEMPLATES {
        let root = library.join(folder);
        if root.exists() {
            continue;
        }
        let written = std::fs::create_dir_all(&root)
            .and_then(|()| std::fs::write(root.join("template.json"), document))
            .and_then(|()| std::fs::write(root.join("poster.jpg"), poster));
        if let Err(error) = written {
            log::warn!("{folder}: the template was not added: {error}");
        }
    }
    let _ = std::fs::write(mark, "");
}
