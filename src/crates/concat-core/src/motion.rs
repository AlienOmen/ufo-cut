// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah

//! A clip's moves: how it comes on, how it goes off, and what it does in
//! between.
//!
//! A move is applied where the clip is placed, after its keys: it moves,
//! turns, scales and fades the whole picture in the frame, so a small
//! picture slides as a card and a title pops as a title - never the pixels
//! inside a fixed box. Every move is a pure function of the clip's own time
//! and length, so a frame drawn cold is the frame played into.
//!
//! An entrance and an exit are the two halves of one motion through rest:
//! [`Move::pose`] at `s` from -1 (arriving) through 0 (at rest) to 1
//! (gone). "Slide Left" comes on moving left and goes off moving left;
//! "Zoom In" grows into place and grows on out of it. A loop runs the
//! whole clip, under both.

/// How a clip comes on or goes off. Named for the way it moves.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Move {
    /// Fades through.
    Fade,
    /// Travels left a whole frame.
    SlideLeft,
    /// Travels right a whole frame.
    SlideRight,
    /// Travels up a whole frame.
    SlideUp,
    /// Travels down a whole frame.
    SlideDown,
    /// Grows: from small into place, and on past it.
    ZoomIn,
    /// Shrinks: from large into place, and on to small.
    ZoomOut,
    /// Springs up from nothing past its size and settles; goes with a
    /// little wind-up.
    Pop,
    /// Falls: in from above with a bounce, out below under gravity.
    Drop,
    /// Floats up a little as it fades.
    Rise,
    /// Turns a whole turn clockwise as it grows or shrinks.
    Spin,
    /// Turns over like a card about its upright.
    Flip,
    /// Swings in like a sign on a hook; drops off its hinge on the way out.
    Swing,
    /// Squashed flat and wide, springing back to shape.
    Squash,
    /// UFO Cut: Down a beam of light, thin at first.
    Beam,
    /// UFO Cut: Lifted away, shrinking and turning.
    Abduct,
    /// UFO Cut: Stretched to a streak, then itself.
    Warp,
    /// UFO Cut: In fits and starts, jumping about.
    Glitch,
    /// UFO Cut: Lands large and settles with a bounce.
    Stomp,
    /// UFO Cut: A fast pass from the right, smeared.
    WhipLeft,
    /// UFO Cut: A fast pass from the left, smeared.
    WhipRight,
    /// UFO Cut: Rolls in along the ground.
    Roll,
    /// UFO Cut: Falls in, turning as it comes.
    Tumble,
    /// UFO Cut: Opens from a line across its middle.
    Unfold,
    /// UFO Cut: Springs out sideways from a line.
    Stretch,
    /// UFO Cut: Leans in from below as it appears.
    Tilt,
    /// UFO Cut: Hits the frame from just past its size.
    Punch,
    /// UFO Cut: Grows from nothing, with no fade.
    Shrink,
    /// UFO Cut: Shot in from the side, past its place and back.
    Slingshot,
    /// UFO Cut: Flickers on like a failing light.
    Blink,
}

impl Move {
    /// Every move, in the order the picker lists them.
    pub const ALL: [Move; 30] = [
        Move::Fade,
        Move::SlideLeft,
        Move::SlideRight,
        Move::SlideUp,
        Move::SlideDown,
        Move::ZoomIn,
        Move::ZoomOut,
        Move::Pop,
        Move::Drop,
        Move::Rise,
        Move::Spin,
        Move::Flip,
        Move::Swing,
        Move::Squash,
        Move::Beam,
        Move::Abduct,
        Move::Warp,
        Move::Glitch,
        Move::Stomp,
        Move::WhipLeft,
        Move::WhipRight,
        Move::Roll,
        Move::Tumble,
        Move::Unfold,
        Move::Stretch,
        Move::Tilt,
        Move::Punch,
        Move::Shrink,
        Move::Slingshot,
        Move::Blink,
    ];

