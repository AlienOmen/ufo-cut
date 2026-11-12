// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

// A.L.I.E.N. acid (#C8FF00) and void (#0A0B0A), in linear light.
const ACID: vec3<f32> = vec3<f32>(0.578, 1.0, 0.0);
const VOID: vec3<f32> = vec3<f32>(0.003, 0.0033, 0.003);

fn ease(p: f32) -> f32 {
    let q = clamp(p, 0.0, 1.0);
    return q * q * (3.0 - 2.0 * q);
}

fn bell(p: f32) -> f32 {
    return sin(3.14159265 * clamp(p, 0.0, 1.0));
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = clamp(progress, 0.0, 1.0);
    let bars = 10.0;
    let across = abs(fract(uv.x * bars) - 0.5) * 2.0;
    // How far the bars have closed: shut at the middle of the cut.
    let shut = bell(p);
    let dark = smoothstep(1.0 - shut - 0.03, 1.0 - shut + 0.03, 1.0 - across);
    let base = mix(from_at(uv), to_at(uv), step(0.5, p));
    let edge = dark * (1.0 - dark) * 4.0;
    return vec4<f32>(mix(base.rgb, VOID, dark) + ACID * edge * 0.5, base.a);
}
