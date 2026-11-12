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
    let strips = 7.0;
    let strip = floor(uv.y * strips);
    // Odd strips leave to the left and even ones to the right, the top first.
    let sense = select(1.0, -1.0, fract(strip * 0.5) > 0.25);
    let late = strip / strips * 0.35;
    let gone = clamp((p - late) / 0.65, 0.0, 1.0);
    let at = vec2<f32>(uv.x - sense * gone * 1.05, uv.y);
    let still_here = step(0.0, at.x) * step(at.x, 1.0);
    return mix(to_at(uv), from_at(clamp(at, vec2<f32>(0.0), vec2<f32>(1.0))), still_here);
}
