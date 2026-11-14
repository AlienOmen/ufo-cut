// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)

//! Reading a waveform: where it falls silent, where a voice speaks and where
//! the beat lands.
//!
//! Everything here works on the [`Peaks`] the timeline already draws from,
//! never on the samples, so a long recording is read in a few thousand
//! comparisons and no audio is decoded twice. Times are seconds into the
//! file (its own clock, not the timeline's); a caller that wants timeline
//! seconds maps them through the clip's in-point and speed.
//!
//! A peak is the loudest swing in a bucket, not the average, so levels here
//! read a few decibels hotter than a loudness meter would. The defaults are
//! chosen for that: -40 dB of peak is a room's hiss, not a quiet voice.

use crate::peaks::Peaks;

/// The quietest level a bucket can read as, so a silent bucket is a finite
/// number of decibels rather than minus infinity.
const FLOOR: f32 = 1e-6;

/// The loudest swing of each bucket, in 0..=1.
pub fn envelope(peaks: &Peaks) -> Vec<f32> {
    (0..peaks.len())
        .map(|index| peaks.max[index].abs().max(peaks.min[index].abs()))
        .collect()
}

/// A level as decibels below full scale: 1.0 is 0 dB, silence is -120 dB.
fn to_db(level: f32) -> f32 {
    20.0 * level.max(FLOOR).log10()
}

/// Half-open runs of buckets, `[from, to)`, that are all louder (`loud`) or
/// all quieter (`!loud`) than `threshold_db`.
fn runs(envelope: &[f32], threshold_db: f32, loud: bool) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut open: Option<usize> = None;
    for (index, &level) in envelope.iter().enumerate() {
        let is_loud = to_db(level) >= threshold_db;
        if is_loud == loud {
            if open.is_none() {
                open = Some(index);
            }
        } else if let Some(from) = open.take() {
            found.push((from, index));
        }
    }
    if let Some(from) = open {
        found.push((from, envelope.len()));
    }
    found
}

/// What counts as a silence worth cutting.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SilenceSettings {
    /// Peak level, in dB below full scale, under which a bucket is quiet.
    pub threshold_db: f32,
    /// The shortest silence, in seconds, that is cut. Shorter pauses are a
    /// breath or a comma and stay.
    pub min_silence: f64,
    /// Seconds of the silence kept on each side that touches sound, so a cut
    /// never clips the start of a word or the tail of one.
    pub padding: f64,
}

impl Default for SilenceSettings {
    fn default() -> Self {
        SilenceSettings {
            threshold_db: -40.0,
            min_silence: 0.35,
            padding: 0.06,
        }
    }
}

/// The silences to take out, as `(start, end)` seconds into the file, in
/// order. Each is the quiet run less its padding: a run at the very start or
/// end of the file has no sound on that side and keeps no padding there.
/// Empty for a waveform with no buckets or no rate.
pub fn silence_ranges(peaks: &Peaks, settings: &SilenceSettings) -> Vec<(f64, f64)> {
    let rate = f64::from(peaks.buckets_per_second);
    if peaks.is_empty() || rate <= 0.0 || !rate.is_finite() {
        return Vec::new();
    }
    let envelope = envelope(peaks);
    let count = envelope.len();
    let mut ranges = Vec::new();
    for (from, to) in runs(&envelope, settings.threshold_db, false) {
        let start = from as f64 / rate;
        let end = to as f64 / rate;
        if end - start < settings.min_silence {
            continue;
        }
        let start = if from == 0 { start } else { start + settings.padding };
        let end = if to == count { end } else { end - settings.padding };
        if end - start > 1e-3 {
            ranges.push((start, end));
        }
    }
    ranges
}

