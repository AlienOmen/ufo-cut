// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { levels: f32, soft: f32, sat: f32 }

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
    var e = clamp(to_display(c.rgb), vec3<f32>(0.0), vec3<f32>(1.0));
    let l = luma(e);
    e = vec3<f32>(l) + (e - vec3<f32>(l)) * params.sat / 100.0;
    let n = params.levels - 1.0;
    let x = clamp(e, vec3<f32>(0.0), vec3<f32>(1.0)) * n;
    let f = fract(x);
    let w = params.soft / 100.0 * 0.5 + 0.0001;
    let q = (floor(x) + smoothstep(vec3<f32>(0.5 - w), vec3<f32>(0.5 + w), f)) / n;
    return vec4<f32>(from_display(q), c.a);
}
