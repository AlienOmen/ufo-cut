// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)
//! Cutting a talk's silences from the clip menu.

use super::*;
use concat_host::autocut::{self as sound, SilenceSettings};

impl Studio {
    /// Takes the silent gaps out of clip `id`'s sound, and out of the rest of
    /// the selection with it when `id` is one of several, so a talk's picture
    /// and its sound stay in step. One undoable edit; the lanes close up.
    /// Says how much went, or why nothing did.
    pub(super) fn cut_silences(&mut self, id: &str) {
        self.flush_commit();
        let wanted: Vec<String> =
            if self.selection.len() > 1 && self.selection.iter().any(|held| held == id) {
                self.selection.clone()
            } else {
                vec![id.to_owned()]
            };
        // Clips on a locked lane are left as they are.
        let ids: Vec<String> = wanted
            .into_iter()
            .filter(|clip_id| {
                self.clip(clip_id)
                    .is_some_and(|clip| !self.locked(&clip.track_id))
            })
            .collect();
        if ids.is_empty() {
            return;
        }
        let folder = self
            .session
            .as_ref()
            .map(|session| session.path().to_owned());
        let spans = match sound::silences(
            self.project(),
            id,
            folder.as_deref(),
            &SilenceSettings::default(),
        ) {
            Ok(spans) => spans,
            Err(reason) => {
                self.notify(&reason, true);
                return;
            }
        };
        if spans.is_empty() {
            self.notify(&t("studio.noSilencesFound"), false);
            return;
        }
        let count = spans.len();
        let seconds: f64 = spans.iter().map(|span| span.end - span.start).sum();
        // A refusal raises its own notice; the success line is only said
        // when none was.
        let seen = self.toast.token;
        self.apply(Command::CutRanges {
            clip_ids: ids,
            ranges: spans,
            ripple: true,
        });
        if self.toast.token == seen {
            let said = tf("studio.silencesCut", &[&count, &format!("{seconds:.1}")]);
            self.notify(&said, false);
        }
    }
}