    /// The move's name in documents and the API.
    pub fn name(self) -> &'static str {
        match self {
            Move::Fade => "fade",
            Move::SlideLeft => "slide-left",
            Move::SlideRight => "slide-right",
            Move::SlideUp => "slide-up",
            Move::SlideDown => "slide-down",
            Move::ZoomIn => "zoom-in",
            Move::ZoomOut => "zoom-out",
            Move::Pop => "pop",
            Move::Drop => "drop",
            Move::Rise => "rise",
            Move::Spin => "spin",
            Move::Flip => "flip",
            Move::Swing => "swing",
            Move::Squash => "squash",
            Move::Beam => "beam",
            Move::Abduct => "abduct",
            Move::Warp => "warp",
            Move::Glitch => "glitch",
            Move::Stomp => "stomp",
            Move::WhipLeft => "whip-left",
            Move::WhipRight => "whip-right",
            Move::Roll => "roll",
            Move::Tumble => "tumble",
            Move::Unfold => "unfold",
            Move::Stretch => "stretch",
            Move::Tilt => "tilt",
            Move::Punch => "punch",
            Move::Shrink => "shrink",
            Move::Slingshot => "slingshot",
            Move::Blink => "blink",
        }
    }

    /// The move a name spells, ignoring case; None for any other.
    pub fn parse(name: &str) -> Option<Move> {
        Move::ALL
            .into_iter()
            .find(|candidate| candidate.name().eq_ignore_ascii_case(name.trim()))
    }

    /// The pose at `s`: -1 arriving, 0 at rest, 1 gone; eased already.
    pub fn pose(self, s: f64) -> Pose {
        let s = s.clamp(-1.0, 1.0);
        let gone = s.abs();
        // Coming on decelerates into place; going off accelerates away.
        let travel = if s < 0.0 {
            -(1.0 - ease_out(1.0 - gone))
        } else {
            ease_in(gone)
        };
        let mut pose = Pose::REST;
        match self {
            Move::Fade => pose.opacity = 1.0 - gone,
            Move::SlideLeft => pose.dx = -travel,
            Move::SlideRight => pose.dx = travel,
            Move::SlideUp => pose.dy = -travel,
            Move::SlideDown => pose.dy = travel,
            Move::ZoomIn => {
                pose.scale = (travel * 1.2).exp2();
                pose.opacity = 1.0 - travel.abs().powf(1.5);
            }
            Move::ZoomOut => {
                pose.scale = (-travel * 1.2).exp2();
                pose.opacity = 1.0 - travel.abs().powf(1.5);
            }
            Move::Pop => {
                if s < 0.0 {
                    let p = 1.0 - gone;
                    pose.scale = back_out(p).max(0.0);
                    pose.opacity = (p * 5.0).min(1.0);
                } else {
                    pose.scale = 1.0 - back_in(gone);
                    pose.opacity = ((1.0 - gone) * 5.0).min(1.0);
                }
            }
            Move::Drop => {
                if s < 0.0 {
                    pose.dy = -(1.0 - bounce_out(1.0 - gone)) * 1.1;
                } else {
                    pose.dy = gone * gone * 1.2;
                }
            }
            Move::Rise => {
                pose.dy = -travel * 0.12;
                pose.opacity = 1.0 - travel.abs();
            }
            Move::Spin => {
                pose.rotation = travel * 360.0;
                pose.scale = (1.0 - travel.abs()).max(0.0);
                pose.opacity = 1.0 - travel.abs().powi(3);
            }
            Move::Flip => pose.stretch_x = (travel * std::f64::consts::FRAC_PI_2).cos(),
            Move::Swing => {
                if s < 0.0 {
                    let p = 1.0 - gone;
                    pose.rotation = -40.0 * (1.0 - p).powi(2) * (p * 11.0).cos();
                    pose.opacity = (p * 4.0).min(1.0);
                } else {
                    pose.rotation = -35.0 * gone * gone;
                    pose.dy = gone.powi(3) * 1.3;
                }
            }
            Move::Squash => {
                if s < 0.0 {
                    let p = elastic_out(1.0 - gone);
                    pose.stretch_x = 1.0 + (1.0 - p) * 1.4;
                    pose.stretch_y = p.max(0.0);
                    pose.opacity = ((1.0 - gone) * 6.0).min(1.0);
                } else {
                    pose.stretch_x = 1.0 + gone * 0.8;
                    pose.stretch_y = 1.0 - gone;
                }
            }
            // UFO Cut: the ALIEN moves.
            Move::Beam => {
                if s < 0.0 {
                    let p = ease_out(1.0 - gone);
                    pose.dy = -(1.0 - p) * 0.6;
                    pose.stretch_x = 0.2 + 0.8 * p;
                    pose.opacity = ((1.0 - gone) * 3.0).min(1.0);
                } else {
                    pose.dy = -ease_in(gone) * 0.6;
                    pose.stretch_x = 1.0 - 0.8 * gone;
                    pose.opacity = 1.0 - gone;
                }
            }
            Move::Abduct => {
                pose.dy = -travel.abs() * 0.5;
                pose.scale = (1.0 - travel.abs()).max(0.0);
                pose.rotation = travel * 90.0;
                pose.opacity = 1.0 - travel.abs().powi(3);
            }
            Move::Warp => {
                pose.stretch_x = 1.0 + travel.abs() * 4.0;
                pose.stretch_y = (1.0 - travel.abs() * 0.9).max(0.05);
                pose.opacity = 1.0 - travel.abs().powi(2);
            }
            Move::Glitch => {
                let step = (gone * 10.0).floor();
                pose.dx = 0.06 * gone * (unit(step, 5) * 2.0 - 1.0);
                pose.dy = 0.02 * gone * (unit(step, 6) * 2.0 - 1.0);
                pose.opacity = (((1.0 - gone) * 6.0).ceil() / 6.0).clamp(0.0, 1.0);
            }
            Move::Stomp => {
                if s < 0.0 {
                    let p = 1.0 - gone;
                    pose.scale = 1.0 + (1.0 - bounce_out(p)) * 2.0;
                    pose.opacity = (p * 6.0).min(1.0);
                } else {
                    pose.scale = 1.0 + gone * gone * 2.0;
                    pose.opacity = 1.0 - gone;
                }
            }
            Move::WhipLeft => {
                pose.dx = -travel * 1.2;
                pose.stretch_x = 1.0 + travel.abs() * 0.8;
            }
            Move::WhipRight => {
                pose.dx = travel * 1.2;
                pose.stretch_x = 1.0 + travel.abs() * 0.8;
            }
            Move::Roll => {
                pose.dx = travel;
                pose.rotation = travel * 360.0;
            }
            Move::Tumble => {
                pose.dy = travel * 1.1;
                pose.rotation = travel * 180.0;
            }
            Move::Unfold => pose.stretch_y = (travel * std::f64::consts::FRAC_PI_2).cos(),
            Move::Stretch => {
                if s < 0.0 {
                    pose.stretch_x = elastic_out(1.0 - gone).max(0.0);
                    pose.opacity = ((1.0 - gone) * 6.0).min(1.0);
                } else {
                    pose.stretch_x = 1.0 - gone;
                }
            }
            Move::Tilt => {
                pose.rotation = travel * 14.0;
                pose.dy = -travel * 0.08;
                pose.opacity = 1.0 - travel.abs();
            }
            Move::Punch => {
                pose.scale = 1.0 + travel.abs() * 0.6;
                pose.opacity = 1.0 - travel.abs().powi(2);
            }
            Move::Shrink => pose.scale = (1.0 - travel.abs()).max(0.0),
            Move::Slingshot => {
                if s < 0.0 {
                    pose.dx = -(1.0 - back_out(1.0 - gone));
                } else {
                    pose.dx = back_in(gone);
                }
            }
            Move::Blink => {
                let step = (gone * 8.0).floor();
                pose.opacity = if step % 2.0 < 1.0 { 1.0 - gone } else { 0.0 };
            }
        }
        pose
    }
}

