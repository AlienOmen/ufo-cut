// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)

//! The lesson preflight: a timeline held against what a course lesson asks
//! of its video, and a plain-language list of what to fix.
//!
//! A lesson of the Alien Designer course (E.19, E.20, C.07, ...) says what
//! the delivered video must be: a frame, a rate, a length, captions of at
//! most two lines, sound that is there. [`LessonSpec`] states those rules as
//! data, [`check`] reads the active timeline against them and gives back
//! [`Finding`]s, each with a code a program can test, a severity, what is
//! wrong and the step that fixes it. Nothing is changed and nothing on disk
//! is read: it is arithmetic over the model, so it runs the same in the
//! window, in the API and in a test.
//!
//! Where the lessons leave a number open, the default is a named constant
//! here with a `TODO(owner)` beside it, so the choice is easy to find and to
//! change in one place.

use serde::Serialize;

use crate::model::{Clip, ClipKind, Project, Timeline};

/// How far the timeline's length may be from the lesson's, in seconds. The
/// lesson says "exactly 15 seconds"; one frame at 30 fps is 0.033 s, so a
/// timeline a frame short or long is off and anything closer is on.
/// TODO(owner): the lesson gives no tolerance; this is the strictest that
/// still forgives floating-point noise.
pub const DURATION_TOLERANCE: f64 = 0.02;

/// How far from a marker a cut may fall before it is called off the marker,
/// in seconds. TODO(owner): E.19 says each moment "sits in place on the
/// Markers" and "cut on the action", which allows a little give.
pub const CUT_TOLERANCE: f64 = 0.25;

/// Seconds a caption needs per word: E.19 says "roughly a second and a half
/// for every three words".
pub const SECONDS_PER_WORD: f64 = 0.5;

/// A hole in the picture shorter than this is not reported, in seconds:
/// just over one frame at 30 fps.
pub const GAP_TOLERANCE: f64 = 0.04;

/// The fewest picture holes reported at once, so one bad edit is not a wall
/// of findings.
const MAX_GAPS: usize = 5;

/// The top of the frame kept clear of captions, as a fraction of its
/// height. These are the "Alien Safe Zone" of the `alien.safe-zones` effect
/// (`docs/SAFE_ZONES.md`): what is safe on Instagram, Facebook and TikTok at
/// once, measured from the owner's "Free Social Media Safe Zone Templates"
/// (270 px of 1920 at the top, 675 px at the bottom, 122 px at the left).
/// TODO(owner): the lessons (E.19, E.20, C.03) say "inside the Safe Zone" and
/// never give numbers; this is strict by design (the bottom third is where
/// Reels puts its caption), and the lesson should quote whatever is chosen.
pub const SAFE_TOP: f64 = 0.1406;

/// The bottom of the frame kept clear, as a fraction of its height. See
/// [`SAFE_TOP`].
pub const SAFE_BOTTOM: f64 = 0.3516;

/// The left margin kept clear, as a fraction of the width. See [`SAFE_TOP`].
pub const SAFE_SIDE: f64 = 0.113;

/// The right margin kept clear, where TikTok's buttons sit below the top
/// strip (everything right of x = 837 of 1080), as a fraction of the width.
/// See [`SAFE_TOP`].
pub const SAFE_RIGHT: f64 = 0.225;

/// The caption font E.20 names. TODO(owner): E.20 says Cairo; the recording
/// standard in 10_Production uses Readex Pro / Changa. Cairo is also the
/// LUNAR ROAST brand font, so it is the default here.
pub const CAPTION_FONT: &str = "Cairo";

/// Where speech should peak on the meter, in dB, as E.20 says. TODO(owner):
/// the lessons speak in dB on the meter and never in LUFS, while Ahmed's own
/// recording standard (10_Production) is -16 LUFS. Two standards; nothing
/// measures either yet, so this is only quoted to the student.
pub const SPEECH_LEVEL_DB: f64 = -6.0;

