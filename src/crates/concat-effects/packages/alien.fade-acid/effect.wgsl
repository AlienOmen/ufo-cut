// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { hold: f32 }
const DIP_COLOUR: vec3<f32> = vec3<f32>(0.784, 1.0, 0.0);

// A dip to a colour: the outgoing shot fades into it, holds, and the incoming
// one fades up out of it. Mixed in the display encoding so it looks even.
fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let base = select(from_at(uv), to_at(uv), progress >= 0.5);
    let bell = 1.0 - abs(progress * 2.0 - 1.0);
    let amt = smoothstep(0.0, 1.0, clamp(bell / max(1.0 - params.hold / 100.0, 0.05), 0.0, 1.0));
    let rgb = from_display(mix(to_display(base.rgb), DIP_COLOUR, amt));
    return vec4<f32>(rgb, base.a);
}