/// What a clip does over its whole length, again and again.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Cycle {
    /// Swells a little and settles.
    Pulse,
    /// Two quick beats, then rest.
    Heartbeat,
    /// Bobs up and down with a slight tilt.
    Float,
    /// Rocks back and forth quickly.
    Wiggle,
    /// Leans side to side, slowly.
    Sway,
    /// Twitches in place.
    Jitter,
    /// Turns round and round.
    Spin,
    /// Dims and returns now and then, like a failing bulb.
    Flicker,
    /// Fades a little and back, slowly.
    Breathe,
    /// UFO Cut: Hangs in the air like a craft.
    Hover,
    /// UFO Cut: Goes round a small circle.
    Orbit,
    /// UFO Cut: A fast shake from side to side.
    Shake,
    /// UFO Cut: Dips and comes back.
    Nod,
    /// UFO Cut: Rocks widely about its middle.
    Rock,
    /// UFO Cut: Bounces on the spot.
    Bounce,
    /// UFO Cut: Swells large and back.
    Throb,
    /// UFO Cut: On and off, hard.
    Strobe,
    /// UFO Cut: Squashes one way then the other.
    Wobble,
    /// UFO Cut: Turns in steps, like a second hand.
    Tick,
}

impl Cycle {
    /// Every loop, in the order the picker lists them.
    pub const ALL: [Cycle; 19] = [
        Cycle::Pulse,
        Cycle::Heartbeat,
        Cycle::Float,
        Cycle::Wiggle,
        Cycle::Sway,
        Cycle::Jitter,
        Cycle::Spin,
        Cycle::Flicker,
        Cycle::Breathe,
        Cycle::Hover,
        Cycle::Orbit,
        Cycle::Shake,
        Cycle::Nod,
        Cycle::Rock,
        Cycle::Bounce,
        Cycle::Throb,
        Cycle::Strobe,
        Cycle::Wobble,
        Cycle::Tick,
    ];

