// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)

//! Reading a clip's sound for the edits that cut by it: where it falls
//! silent, where someone speaks, where the beat lands.
//!
//! The analyses themselves are `concat_media::analysis`, over the waveform
//! peaks the timeline already keeps (cached beside the project, so a second
//! look costs nothing). This module finds the clip's file, asks for those
//! peaks and hands the findings back as seconds of *timeline*, which is what
//! every edit speaks in.

use concat_media::analysis;
pub use concat_media::analysis::{BeatSettings, SilenceSettings};
use concat_media::peaks::Peaks;
use concat_project::autocut::{instants_to_timeline, to_timeline};
use concat_project::commands::CutSpan;
use concat_project::model::{Clip, ClipKind, Project};

use crate::media;

/// The clip, and the waveform of the sound it plays.
fn clip_peaks(
    project: &Project,
    clip_id: &str,
    folder: Option<&str>,
) -> Result<(Clip, Peaks), String> {
    let clip = project
        .active()
        .clip(clip_id)
        .ok_or_else(|| format!("There is no clip {clip_id} on this timeline."))?;
    let item = project
        .media_by_id(&clip.media_id)
        .ok_or_else(|| "The clip's file is not in the project.".to_owned())?;
    let has_sound = match clip.kind {
        ClipKind::Audio => true,
        ClipKind::Video => item.has_audio,
        _ => false,
    };
    if !has_sound {
        return Err("That clip has no sound to read.".to_owned());
    }
    let peaks = media::peaks(&item.path, clip.audio_stream, folder)?;
    Ok((clip.clone(), peaks))
}

/// The silences in a clip's sound, as spans of timeline, in order.
///
/// Errs for a clip with no sound, a file that cannot be read, or a clip
/// under a speed curve (its speed is not one number, so a file's second has
/// no single place on the timeline).
pub fn silences(
    project: &Project,
    clip_id: &str,
    folder: Option<&str>,
    settings: &SilenceSettings,
) -> Result<Vec<CutSpan>, String> {
    let (clip, peaks) = clip_peaks(project, clip_id, folder)?;
    let found = analysis::silence_ranges(&peaks, settings);
    to_timeline(&clip, &found).ok_or_else(curve_refusal)
}

/// Where a clip's sound is present - the opposite of its silences - as
/// spans of timeline, gaps under `hold` seconds bridged.
pub fn voice(
    project: &Project,
    clip_id: &str,
    folder: Option<&str>,
    threshold_db: f32,
    hold: f64,
) -> Result<Vec<CutSpan>, String> {
    let (clip, peaks) = clip_peaks(project, clip_id, folder)?;
    let found = analysis::voice_ranges(&peaks, threshold_db, hold);
    to_timeline(&clip, &found).ok_or_else(curve_refusal)
}

/// The beats of a clip's sound, as seconds of timeline in order, and the
/// tempo they suggest in beats per minute when there are enough of them.
pub fn beats(
    project: &Project,
    clip_id: &str,
    folder: Option<&str>,
    settings: &BeatSettings,
) -> Result<(Vec<f64>, Option<f64>), String> {
    let (clip, peaks) = clip_peaks(project, clip_id, folder)?;
    let found = analysis::beat_times(&peaks, settings);
    let bpm = analysis::estimate_bpm(&found);
    let times = instants_to_timeline(&clip, &found).ok_or_else(curve_refusal)?;
    Ok((times, bpm))
}

fn curve_refusal() -> String {
    "That clip's speed changes along its length, so its sound cannot be placed on the timeline \
     by a single rate. Give it one speed first."
        .to_owned()
}
