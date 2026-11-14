// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)

//! Where a finding about a file's sound lands on the timeline.
//!
//! The analyses of `concat-media` speak in seconds of the *file*: a silence
//! from 12.4 s to 13.9 s of the recording. A clip shows only a window of its
//! file, starting at its in-point and playing at its speed, so before a cut
//! the file's seconds are turned into the timeline's. This is pure
//! arithmetic over a [`Clip`] - no audio is read here.

use crate::commands::CutSpan;
use crate::model::Clip;

/// The shortest stretch worth keeping once a finding is clipped to the
/// clip's window, in seconds.
const SLIVER: f64 = 1e-3;

/// `ranges` - `(start, end)` seconds of the clip's file - as spans of
/// timeline, clipped to the window the clip shows.
///
/// The window is the file from the clip's in-point for as much as its length
/// plays at its speed. A range wholly outside it is dropped; one that
/// crosses an edge is cut to it. `None` for a clip this cannot be done for:
/// under a speed curve the clip's speed is not one number, so a file's
/// second has no single place on the timeline.
pub fn to_timeline(clip: &Clip, ranges: &[(f64, f64)]) -> Option<Vec<CutSpan>> {
    if clip.speed_curve.is_some() || !(clip.speed > 0.0) || !clip.speed.is_finite() {
        return None;
    }
    let window_start = clip.source_start;
    let window_end = clip.source_start + clip.duration * clip.speed;
    Some(
        ranges
            .iter()
            .filter_map(|&(from, to)| {
                let (from, to) = (from.max(window_start), to.min(window_end));
                if to - from <= SLIVER {
                    return None;
                }
                Some(CutSpan {
                    start: clip.start + (from - window_start) / clip.speed,
                    end: clip.start + (to - window_start) / clip.speed,
                })
            })
            .collect(),
    )
}

/// `times` - seconds of the clip's file - as seconds of timeline, those that
/// fall inside the window the clip shows, in order. `None` as for
/// [`to_timeline`].
pub fn instants_to_timeline(clip: &Clip, times: &[f64]) -> Option<Vec<f64>> {
    if clip.speed_curve.is_some() || !(clip.speed > 0.0) || !clip.speed.is_finite() {
        return None;
    }
    let window_start = clip.source_start;
    let window_end = clip.source_start + clip.duration * clip.speed;
    Some(
        times
            .iter()
            .filter(|&&time| time >= window_start && time < window_end)
            .map(|&time| clip.start + (time - window_start) / clip.speed)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ClipKind;

    /// A clip at `start` showing `duration` seconds of the file from
    /// `source_start`, at `speed`.
    fn clip(start: f64, duration: f64, source_start: f64, speed: f64) -> Clip {
        let mut clip = Clip::blank(
            "c1".to_owned(),
            "t1".to_owned(),
            ClipKind::Video,
            "talk".to_owned(),
            start,
            duration,
        );
        clip.source_start = source_start;
        clip.speed = speed;
        clip
    }

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn a_range_moves_by_the_clip_start_and_in_point() {
        // The clip sits at 10 s and plays the file from 3 s on.
        let spans = to_timeline(&clip(10.0, 20.0, 3.0, 1.0), &[(5.0, 7.0)]).expect("maps");
        assert_eq!(spans.len(), 1);
        assert!(near(spans[0].start, 12.0) && near(spans[0].end, 14.0), "{spans:?}");
    }

    #[test]
    fn a_faster_clip_takes_less_timeline() {
        // Twice as fast: a file's two seconds are one second of timeline.
        let spans = to_timeline(&clip(0.0, 10.0, 0.0, 2.0), &[(4.0, 6.0)]).expect("maps");
        assert!(near(spans[0].start, 2.0) && near(spans[0].end, 3.0), "{spans:?}");
    }

    #[test]
    fn ranges_outside_the_window_go_and_those_across_an_edge_are_cut_to_it() {
        // The window is the file's 3 s to 13 s.
        let spans = to_timeline(
            &clip(0.0, 10.0, 3.0, 1.0),
            &[(0.0, 2.0), (2.0, 5.0), (12.0, 20.0), (14.0, 15.0)],
        )
        .expect("maps");
        assert_eq!(spans.len(), 2, "{spans:?}");
        assert!(near(spans[0].start, 0.0) && near(spans[0].end, 2.0), "{spans:?}");
        assert!(near(spans[1].start, 9.0) && near(spans[1].end, 10.0), "{spans:?}");
    }

    #[test]
    fn a_speed_curve_has_no_single_mapping() {
        let mut curved = clip(0.0, 10.0, 0.0, 1.0);
        curved.speed_curve = Some(vec![
            crate::model::SpeedPoint {
                at: 0.0,
                speed: 1.0,
                slope: None,
            },
            crate::model::SpeedPoint {
                at: 1.0,
                speed: 2.0,
                slope: None,
            },
        ]);
        assert!(to_timeline(&curved, &[(1.0, 2.0)]).is_none());
        assert!(instants_to_timeline(&curved, &[1.0]).is_none());
    }

    #[test]
    fn instants_inside_the_window_land_on_the_timeline() {
        let found = instants_to_timeline(&clip(5.0, 4.0, 10.0, 1.0), &[9.0, 10.0, 12.5, 14.0, 20.0])
            .expect("maps");
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(near(found[0], 5.0) && near(found[1], 7.5), "{found:?}");
    }
}