    /// The loop's name in documents and the API.
    pub fn name(self) -> &'static str {
        match self {
            Cycle::Pulse => "pulse",
            Cycle::Heartbeat => "heartbeat",
            Cycle::Float => "float",
            Cycle::Wiggle => "wiggle",
            Cycle::Sway => "sway",
            Cycle::Jitter => "jitter",
            Cycle::Spin => "spin",
            Cycle::Flicker => "flicker",
            Cycle::Breathe => "breathe",
            Cycle::Hover => "hover",
            Cycle::Orbit => "orbit",
            Cycle::Shake => "shake",
            Cycle::Nod => "nod",
            Cycle::Rock => "rock",
            Cycle::Bounce => "bounce",
            Cycle::Throb => "throb",
            Cycle::Strobe => "strobe",
            Cycle::Wobble => "wobble",
            Cycle::Tick => "tick",
        }
    }

    /// The loop a name spells, ignoring case; None for any other.
    pub fn parse(name: &str) -> Option<Cycle> {
        Cycle::ALL
            .into_iter()
            .find(|candidate| candidate.name().eq_ignore_ascii_case(name.trim()))
    }

    /// The pose `phase` cycles in (a whole number is a lap), at `amount`
    /// (1 as designed, 0 still, 2 twice as much).
    pub fn pose(self, phase: f64, amount: f64) -> Pose {
        let a = amount.max(0.0);
        let turn = std::f64::consts::TAU * phase;
        let lap = phase - phase.floor();
        let mut pose = Pose::REST;
        match self {
            Cycle::Pulse => pose.scale = 1.0 + 0.06 * a * (0.5 - 0.5 * turn.cos()),
            Cycle::Heartbeat => {
                let beat = |at: f64| (-((lap - at) / 0.045).powi(2)).exp();
                pose.scale = 1.0 + 0.09 * a * (beat(0.1) + 0.7 * beat(0.28));
            }
            Cycle::Float => {
                pose.dy = -0.02 * a * turn.sin();
                pose.rotation = 1.5 * a * (turn + 1.0).sin();
            }
            Cycle::Wiggle => pose.rotation = 6.0 * a * (turn * 2.0).sin(),
            Cycle::Sway => {
                pose.rotation = 4.0 * a * turn.sin();
                pose.dx = 0.012 * a * turn.sin();
            }
            Cycle::Jitter => {
                let step = (phase * 12.0).floor();
                pose.dx = 0.008 * a * (unit(step, 1) * 2.0 - 1.0);
                pose.dy = 0.008 * a * (unit(step, 2) * 2.0 - 1.0);
                pose.rotation = 0.8 * a * (unit(step, 3) * 2.0 - 1.0);
            }
            Cycle::Spin => pose.rotation = if a > 0.0 { 360.0 * lap } else { 0.0 },
            Cycle::Flicker => {
                let step = (phase * 10.0).floor();
                if unit(step, 4) > 0.65 {
                    pose.opacity = (1.0 - 0.6 * a).clamp(0.0, 1.0);
                }
            }
            Cycle::Breathe => {
                pose.opacity = (1.0 - 0.35 * a * (0.5 - 0.5 * turn.cos())).clamp(0.0, 1.0);
            }
            // UFO Cut: the ALIEN loops.
            Cycle::Hover => {
                pose.dy = -0.03 * a * turn.sin();
                pose.scale = 1.0 + 0.015 * a * (turn * 2.0).sin();
            }
            Cycle::Orbit => {
                pose.dx = 0.02 * a * turn.cos();
                pose.dy = 0.02 * a * turn.sin();
            }
            Cycle::Shake => pose.dx = 0.012 * a * (turn * 6.0).sin(),
            Cycle::Nod => pose.dy = 0.025 * a * (0.5 - 0.5 * turn.cos()),
            Cycle::Rock => pose.rotation = 12.0 * a * turn.sin(),
            Cycle::Bounce => pose.dy = -0.05 * a * (turn * 0.5).sin().abs(),
            Cycle::Throb => pose.scale = 1.0 + 0.15 * a * (0.5 - 0.5 * turn.cos()),
            Cycle::Strobe => {
                if a > 0.0 && lap >= 0.5 {
                    pose.opacity = (1.0 - a).clamp(0.0, 1.0);
                }
            }
            Cycle::Wobble => {
                pose.stretch_x = 1.0 + 0.08 * a * turn.sin();
                pose.stretch_y = 1.0 - 0.08 * a * turn.sin();
            }
            Cycle::Tick => pose.rotation = if a > 0.0 { 360.0 * (lap * 12.0).floor() / 12.0 } else { 0.0 },
        }
        pose
    }
}

