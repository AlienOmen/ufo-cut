// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: the A.L.I.E.N. looks. Twelve colour tables of A.L.I.E.N.'s
//! own, carried in the binary and put among the user's looks the first
//! time the app runs, through the same import a `.cube` of theirs takes.
//! After that they are the user's: renamed, removed or kept, and never
//! put back, because a mark beside the looks folder says it was done.

use std::path::Path;

/// The mark's name. A later set of looks takes a new number.
const MARK: &str = "alien-looks-1";

/// Each look's name, which is its file's name and so its card's, and its
/// table.
const LOOKS: &[(&str, &str)] = &[
    ("ALIEN Acid Night", include_str!("../luts/alien-acid-night.cube")),
    ("ALIEN Void Fade", include_str!("../luts/alien-void-fade.cube")),
    ("ALIEN Teal Orange", include_str!("../luts/alien-teal-orange.cube")),
    ("ALIEN Warm Film", include_str!("../luts/alien-warm-film.cube")),
    ("ALIEN Cold Steel", include_str!("../luts/alien-cold-steel.cube")),
    ("ALIEN Mono Punch", include_str!("../luts/alien-mono-punch.cube")),
    ("ALIEN Mono Soft", include_str!("../luts/alien-mono-soft.cube")),
    ("ALIEN Bleach", include_str!("../luts/alien-bleach.cube")),
    ("ALIEN Golden Hour", include_str!("../luts/alien-golden-hour.cube")),
    ("ALIEN Neon Pop", include_str!("../luts/alien-neon-pop.cube")),
    ("ALIEN Clay Soft", include_str!("../luts/alien-clay-soft.cube")),
    ("ALIEN Midnight", include_str!("../luts/alien-midnight.cube")),
];

/// Puts the looks into `dir`, the user's looks folder, once.
pub fn seed(dir: &Path) {
    let mark = dir.parent().unwrap_or(dir).join(MARK);
    if mark.exists() || std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let staging = std::env::temp_dir().join(format!("ufo-cut-looks-{}", std::process::id()));
    if std::fs::create_dir_all(&staging).is_err() {
        return;
    }
    for (name, table) in LOOKS {
        let file = staging.join(format!("{name}.cube"));
        if let Err(error) = std::fs::write(&file, table) {
            log::warn!("{name}: the look was not added: {error}");
            continue;
        }
        if let Err(error) = concat_effects::looks::import(dir, &file) {
            log::warn!("{name}: the look was not added: {error}");
        }
    }
    let _ = std::fs::remove_dir_all(&staging);
    let _ = std::fs::write(mark, "");
}
