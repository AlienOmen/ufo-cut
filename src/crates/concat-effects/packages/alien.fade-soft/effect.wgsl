// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { drift: f32 }

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = clamp(progress, 0.0, 1.0);
    let e = p * p * (3.0 - 2.0 * p);
    let k = params.drift / 100.0 * 0.08;
    let d = uv - vec2<f32>(0.5);
    let a = from_at(vec2<f32>(0.5) + d / (1.0 + k * e));
    let b = to_at(vec2<f32>(0.5) + d / (1.0 + k * (1.0 - e)));
    let rgb = from_display(mix(to_display(a.rgb), to_display(b.rgb), e));
    return vec4<f32>(rgb, mix(a.a, b.a, e));
}