/// An entrance or an exit: which move, over how many seconds.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Moving {
    /// The move.
    pub kind: Move,
    /// Seconds it takes.
    pub seconds: f64,
}

/// A loop: which, how many seconds a lap, and how much.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Looping {
    /// The loop.
    pub kind: Cycle,
    /// Seconds a lap takes.
    pub seconds: f64,
    /// How much of it: 1 as designed.
    pub amount: f64,
}

/// A clip's entrance, exit and loop, any of them.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Motion {
    /// How it comes on.
    pub enter: Option<Moving>,
    /// How it goes off.
    pub exit: Option<Moving>,
    /// What it does all along.
    pub cycle: Option<Looping>,
}

impl Motion {
    /// True when it does nothing.
    pub fn is_empty(&self) -> bool {
        self.enter.is_none() && self.exit.is_none() && self.cycle.is_none()
    }

    /// The pose `local` seconds into a clip `length` seconds long. An
    /// entrance and an exit that would overlap on a short clip share it,
    /// each shortened by the same factor.
    pub fn pose_at(&self, local: f64, length: f64) -> Pose {
        let length = length.max(0.0);
        let local = local.clamp(0.0, length);
        let enter = self.enter.map_or(0.0, |m| m.seconds.max(0.0));
        let exit = self.exit.map_or(0.0, |m| m.seconds.max(0.0));
        let fit = if enter + exit > length && enter + exit > 0.0 {
            length / (enter + exit)
        } else {
            1.0
        };
        let mut pose = Pose::REST;
        if let Some(moving) = self.enter {
            let span = enter * fit;
            if span > 0.0 && local < span {
                pose = pose.then(moving.kind.pose(local / span - 1.0));
            }
        }
        if let Some(moving) = self.exit {
            let span = exit * fit;
            let left = length - local;
            if span > 0.0 && left < span {
                pose = pose.then(moving.kind.pose(1.0 - left / span));
            }
        }
        if let Some(looping) = self.cycle {
            let lap = looping.seconds.max(0.05);
            pose = pose.then(looping.kind.pose(local / lap, looping.amount));
        }
        pose
    }
}