/// How far below the speech the music sits while someone talks, in dB, as
/// E.20 says: the nearest and the furthest. See [`SPEECH_LEVEL_DB`].
pub const MUSIC_BELOW_SPEECH_DB: (f64, f64) = (12.0, 18.0);

/// The loudness of Ahmed's recording standard, in LUFS. TODO(owner): not
/// used by any check; kept beside [`SPEECH_LEVEL_DB`] so the two standards
/// are reconciled in one place.
pub const LOUDNESS_TARGET_LUFS: f64 = -16.0;

/// How serious a finding is.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// The video breaks a rule of the lesson; fix it before handing in.
    Error,
    /// Worth a look; the lesson's rule may still be met.
    Warning,
}

impl Severity {
    /// The word for it: "error" or "warning".
    pub fn label(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }
    }
}

/// One thing the preflight found.
#[derive(Clone, PartialEq, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    /// A stable code a program can test: "FRAME_SIZE", "DURATION", ...
    pub code: &'static str,
    /// How serious it is.
    pub severity: Severity,
    /// What is wrong, in plain words.
    pub message: String,
    /// The step that fixes it, in plain words.
    pub fix: String,
    /// Where on the timeline it is, in seconds, when it is somewhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
}

impl Finding {
    fn new(
        code: &'static str,
        severity: Severity,
        message: String,
        fix: String,
        at: Option<f64>,
    ) -> Finding {
        Finding {
            code,
            severity,
            message,
            fix,
            at,
        }
    }
}

/// What one lesson asks of its video.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LessonSpec {
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
    /// Frame rate numerator (30 for 30 fps).
    pub rate_num: i64,
    /// Frame rate denominator.
    pub rate_den: i64,
    /// The length, in seconds.
    pub duration: f64,
    /// How far the length may be from `duration`, in seconds.
    pub tolerance: f64,
    /// The most lines a caption or title may have; `None` for no rule.
    pub max_caption_lines: Option<usize>,
    /// Whether captions are required, and how seriously; `None` when the
    /// lesson does not ask for them.
    pub captions: Option<Severity>,
    /// Whether sound is required, and how seriously; `None` when the lesson
    /// does not ask for it.
    pub audio: Option<Severity>,
    /// Seconds at which one shot should end and the next begin (the
    /// lesson's markers); empty for no rule.
    pub cut_times: &'static [f64],
    /// Seconds at the end that should be a still end card; `None` for no
    /// rule.
    pub still_end: Option<f64>,
}

/// E.19's four markers: Hook 0-2, Context 2-6, Hero 6-10, Offer 10-13, End
/// Card 13-15.
const REELS_MARKERS: &[f64] = &[2.0, 6.0, 10.0, 13.0];

/// E.19: the 15-second 9:16 motion, cut from five shots on the five
/// moments. Sound and captions are finished in E.20, so neither is asked
/// for here.
pub const SPEC_E19: LessonSpec = LessonSpec {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    duration: 15.0,
    tolerance: DURATION_TOLERANCE,
    max_caption_lines: Some(2),
    captions: None,
    audio: None,
    cut_times: REELS_MARKERS,
    still_end: Some(2.0),
};

/// E.20: the same picture with its sound (VO, bed and one effect) and its
/// captions, two lines at most.
pub const SPEC_E20: LessonSpec = LessonSpec {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    duration: 15.0,
    tolerance: DURATION_TOLERANCE,
    max_caption_lines: Some(2),
    captions: Some(Severity::Error),
    audio: Some(Severity::Error),
    cut_times: REELS_MARKERS,
    still_end: Some(2.0),
};

/// I.09's free route: an animatic of five stills and a voice-over, fifteen
/// seconds. TODO(owner): the lesson gives the length ("a Shot List for 15
/// seconds") but not the frame; it is taken as the 9:16 of the motion the
/// shots are for. The voice-over is advised, not required.
pub const SPEC_I09: LessonSpec = LessonSpec {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    duration: 15.0,
    tolerance: DURATION_TOLERANCE,
    max_caption_lines: None,
    captions: None,
    audio: Some(Severity::Warning),
    cut_times: &[],
    still_end: None,
};

