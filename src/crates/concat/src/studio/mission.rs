// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)
//! Holding the timeline against a course lesson's spec, from the clip menu.

use super::*;
use concat_project::mission as lessons;
use concat_project::preflight;

impl Studio {
    /// Checks the active timeline against the lesson the project is named
    /// for (a project called "E.20_LunarRoast_Motion_v02" is E.20's), or
    /// E.19's when its name starts with no lesson code, and says the result
    /// in a notice: all clear, or how many things to fix and the first one.
    /// Changes nothing. The full list, with the step that fixes each, is the
    /// `project_preflight` tool's.
    pub(super) fn check_lesson_spec(&mut self) {
        self.flush_commit();
        let Some(lesson) = lessons::lesson_in_name(&self.project_name)
            .or_else(|| lessons::lesson(lessons::DEFAULT_LESSON))
        else {
            return;
        };
        let Some(spec) = lesson.spec else {
            self.notify(&tf("studio.lessonNoSpec", &[&lesson.code]), false);
            return;
        };
        let findings = preflight::check(self.project(), &spec);
        if findings.is_empty() {
            self.notify(&tf("studio.lessonCheckPassed", &[&lesson.code]), false);
            return;
        }
        let errors = findings
            .iter()
            .filter(|finding| finding.severity == preflight::Severity::Error)
            .count();
        let warnings = findings.len() - errors;
        // Errors are listed first, so the first finding is the worst.
        let first = self.describe_finding(&findings[0], &spec);
        let said = tf(
            "studio.lessonCheckFound",
            &[&lesson.code, &errors, &warnings, &first],
        );
        self.notify(&said, errors > 0);
    }

    /// One finding in the window's language, short enough for a notice. The
    /// fix steps stay in the tool's English, which the assistants read.
    fn describe_finding(
        &self,
        finding: &preflight::Finding,
        spec: &preflight::LessonSpec,
    ) -> String {
        let at = format!("{:.1}", finding.at.unwrap_or(0.0));
        match finding.code {
            "NO_CLIPS" => t("studio.lessonNoClips"),
            "FRAME_SIZE" => tf("studio.lessonFrameSize", &[&spec.width, &spec.height]),
            "FRAME_RATE" => tf(
                "studio.lessonFrameRate",
                &[&preflight::fps_text(spec.rate_num, spec.rate_den)],
            ),
            "DURATION" => tf(
                "studio.lessonDuration",
                &[
                    &format!("{:.2}", self.timeline().duration()),
                    &spec.duration,
                ],
            ),
            "NO_SOUND" => t("studio.lessonNoSound"),
            "NO_CAPTIONS" => t("studio.lessonNoCaptions"),
            "CAPTION_LINES" => tf(
                "studio.lessonCaptionLines",
                &[&at, &spec.max_caption_lines.unwrap_or(2)],
            ),
            "CAPTION_TOO_FAST" => tf("studio.lessonCaptionFast", &[&at]),
            "CAPTION_SAFE_ZONE" => tf("studio.lessonCaptionSafe", &[&at]),
            "CUT_OFF_MARKER" => tf("studio.lessonCutMarker", &[&at]),
            "END_CARD_MOVES" => t("studio.lessonEndCard"),
            "PICTURE_GAP" => tf("studio.lessonGap", &[&at]),
            _ => finding.message.clone(),
        }
    }
}