/// Where a move has the picture: moved by `dx` frame widths and `dy` frame
/// heights, its scale and stretches multiplied, turned by `rotation`
/// degrees clockwise, its opacity multiplied.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pose {
    /// Across, in frame widths.
    pub dx: f64,
    /// Down, in frame heights.
    pub dy: f64,
    /// A factor on the scale.
    pub scale: f64,
    /// Degrees clockwise.
    pub rotation: f64,
    /// A factor on the opacity.
    pub opacity: f64,
    /// A factor on the width alone.
    pub stretch_x: f64,
    /// A factor on the height alone.
    pub stretch_y: f64,
}

impl Pose {
    /// No move at all.
    pub const REST: Pose = Pose {
        dx: 0.0,
        dy: 0.0,
        scale: 1.0,
        rotation: 0.0,
        opacity: 1.0,
        stretch_x: 1.0,
        stretch_y: 1.0,
    };

    /// This pose with `other` on top of it.
    pub fn then(self, other: Pose) -> Pose {
        Pose {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
            scale: self.scale * other.scale,
            rotation: self.rotation + other.rotation,
            opacity: self.opacity * other.opacity,
            stretch_x: self.stretch_x * other.stretch_x,
            stretch_y: self.stretch_y * other.stretch_y,
        }
    }
}

fn ease_out(p: f64) -> f64 {
    1.0 - (1.0 - p).powi(3)
}

fn ease_in(p: f64) -> f64 {
    p * p * p
}

/// Past the end and back: an overshoot of about a tenth.
fn back_out(p: f64) -> f64 {
    let c = 1.70158;
    let q = p - 1.0;
    1.0 + (c + 1.0) * q.powi(3) + c * q * q
}

/// A wind-up before the move: dips below nought first.
fn back_in(p: f64) -> f64 {
    let c = 1.70158;
    (c + 1.0) * p.powi(3) - c * p * p
}

/// A ball dropped on a floor, landing at one.
fn bounce_out(p: f64) -> f64 {
    let n = 7.5625;
    let d = 2.75;
    if p < 1.0 / d {
        n * p * p
    } else if p < 2.0 / d {
        let q = p - 1.5 / d;
        n * q * q + 0.75
    } else if p < 2.5 / d {
        let q = p - 2.25 / d;
        n * q * q + 0.9375
    } else {
        let q = p - 2.625 / d;
        n * q * q + 0.984375
    }
}

/// A spring let go at nought, ringing down onto one.
fn elastic_out(p: f64) -> f64 {
    if p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return 1.0;
    }
    1.0 - (-7.0 * p).exp() * (p * 14.0).cos()
}