/// C.07: the capstone's motion, "the Final MP4, 15 seconds" with "audible
/// sound and Captions present". It is the student's own re-cut (a 4-shot
/// list in C.08's plan), so no markers are held to.
pub const SPEC_C07: LessonSpec = LessonSpec {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    duration: 15.0,
    tolerance: DURATION_TOLERANCE,
    max_caption_lines: Some(2),
    captions: Some(Severity::Error),
    audio: Some(Severity::Error),
    cut_times: &[],
    still_end: None,
};

/// A frame rate as a person reads it: "30", or "29.97".
pub fn fps_text(num: i64, den: i64) -> String {
    if den == 1 {
        num.to_string()
    } else if den <= 0 {
        "?".to_owned()
    } else {
        format!("{:.2}", num as f64 / den as f64)
    }
}

/// True when none of `findings` is an error: the video meets the lesson's
/// rules (warnings may remain).
pub fn passed(findings: &[Finding]) -> bool {
    findings
        .iter()
        .all(|finding| finding.severity != Severity::Error)
}

/// The active timeline of `project`, held against `spec`. Errors come
/// first, then warnings, each group in the order the rules are listed.
pub fn check(project: &Project, spec: &LessonSpec) -> Vec<Finding> {
    let timeline = project.active();
    let mut found: Vec<Finding> = Vec::new();
    frame(timeline, spec, &mut found);
    if timeline.clips.is_empty() {
        found.push(Finding::new(
            "NO_CLIPS",
            Severity::Error,
            "The timeline is empty.".to_owned(),
            "Add the shots to the timeline first, then run the check again.".to_owned(),
            None,
        ));
        found.sort_by_key(|finding| finding.severity != Severity::Error);
        return found;
    }
    length(timeline, spec, &mut found);
    sound(project, timeline, spec, &mut found);
    texts(timeline, spec, &mut found);
    cuts(timeline, spec, &mut found);
    end_card(timeline, spec, &mut found);
    gaps(timeline, &mut found);
    found.sort_by_key(|finding| finding.severity != Severity::Error);
    found
}

/// Frame size and rate.
fn frame(timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    let video = timeline.video;
    if video.width != spec.width || video.height != spec.height {
        found.push(Finding::new(
            "FRAME_SIZE",
            Severity::Error,
            format!(
                "The frame is {} x {}; the lesson asks for {} x {}.",
                video.width, video.height, spec.width, spec.height
            ),
            format!(
                "Set the project frame to {} x {} (project settings, or the project_set_video \
                 tool), then check that every clip still fills the frame.",
                spec.width, spec.height
            ),
            None,
        ));
    }
    let have = i128::from(video.rate_num) * i128::from(spec.rate_den);
    let want = i128::from(spec.rate_num) * i128::from(video.rate_den);
    if have != want {
        found.push(Finding::new(
            "FRAME_RATE",
            Severity::Error,
            format!(
                "The frame rate is {} fps; the lesson asks for {} fps.",
                fps_text(video.rate_num, video.rate_den),
                fps_text(spec.rate_num, spec.rate_den)
            ),
            format!(
                "Set the project frame rate to {} fps (project settings, or the \
                 project_set_video tool) before you cut.",
                fps_text(spec.rate_num, spec.rate_den)
            ),
            None,
        ));
    }
}

