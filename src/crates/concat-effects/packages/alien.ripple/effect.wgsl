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
    let strength = bell(p) * 0.035;
    let wave = vec2<f32>(sin(uv.y * 38.0 + p * 18.0), cos(uv.x * 30.0 - p * 14.0)) * strength;
    let at = clamp(uv + wave, vec2<f32>(0.0), vec2<f32>(1.0));
    return mix(from_at(at), to_at(at), ease(p));
}