/// A number in 0..1 from a step and a stream, the same on every machine.
fn unit(step: f64, stream: u32) -> f64 {
    let mut h = (step as i64 as u32) ^ stream.wrapping_mul(0x9e37_79b9);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^= h >> 16;
    f64::from(h) / f64::from(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn every_move_is_at_rest_at_rest() {
        for kind in Move::ALL {
            assert_eq!(kind.pose(0.0), Pose::REST, "{kind:?}");
        }
    }

    #[test]
    fn a_slide_comes_on_and_goes_off_the_way_it_is_named() {
        assert!(
            close(Move::SlideLeft.pose(-1.0).dx, 1.0),
            "arrives from the right"
        );
        assert!(
            close(Move::SlideLeft.pose(1.0).dx, -1.0),
            "leaves to the left"
        );
        assert!(
            close(Move::SlideUp.pose(-1.0).dy, 1.0),
            "arrives from below"
        );
    }

    #[test]
    fn an_entrance_and_an_exit_frame_the_clip() {
        let motion = Motion {
            enter: Some(Moving {
                kind: Move::Fade,
                seconds: 0.5,
            }),
            exit: Some(Moving {
                kind: Move::Fade,
                seconds: 0.5,
            }),
            cycle: None,
        };
        assert!(close(motion.pose_at(0.0, 4.0).opacity, 0.0));
        assert!(close(motion.pose_at(0.25, 4.0).opacity, 0.5));
        assert_eq!(motion.pose_at(2.0, 4.0), Pose::REST);
        assert!(close(motion.pose_at(3.75, 4.0).opacity, 0.5));
        assert!(close(motion.pose_at(4.0, 4.0).opacity, 0.0));
    }

    #[test]
    fn a_short_clip_shares_itself_between_entrance_and_exit() {
        let motion = Motion {
            enter: Some(Moving {
                kind: Move::Fade,
                seconds: 1.0,
            }),
            exit: Some(Moving {
                kind: Move::Fade,
                seconds: 1.0,
            }),
            cycle: None,
        };
        // One second long: each gets half, so the middle is at rest.
        assert!(close(motion.pose_at(0.5, 1.0).opacity, 1.0));
        assert!(close(motion.pose_at(0.25, 1.0).opacity, 0.5));
    }

    #[test]
    fn a_loop_comes_back_to_where_it_began() {
        // Jitter and Flicker are random steps by design, not laps.
        for kind in Cycle::ALL
            .into_iter()
            .filter(|kind| !matches!(kind, Cycle::Jitter | Cycle::Flicker))
        {
            let a = kind.pose(0.0, 1.0);
            let b = kind.pose(3.0, 1.0);
            assert!(close(a.scale, b.scale) && close(a.dy, b.dy), "{kind:?}");
        }
    }

    #[test]
    fn a_clip_slides_in_and_fades_out_where_it_is_placed() {
        use crate::time::Rational;
        use crate::timeline::{Clip, MediaRef};
        let mut clip = Clip::new(
            MediaRef::new("a.mp4"),
            Rational::from_int(2),
            Rational::from_int(4),
        );
        clip.motion = Some(Motion {
            enter: Some(Moving {
                kind: Move::SlideLeft,
                seconds: 1.0,
            }),
            exit: Some(Moving {
                kind: Move::Fade,
                seconds: 1.0,
            }),
            cycle: None,
        });
        // Its first frame is a whole frame to the right; by a second in it is home.
        assert!(close(
            clip.transform_at(Rational::from_int(2)).offset_x,
            1.0
        ));
        assert!(close(
            clip.transform_at(Rational::from_int(3)).offset_x,
            0.0
        ));
        assert!((clip.opacity_at(Rational::from_int(4)) - 1.0).abs() < 1e-6);
        assert!((clip.opacity_at(Rational::new(11, 2)) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn names_read_back() {
        for kind in Move::ALL {
            assert_eq!(Move::parse(kind.name()), Some(kind));
        }
        for kind in Cycle::ALL {
            assert_eq!(Cycle::parse(kind.name()), Some(kind));
        }
    }
}