/// The length of the edit.
fn length(timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    let actual = timeline.duration();
    let off = actual - spec.duration;
    if off.abs() <= spec.tolerance {
        return;
    }
    if off > 0.0 {
        found.push(Finding::new(
            "DURATION",
            Severity::Error,
            format!(
                "The edit runs {actual:.2} s, {off:.2} s longer than the {} s the lesson asks for.",
                spec.duration
            ),
            format!(
                "Trim {off:.2} s off the end: shorten the last shot, and pull back anything on \
                 the other lanes (sound, captions) that runs past {} s.",
                spec.duration
            ),
            Some(spec.duration),
        ));
    } else {
        let short = -off;
        found.push(Finding::new(
            "DURATION",
            Severity::Error,
            format!(
                "The edit runs {actual:.2} s, {short:.2} s shorter than the {} s the lesson asks \
                 for.",
                spec.duration
            ),
            format!(
                "Add {short:.2} s: lengthen the last shot or the end card until the timeline \
                 ends at {} s.",
                spec.duration
            ),
            Some(actual),
        ));
    }
}

/// Whether sound is on the timeline.
fn sound(project: &Project, timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    let Some(severity) = spec.audio else {
        return;
    };
    let heard = timeline.clips.iter().any(|clip| {
        let lane_muted = timeline
            .track(&clip.track_id)
            .is_some_and(|track| track.muted);
        if lane_muted || clip.volume <= 0.0 {
            return false;
        }
        match clip.kind {
            ClipKind::Audio => true,
            ClipKind::Video => {
                clip.muted != Some(true)
                    && project
                        .media_by_id(&clip.media_id)
                        .is_some_and(|media| media.has_audio)
            }
            _ => false,
        }
    });
    if !heard {
        found.push(Finding::new(
            "NO_SOUND",
            severity,
            "There is no sound on the timeline: no audio clip, and no video clip with its sound \
             on."
                .to_owned(),
            "Add the voice-over, the sound bed and the effect on audio lanes, and make sure the \
             lanes are not muted."
                .to_owned(),
            None,
        ));
    }
}

/// Captions and titles: that there are some, their lines, how long each
/// stays and whether it sits clear of the platform's edges.
fn texts(timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    let mut titles: Vec<&Clip> = timeline
        .clips
        .iter()
        .filter(|clip| clip.kind == ClipKind::Text && clip.text.is_some())
        .map(|clip| &**clip)
        .collect();
    titles.sort_by(|a, b| a.start.total_cmp(&b.start));
    if titles.is_empty() {
        if let Some(severity) = spec.captions {
            found.push(Finding::new(
                "NO_CAPTIONS",
                severity,
                "There are no captions on the timeline.".to_owned(),
                format!(
                    "Add the captions (import the .srt or type them), in {CAPTION_FONT}, \
                     Off-White on a Void Black bar, then check every word."
                ),
                None,
            ));
        }
        return;
    }
    for clip in titles {
        let Some(style) = clip.text.as_ref() else {
            continue;
        };
        let lines = style
            .content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();
        if let Some(most) = spec.max_caption_lines
            && lines > most
        {
            found.push(Finding::new(
                "CAPTION_LINES",
                Severity::Error,
                format!(
                    "The text at {:.1} s has {lines} lines; the lesson allows {most} at most.",
                    clip.start
                ),
                format!(
                    "Rewrite or split the text at {:.1} s so no screen shows more than {most} \
                     lines.",
                    clip.start
                ),
                Some(clip.start),
            ));
        }
        let words = style.content.split_whitespace().count();
        let wanted = words as f64 * SECONDS_PER_WORD;
        if clip.duration + 1e-6 < wanted {
            found.push(Finding::new(
                "CAPTION_TOO_FAST",
                Severity::Warning,
                format!(
                    "The text at {:.1} s ({words} words) stays {:.1} s; it needs about {wanted:.1} \
                     s to be read.",
                    clip.start, clip.duration
                ),
                format!(
                    "Keep it on screen about {wanted:.1} s (a second and a half for every three \
                     words), or shorten the words."
                ),
                Some(clip.start),
            ));
        }
        if spec.max_caption_lines.is_some() {
            let height = lines.max(1) as f64 * style.font_size * style.line_height;
            let centre = 0.5 + clip.offset_y;
            let top = centre - height / 2.0;
            let bottom = centre + height / 2.0;
            if top < SAFE_TOP - 1e-9 || bottom > 1.0 - SAFE_BOTTOM + 1e-9 {
                found.push(Finding::new(
                    "CAPTION_SAFE_ZONE",
                    Severity::Warning,
                    format!(
                        "The text at {:.1} s looks to reach the top or bottom edge, where the \
                         platform's own buttons and caption sit.",
                        clip.start
                    ),
                    format!(
                        "Move it so it stays between {:.0}% and {:.0}% of the frame's height, \
                         then check on a phone.",
                        SAFE_TOP * 100.0,
                        (1.0 - SAFE_BOTTOM) * 100.0
                    ),
                    Some(clip.start),
                ));
            }
        }
    }
}

