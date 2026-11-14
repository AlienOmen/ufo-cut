// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { bpm: f32, punch: f32, decay: f32, split: f32 }

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
    let phase = fract(frame.time * params.bpm / 60.0);
    let hit = exp(-phase * params.decay);
    let z = 1.0 + params.punch / 100.0 * hit;
    let d = uv - vec2<f32>(0.5);
    let p = fold(vec2<f32>(0.5) + d / z);
    let s = params.split / 100.0 * hit * 0.012;
    let c = sample(p);
    let r = sample(fold(vec2<f32>(0.5) + d / (z + s * 4.0)));
    let b = sample(fold(vec2<f32>(0.5) + d / max(z - s * 4.0, 0.2)));
    return vec4<f32>(r.r, c.g, b.b, c.a);
}
