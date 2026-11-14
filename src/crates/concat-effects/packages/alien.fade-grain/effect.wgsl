// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { size: f32, soft: f32 }

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = clamp(progress, 0.0, 1.0);
    let a = from_at(uv);
    let b = to_at(uv);
    let cell = floor(uv * frame.size / params.size);
    let n = hash(cell, 1.0) * 0.6 + hash(floor(cell / 4.0), 2.0) * 0.4;
    let s = params.soft / 100.0 * 0.4 + 0.02;
    let front = mix(-s, 1.0 + s, p);
    let w = smoothstep(n - s, n + s, front);
    return vec4<f32>(mix(a.rgb, b.rgb, w), mix(a.a, b.a, w));
}