/// Whether each marker has a cut on it.
fn cuts(timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    if spec.cut_times.is_empty() {
        return;
    }
    let mut edges: Vec<f64> = Vec::new();
    for clip in timeline.clips.iter().filter(|clip| clip.kind.is_visual()) {
        edges.push(clip.start);
        edges.push(clip.start + clip.duration);
    }
    if edges.is_empty() {
        return;
    }
    for &at in spec.cut_times.iter() {
        let nearest = edges.iter().copied().fold(f64::INFINITY, |best, edge| {
            if (edge - at).abs() < (best - at).abs() {
                edge
            } else {
                best
            }
        });
        if (nearest - at).abs() > CUT_TOLERANCE {
            found.push(Finding::new(
                "CUT_OFF_MARKER",
                Severity::Warning,
                format!(
                    "No shot starts or ends near {at} s; the nearest cut is at {nearest:.2} s."
                ),
                format!(
                    "Move the cut to {at} s so each moment sits on its marker (or move the \
                     marker, if you meant it)."
                ),
                Some(at),
            ));
        }
    }
}

/// Whether the last seconds are a still end card.
fn end_card(timeline: &Timeline, spec: &LessonSpec, found: &mut Vec<Finding>) {
    let Some(still) = spec.still_end else {
        return;
    };
    let end = timeline.duration();
    let from = end - still;
    if from <= 0.0 {
        return;
    }
    let moving = timeline.clips.iter().find(|clip| {
        clip.kind == ClipKind::Video
            && clip.start < end - 1e-3
            && clip.start + clip.duration > from + 1e-3
            && timeline
                .track(&clip.track_id)
                .is_none_or(|track| track.visible)
    });
    if let Some(clip) = moving {
        found.push(Finding::new(
            "END_CARD_MOVES",
            Severity::Warning,
            format!(
                "The last {still} s should be a still end card, but \"{}\" is video that plays \
                 there.",
                clip.name
            ),
            format!(
                "Put the end card (logo and button) on a still image for the last {still} s and \
                 end the video before it."
            ),
            Some(from),
        ));
    }
}