/// Where someone is speaking (or anything is playing): the runs louder than
/// `threshold_db`, with gaps shorter than `hold` seconds bridged, so a
/// sentence is one range and not a hundred. `(start, end)` seconds into the
/// file, in order.
pub fn voice_ranges(peaks: &Peaks, threshold_db: f32, hold: f64) -> Vec<(f64, f64)> {
    let rate = f64::from(peaks.buckets_per_second);
    if peaks.is_empty() || rate <= 0.0 || !rate.is_finite() {
        return Vec::new();
    }
    let envelope = envelope(peaks);
    let mut merged: Vec<(f64, f64)> = Vec::new();
    for (from, to) in runs(&envelope, threshold_db, true) {
        let start = from as f64 / rate;
        let end = to as f64 / rate;
        match merged.last_mut() {
            Some(last) if start - last.1 < hold => last.1 = end,
            _ => merged.push((start, end)),
        }
    }
    merged
}

/// How eagerly beats are found.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BeatSettings {
    /// 0 finds only the hardest hits, 1 finds nearly every rise in level.
    pub sensitivity: f32,
    /// The least time, in seconds, between two beats. Closer hits count as
    /// one, the stronger of them.
    pub min_gap: f64,
}

impl Default for BeatSettings {
    fn default() -> Self {
        BeatSettings {
            sensitivity: 0.5,
            min_gap: 0.25,
        }
    }
}

/// The mean of `values` over each window of `width` ending at its index,
/// short at the start (the mean of what there is).
fn trailing_mean(values: &[f32], width: usize) -> Vec<f32> {
    let width = width.max(1);
    let mut sums = Vec::with_capacity(values.len() + 1);
    sums.push(0.0f64);
    for &value in values {
        let last = *sums.last().expect("seeded with a zero");
        sums.push(last + f64::from(value));
    }
    (0..values.len())
        .map(|index| {
            let from = (index + 1).saturating_sub(width);
            ((sums[index + 1] - sums[from]) / (index + 1 - from) as f64) as f32
        })
        .collect()
}

/// The moments, as seconds into the file, where the sound jumps: a kick, a
/// clap, a snare - what a foot taps to. Found as rises in the smoothed level
/// above its own recent average, so a steady tone or a swell is not a beat
/// and a hit is, at any volume. A file that begins loud has a beat at its
/// first instant (sound began there), which is what a clip trimmed on the
/// beat needs. Percussive music reads best; a drone or a voice alone gives
/// few.
pub fn beat_times(peaks: &Peaks, settings: &BeatSettings) -> Vec<f64> {
    let rate = f64::from(peaks.buckets_per_second);
    if peaks.len() < 8 || rate <= 0.0 || !rate.is_finite() {
        return Vec::new();
    }
    let smooth = ((0.02 * rate).round() as usize).max(1);
    // How far each moment stands above the average of the last fifth of a
    // second: a hit is a spike over that, a plateau is not.
    let back = ((0.2 * rate).round() as usize).max(2);
    // The file is read as if it began after a stretch of silence, so a hit
    // on its very first bucket - a clip trimmed on the beat - still rises
    // out of something. `lead` buckets of it come first and are not counted.
    let lead = smooth.max(back);
    let mut padded = vec![0.0f32; lead];
    padded.extend(envelope(peaks));
    let level = trailing_mean(&padded, smooth);
    let recent = trailing_mean(&level, back);
    let flux: Vec<f32> = level
        .iter()
        .zip(recent.iter())
        .map(|(now, before)| (now - before).max(0.0))
        .collect();

    let real = &flux[lead..];
    let count = real.len() as f32;
    let mean = real.iter().sum::<f32>() / count;
    let spread = (real.iter().map(|v| (v - mean) * (v - mean)).sum::<f32>() / count).sqrt();
    let sensitivity = settings.sensitivity.clamp(0.0, 1.0);
    let bar = mean + (3.0 - 2.2 * sensitivity) * spread;

    // Local peaks of the rise that clear the bar, strongest first, each
    // keeping the others at least `min_gap` away.
    let mut candidates: Vec<usize> = (lead..flux.len() - 1)
        .filter(|&i| flux[i] > bar && flux[i] >= flux[i - 1] && flux[i] > flux[i + 1])
        .collect();
    candidates.sort_by(|a, b| flux[*b].total_cmp(&flux[*a]));
    let mut chosen: Vec<usize> = Vec::new();
    for index in candidates {
        let at = index as f64 / rate;
        if chosen
            .iter()
            .all(|&other| (other as f64 / rate - at).abs() >= settings.min_gap)
        {
            chosen.push(index);
        }
    }
    chosen.sort_unstable();
    // The smoothed rise peaks a few buckets after the hit began; the time
    // given is where it began.
    let lag = (smooth - 1) as f64;
    chosen
        .into_iter()
        .map(|index| (((index - lead) as f64 - lag) / rate).max(0.0))
        .collect()
}

