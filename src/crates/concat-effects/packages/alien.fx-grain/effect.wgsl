// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, size: f32, colour: f32, shadows: f32 }

const ACID: vec3<f32> = vec3<f32>(0.578, 1.0, 0.0);
const VOID: vec3<f32> = vec3<f32>(0.003, 0.0033, 0.003);

fn turn_hue(c: vec3<f32>, a: f32) -> vec3<f32> {
    let k = vec3<f32>(0.57735);
    let ca = cos(a);
    return c * ca + cross(k, c) * sin(a) + k * dot(k, c) * (1.0 - ca);
}

fn fold(uv: vec2<f32>) -> vec2<f32> {
    let m = abs(uv - 2.0 * floor(uv * 0.5 + vec2<f32>(0.5)));
    return clamp(m, vec2<f32>(0.0), vec2<f32>(1.0));
}

fn smooth_noise(t: f32, seed: f32) -> f32 {
    let i = floor(t);
    let f = t - i;
    let a = hash(vec2<f32>(i, seed), 5.0) * 2.0 - 1.0;
    let b = hash(vec2<f32>(i + 1.0, seed), 5.0) * 2.0 - 1.0;
    return mix(a, b, f * f * f * (f * (f * 6.0 - 15.0) + 10.0));
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let c = sample(uv);
    let cell = floor(uv * frame.size / params.size);
    let seed = floor(frame.time * 24.0);
    let n = hash(cell, seed) - 0.5;
    let nr = hash(cell + vec2<f32>(17.0, 3.0), seed) - 0.5;
    let ng = hash(cell + vec2<f32>(5.0, 29.0), seed) - 0.5;
    let nb = hash(cell + vec2<f32>(41.0, 11.0), seed) - 0.5;
    let col = vec3<f32>(nr, ng, nb) * params.colour / 100.0 + vec3<f32>(n) * (1.0 - params.colour / 100.0 * 0.5);
    let l = clamp(luma(to_display(c.rgb)), 0.0, 1.0);
    let resp = mix(1.0, 4.0 * l * (1.0 - l) + 0.15, params.shadows / 100.0);
    return vec4<f32>(from_log(to_log(c.rgb) + col * params.amount / 100.0 * 0.45 * resp), c.a);
}
