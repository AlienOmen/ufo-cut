// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { glow: f32 }

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = clamp(progress, 0.0, 1.0);
    let a = from_at(uv);
    let b = to_at(uv);
    let wa = min(1.0, 2.0 * (1.0 - p));
    let wb = min(1.0, 2.0 * p);
    let plain = mix(a.rgb, b.rgb, p);
    let add = a.rgb * wa + b.rgb * wb;
    let rgb = mix(plain, add, params.glow / 100.0);
    return vec4<f32>(rgb, mix(a.a, b.a, p));
}
