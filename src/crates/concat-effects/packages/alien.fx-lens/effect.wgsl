// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, zoom: f32 }

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
    let asp = frame.size.x / frame.size.y;
    var d = (uv - vec2<f32>(0.5)) * vec2<f32>(asp, 1.0);
    let k = params.amount / 100.0 * 0.6;
    let r2 = dot(d, d);
    let s = max(1.0 + k * r2, 0.2);
    let fix = 1.0 / (1.0 + max(k, 0.0) * 0.35 * params.zoom / 100.0 * 2.0) ;
    d = d * s * mix(1.0, fix, params.zoom / 100.0);
    let p = d / vec2<f32>(asp, 1.0) + vec2<f32>(0.5);
    return sample(fold(p));
}
