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
    let p = ease(progress);
    let lines = 36.0;
    let row = floor(uv.y * lines) / lines;
    // Each row switches a little after the one above it.
    let front = mix(-0.25, 1.25, p);
    let w = smoothstep(row - 0.02, row + 0.02, front - 0.12 * fract(uv.y * lines));
    let base = mix(from_at(uv), to_at(uv), w);
    let edge = exp(-pow((uv.y - front) / 0.03, 2.0)) * bell(progress);
    return vec4<f32>(base.rgb + ACID * edge * 0.9, base.a);
}
