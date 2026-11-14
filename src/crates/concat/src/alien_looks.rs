// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: the A.L.I.E.N. looks. Twelve colour tables of A.L.I.E.N.'s
//! own, carried in the binary and put among the user's looks the first
//! time the app runs, through the same import a `.cube` of theirs takes.
//! After that they are the user's: renamed, removed or kept, and never
//! put back, because a mark beside the looks folder says it was done.

use std::path::Path;

/// The first set's mark. A later set of looks takes a new number.
const MARK: &str = "alien-looks-1";
/// The second set's mark: the twelve 33-point looks added in UFO Cut 2.0.
/// Installs that already ran the first set get these on their next start,
/// and only once.
const MARK_2: &str = "alien-looks-2";

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

/// The second set: procedural 33-point tables.
const LOOKS_2: &[(&str, &str)] = &[
    ("ALIEN Cine Teal Orange", include_str!("../luts/alien-cine-teal-orange.cube")),
    ("ALIEN Gulf Sunset", include_str!("../luts/alien-gulf-sunset.cube")),
    ("ALIEN Warm Skin", include_str!("../luts/alien-warm-skin.cube")),
    ("ALIEN Documentary", include_str!("../luts/alien-documentary.cube")),
    ("ALIEN Soft Film", include_str!("../luts/alien-soft-film.cube")),
    ("ALIEN Bleach Bypass", include_str!("../luts/alien-bleach-bypass.cube")),
    ("ALIEN Cool Night", include_str!("../luts/alien-cool-night.cube")),
    ("ALIEN Vintage Fade", include_str!("../luts/alien-vintage-fade.cube")),
    ("ALIEN BW Contrast", include_str!("../luts/alien-bw-contrast.cube")),
    ("ALIEN Ramadan Lantern", include_str!("../luts/alien-ramadan-lantern.cube")),
    ("ALIEN Lunar Roast", include_str!("../luts/alien-lunar-roast.cube")),
    ("ALIEN Neutral Clean", include_str!("../luts/alien-neutral-clean.cube")),
];

/// Puts the looks into `dir`, the user's looks folder, once per set.
pub fn seed(dir: &Path) {
    seed_set(dir, MARK, LOOKS);
    seed_set(dir, MARK_2, LOOKS_2);
}

fn seed_set(dir: &Path, mark: &str, looks: &[(&str, &str)]) {
    let mark = dir.parent().unwrap_or(dir).join(mark);
    if mark.exists() || std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let staging = std::env::temp_dir().join(format!("ufo-cut-looks-{}-{}", std::process::id(), looks.len()));
    if std::fs::create_dir_all(&staging).is_err() {
        return;
    }
    for (name, table) in looks {
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