/// Stretches with nothing on the picture lanes, which export as black.
fn gaps(timeline: &Timeline, found: &mut Vec<Finding>) {
    let end = timeline.duration();
    let mut spans: Vec<(f64, f64)> = timeline
        .clips
        .iter()
        .filter(|clip| {
            clip.kind.is_visual()
                && timeline
                    .track(&clip.track_id)
                    .is_none_or(|track| track.visible)
        })
        .map(|clip| (clip.start, clip.start + clip.duration))
        .collect();
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut cursor = 0.0_f64;
    let mut holes: Vec<(f64, f64)> = Vec::new();
    for (from, to) in spans {
        if from > cursor + GAP_TOLERANCE {
            holes.push((cursor, from));
        }
        if to > cursor {
            cursor = to;
        }
    }
    if end > cursor + GAP_TOLERANCE {
        holes.push((cursor, end));
    }
    for (from, to) in holes.into_iter().take(MAX_GAPS) {
        found.push(Finding::new(
            "PICTURE_GAP",
            Severity::Warning,
            format!("Nothing is on the picture lanes from {from:.2} s to {to:.2} s: it exports as black."),
            format!(
                "Fill {from:.2} s to {to:.2} s with a shot, or close the gap by moving the \
                 clips after it back."
            ),
            Some(from),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Clip, TextStyle, VideoSettings};
    use std::sync::Arc;

    /// A project with the frame and rate the E.19 lesson asks for.
    fn reels() -> Project {
        Project::with_video(VideoSettings {
            width: 1080,
            height: 1920,
            rate_num: 30,
            rate_den: 1,
            ..VideoSettings::default()
        })
    }

    fn put(project: &mut Project, clip: Clip) {
        project.active_mut().clips.push(Arc::new(clip));
    }

    fn shot(id: &str, start: f64, duration: f64, kind: ClipKind) -> Clip {
        Clip::blank(id, "T1", kind, id, start, duration)
    }

    /// The five moments of E.19: four shots and a still end card.
    fn five_moments(project: &mut Project) {
        put(project, shot("c1", 0.0, 2.0, ClipKind::Video));
        put(project, shot("c2", 2.0, 4.0, ClipKind::Video));
        put(project, shot("c3", 6.0, 4.0, ClipKind::Video));
        put(project, shot("c4", 10.0, 3.0, ClipKind::Video));
        put(project, shot("c5", 13.0, 2.0, ClipKind::Image));
    }

    fn caption(id: &str, start: f64, duration: f64, content: &str) -> Clip {
        let mut clip = Clip::blank(id, "T2", ClipKind::Text, id, start, duration);
        clip.text = Some(TextStyle {
            content: content.to_owned(),
            font_size: 0.05,
            ..TextStyle::default()
        });
        clip.offset_y = 0.08;
        clip
    }

    fn codes(findings: &[Finding]) -> Vec<&'static str> {
        findings.iter().map(|finding| finding.code).collect()
    }

    #[test]
    fn the_five_moments_on_their_markers_pass_the_picture_rules() {
        let mut project = reels();
        five_moments(&mut project);
        let found = check(&project, &SPEC_E19);
        assert!(found.is_empty(), "{found:?}");
        assert!(passed(&found));
    }

    #[test]
    fn an_empty_timeline_says_so_and_the_wrong_frame_too() {
        let project = Project::new();
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["FRAME_SIZE", "NO_CLIPS"]);
        assert!(!passed(&found));
    }

    #[test]
    fn a_wrong_frame_size_or_rate_is_an_error_with_a_fix() {
        let mut project = Project::with_video(VideoSettings {
            width: 1920,
            height: 1080,
            rate_num: 24,
            rate_den: 1,
            ..VideoSettings::default()
        });
        five_moments(&mut project);
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["FRAME_SIZE", "FRAME_RATE"]);
        assert!(found[0].fix.contains("1080 x 1920"), "{}", found[0].fix);
        assert!(found[1].message.contains("24"), "{}", found[1].message);
        // 30000/1001 is 29.97, not 30.
        let mut ntsc = Project::with_video(VideoSettings {
            width: 1080,
            height: 1920,
            rate_num: 30000,
            rate_den: 1001,
            ..VideoSettings::default()
        });
        five_moments(&mut ntsc);
        assert_eq!(codes(&check(&ntsc, &SPEC_E19)), vec!["FRAME_RATE"]);
    }

    #[test]
    fn a_length_off_by_more_than_the_tolerance_is_an_error() {
        let mut long = reels();
        five_moments(&mut long);
        put(&mut long, shot("tail", 15.0, 1.0, ClipKind::Image));
        let found = check(&long, &SPEC_E19);
        assert!(codes(&found).contains(&"DURATION"), "{found:?}");
        let duration = found
            .iter()
            .find(|finding| finding.code == "DURATION")
            .expect("a duration finding");
        assert!(duration.message.contains("longer"), "{}", duration.message);

        let mut short = reels();
        put(&mut short, shot("c1", 0.0, 14.0, ClipKind::Video));
        let found = check(&short, &SPEC_E19);
        let duration = found
            .iter()
            .find(|finding| finding.code == "DURATION")
            .expect("a duration finding");
        assert!(duration.message.contains("shorter"), "{}", duration.message);
        assert!(duration.fix.contains("1.00"), "{}", duration.fix);
    }

    #[test]
    fn a_frame_of_give_either_way_is_still_exact() {
        let mut project = reels();
        put(&mut project, shot("c1", 0.0, 2.0, ClipKind::Video));
        put(&mut project, shot("c2", 2.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c3", 6.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c4", 10.0, 3.0, ClipKind::Video));
        put(&mut project, shot("c5", 13.0, 2.0 + 0.01, ClipKind::Image));
        let found = check(&project, &SPEC_E19);
        assert!(!codes(&found).contains(&"DURATION"), "{found:?}");
    }

    #[test]
    fn captions_of_three_lines_break_the_rule() {
        let mut project = reels();
        five_moments(&mut project);
        put(&mut project, caption("t1", 1.0, 3.0, "one\ntwo\nthree"));
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["CAPTION_LINES"]);
        assert_eq!(found[0].severity, Severity::Error);
        assert_eq!(found[0].at, Some(1.0));
    }

    #[test]
    fn two_lines_a_readable_time_and_a_clear_position_are_fine() {
        let mut project = reels();
        five_moments(&mut project);
        put(&mut project, caption("t1", 1.0, 3.0, "Premium capsules,\nbuilt at night"));
        let found = check(&project, &SPEC_E19);
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_caption_gone_before_it_can_be_read_is_a_warning() {
        let mut project = reels();
        five_moments(&mut project);
        // Six words need about three seconds.
        put(&mut project, caption("t1", 1.0, 1.0, "one two three four five six"));
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["CAPTION_TOO_FAST"]);
        assert_eq!(found[0].severity, Severity::Warning);
    }

    #[test]
    fn a_caption_low_in_the_frame_is_warned_about() {
        let mut project = reels();
        five_moments(&mut project);
        let mut low = caption("t1", 1.0, 3.0, "Book your first box");
        low.offset_y = 0.44;
        put(&mut project, low);
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["CAPTION_SAFE_ZONE"]);
    }

    #[test]
    fn e20_asks_for_sound_and_captions_and_says_which_is_missing() {
        let mut project = reels();
        five_moments(&mut project);
        let found = check(&project, &SPEC_E20);
        assert_eq!(codes(&found), vec!["NO_SOUND", "NO_CAPTIONS"]);
        assert!(!passed(&found));

        put(&mut project, shot("a1", 0.0, 15.0, ClipKind::Audio));
        put(&mut project, caption("t1", 1.0, 3.0, "Premium capsules"));
        let found = check(&project, &SPEC_E20);
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_muted_lane_or_a_silent_clip_is_not_sound() {
        let mut project = reels();
        five_moments(&mut project);
        put(&mut project, caption("t1", 1.0, 3.0, "Premium capsules"));
        let mut quiet = shot("a1", 0.0, 15.0, ClipKind::Audio);
        quiet.volume = 0.0;
        put(&mut project, quiet);
        assert!(codes(&check(&project, &SPEC_E20)).contains(&"NO_SOUND"));

        let mut lane = reels();
        five_moments(&mut lane);
        put(&mut lane, caption("t1", 1.0, 3.0, "Premium capsules"));
        let mut on_muted = shot("a1", 0.0, 15.0, ClipKind::Audio);
        on_muted.track_id = "T3".to_owned();
        put(&mut lane, on_muted);
        lane.active_mut().tracks[2].muted = true;
        assert!(codes(&check(&lane, &SPEC_E20)).contains(&"NO_SOUND"));
    }

    #[test]
    fn a_missing_cut_on_a_marker_is_a_warning() {
        let mut project = reels();
        // Three shots only: nothing ends at 2 or 6, and the cut at 13 is
        // still there.
        put(&mut project, shot("c1", 0.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c2", 4.0, 9.0, ClipKind::Video));
        put(&mut project, shot("c3", 13.0, 2.0, ClipKind::Image));
        let found = check(&project, &SPEC_E19);
        let marked: Vec<f64> = found
            .iter()
            .filter(|finding| finding.code == "CUT_OFF_MARKER")
            .filter_map(|finding| finding.at)
            .collect();
        assert_eq!(marked, vec![2.0, 6.0, 10.0]);
    }

    #[test]
    fn a_video_running_through_the_end_card_is_a_warning() {
        let mut project = reels();
        put(&mut project, shot("c1", 0.0, 2.0, ClipKind::Video));
        put(&mut project, shot("c2", 2.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c3", 6.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c4", 10.0, 3.0, ClipKind::Video));
        put(&mut project, shot("c5", 13.0, 2.0, ClipKind::Video));
        let found = check(&project, &SPEC_E19);
        assert_eq!(codes(&found), vec!["END_CARD_MOVES"]);
        assert_eq!(found[0].at, Some(13.0));
    }

    #[test]
    fn a_hole_in_the_picture_is_found_and_the_report_is_capped() {
        let mut project = reels();
        put(&mut project, shot("c1", 0.0, 2.0, ClipKind::Video));
        // 2 s to 3 s is empty.
        put(&mut project, shot("c2", 3.0, 3.0, ClipKind::Video));
        put(&mut project, shot("c3", 6.0, 4.0, ClipKind::Video));
        put(&mut project, shot("c4", 10.0, 3.0, ClipKind::Video));
        put(&mut project, shot("c5", 13.0, 2.0, ClipKind::Image));
        let found = check(&project, &SPEC_E19);
        let holes: Vec<&Finding> = found
            .iter()
            .filter(|finding| finding.code == "PICTURE_GAP")
            .collect();
        assert_eq!(holes.len(), 1, "{found:?}");
        assert_eq!(holes[0].at, Some(2.0));

        // Eight one-second pieces with a hole between each leave seven
        // holes; five are reported.
        let mut scattered = reels();
        for index in 0..8 {
            put(
                &mut scattered,
                shot(&format!("s{index}"), index as f64 * 2.0, 1.0, ClipKind::Video),
            );
        }
        let found = check(&scattered, &SPEC_E19);
        let count = found
            .iter()
            .filter(|finding| finding.code == "PICTURE_GAP")
            .count();
        assert_eq!(count, MAX_GAPS);
    }

    #[test]
    fn errors_are_listed_before_warnings() {
        let mut project = Project::new();
        put(&mut project, shot("c1", 0.0, 4.0, ClipKind::Video));
        let found = check(&project, &SPEC_E19);
        let first_warning = found
            .iter()
            .position(|finding| finding.severity == Severity::Warning);
        let last_error = found
            .iter()
            .rposition(|finding| finding.severity == Severity::Error);
        if let (Some(warning), Some(error)) = (first_warning, last_error) {
            assert!(error < warning, "{found:?}");
        }
        assert!(!found.is_empty());
    }

    #[test]
    fn a_finding_writes_as_camel_case_json() {
        let finding = Finding::new(
            "DURATION",
            Severity::Warning,
            "m".to_owned(),
            "f".to_owned(),
            None,
        );
        let json = serde_json::to_value(&finding).expect("writes");
        assert_eq!(json["severity"], "warning");
        assert_eq!(json["code"], "DURATION");
        assert!(json.get("at").is_none());
    }

    #[test]
    fn a_rate_reads_as_a_person_writes_it() {
        assert_eq!(fps_text(30, 1), "30");
        assert_eq!(fps_text(30000, 1001), "29.97");
        assert_eq!(fps_text(30, 0), "?");
    }
}