/// The tempo the beats suggest, in beats per minute: the median gap between
/// them, doubled or halved into 70..=180 where that is a likelier tempo than
/// a half or a double. None under three beats.
pub fn estimate_bpm(beats: &[f64]) -> Option<f64> {
    if beats.len() < 3 {
        return None;
    }
    let mut gaps: Vec<f64> = beats.windows(2).map(|pair| pair[1] - pair[0]).collect();
    gaps.retain(|gap| *gap > 0.0 && gap.is_finite());
    if gaps.is_empty() {
        return None;
    }
    gaps.sort_by(f64::total_cmp);
    let mut bpm = 60.0 / gaps[gaps.len() / 2];
    while bpm < 70.0 {
        bpm *= 2.0;
    }
    while bpm > 180.0 {
        bpm /= 2.0;
    }
    Some(bpm)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A waveform whose buckets swing `levels` either way of zero.
    fn peaks_of(levels: &[f32], rate: f32) -> Peaks {
        Peaks {
            min: levels.iter().map(|level| -level).collect(),
            max: levels.to_vec(),
            buckets_per_second: rate,
        }
    }

    /// `seconds` of `level` at 200 buckets a second.
    fn hold(level: f32, seconds: f32) -> Vec<f32> {
        vec![level; (seconds * 200.0).round() as usize]
    }

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn a_silence_between_two_sounds_is_one_padded_range() {
        let levels = [hold(0.5, 1.0), hold(0.0, 1.0), hold(0.5, 1.0)].concat();
        let ranges = silence_ranges(&peaks_of(&levels, 200.0), &SilenceSettings::default());
        assert_eq!(ranges.len(), 1);
        assert!(near(ranges[0].0, 1.06) && near(ranges[0].1, 1.94), "{ranges:?}");
    }

    #[test]
    fn a_pause_shorter_than_the_minimum_stays() {
        let levels = [hold(0.5, 1.0), hold(0.0, 0.2), hold(0.5, 1.0)].concat();
        let ranges = silence_ranges(&peaks_of(&levels, 200.0), &SilenceSettings::default());
        assert!(ranges.is_empty(), "{ranges:?}");
    }

    #[test]
    fn the_ends_of_the_file_keep_no_padding() {
        let levels = [hold(0.0, 0.5), hold(0.5, 1.0), hold(0.0, 0.5)].concat();
        let ranges = silence_ranges(&peaks_of(&levels, 200.0), &SilenceSettings::default());
        assert_eq!(ranges.len(), 2, "{ranges:?}");
        assert!(near(ranges[0].0, 0.0) && near(ranges[0].1, 0.44), "{ranges:?}");
        assert!(near(ranges[1].0, 1.56) && near(ranges[1].1, 2.0), "{ranges:?}");
    }

    #[test]
    fn a_file_of_nothing_is_one_silence_and_no_buckets_is_none() {
        let all = silence_ranges(&peaks_of(&hold(0.0, 2.0), 200.0), &SilenceSettings::default());
        assert_eq!(all.len(), 1);
        assert!(near(all[0].0, 0.0) && near(all[0].1, 2.0));
        let none = silence_ranges(&peaks_of(&[], 200.0), &SilenceSettings::default());
        assert!(none.is_empty());
        let no_rate = silence_ranges(&peaks_of(&hold(0.0, 1.0), 0.0), &SilenceSettings::default());
        assert!(no_rate.is_empty());
    }

    #[test]
    fn a_higher_threshold_treats_quiet_speech_as_silence() {
        // 0.02 is about -34 dB: sound at -40, silence at -30.
        let levels = [hold(0.5, 1.0), hold(0.02, 1.0), hold(0.5, 1.0)].concat();
        let peaks = peaks_of(&levels, 200.0);
        let low = SilenceSettings::default();
        let high = SilenceSettings {
            threshold_db: -30.0,
            ..low
        };
        assert!(silence_ranges(&peaks, &low).is_empty());
        assert_eq!(silence_ranges(&peaks, &high).len(), 1);
    }

    #[test]
    fn voice_gaps_shorter_than_the_hold_are_bridged() {
        let levels = [hold(0.5, 1.0), hold(0.0, 0.2), hold(0.5, 1.0)].concat();
        let peaks = peaks_of(&levels, 200.0);
        let bridged = voice_ranges(&peaks, -40.0, 0.3);
        assert_eq!(bridged.len(), 1, "{bridged:?}");
        assert!(near(bridged[0].0, 0.0) && near(bridged[0].1, 2.2), "{bridged:?}");
        let apart = voice_ranges(&peaks, -40.0, 0.1);
        assert_eq!(apart.len(), 2, "{apart:?}");
    }

    /// A kick every `gap` seconds: a 50 ms burst over silence.
    fn kicks(count: usize, gap: f32) -> Vec<f32> {
        let mut levels = Vec::new();
        for _ in 0..count {
            levels.extend(hold(0.9, 0.05));
            levels.extend(hold(0.0, gap - 0.05));
        }
        levels
    }

    #[test]
    fn a_steady_kick_is_found_beat_by_beat_and_reads_as_its_tempo() {
        let beats = beat_times(&peaks_of(&kicks(8, 0.5), 200.0), &BeatSettings::default());
        assert_eq!(beats.len(), 8, "{beats:?}");
        for (index, beat) in beats.iter().enumerate() {
            assert!((beat - 0.5 * index as f64).abs() < 0.06, "{beats:?}");
        }
        let bpm = estimate_bpm(&beats).expect("eight beats have a tempo");
        assert!((bpm - 120.0).abs() < 4.0, "{bpm}");
    }

    #[test]
    fn two_hits_closer_than_the_gap_are_one_beat() {
        let mut levels = Vec::new();
        for _ in 0..4 {
            levels.extend(hold(0.9, 0.03));
            levels.extend(hold(0.0, 0.07));
            levels.extend(hold(0.8, 0.03));
            levels.extend(hold(0.0, 0.37));
        }
        let beats = beat_times(&peaks_of(&levels, 200.0), &BeatSettings::default());
        assert!(beats.len() <= 4, "{beats:?}");
    }

    #[test]
    fn a_steady_tone_has_no_beats_after_its_start_and_a_short_file_has_none() {
        let tone = beat_times(&peaks_of(&hold(0.5, 3.0), 200.0), &BeatSettings::default());
        assert!(tone.iter().all(|beat| *beat < 0.1), "{tone:?}");
        let short = beat_times(&peaks_of(&[0.5, 0.0, 0.5], 200.0), &BeatSettings::default());
        assert!(short.is_empty());
    }

    #[test]
    fn the_tempo_is_folded_into_a_likely_range() {
        // Gaps of 1.0 s are 60 bpm: doubled to 120.
        assert!(near(estimate_bpm(&[0.0, 1.0, 2.0, 3.0]).unwrap(), 120.0));
        // Gaps of 0.25 s are 240 bpm: halved to 120.
        assert!(near(estimate_bpm(&[0.0, 0.25, 0.5, 0.75]).unwrap(), 120.0));
        assert!(estimate_bpm(&[0.0, 1.0]).is_none());
    }
}
