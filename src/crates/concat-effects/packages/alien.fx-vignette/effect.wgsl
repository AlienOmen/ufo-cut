// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, size: f32, softness: f32, round: f32, acid: f32 }

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
    let asp = frame.size.x / frame.size.y;
    var d = (uv - vec2<f32>(0.5)) * 2.0;
    let rd = vec2<f32>(d.x * mix(1.0, asp, params.round / 100.0), d.y);
    let dist = length(rd) / mix(1.0, asp, params.round / 100.0) * 1.0;
    let edge = params.size / 100.0 * 1.0;
    let m = smoothstep(edge, edge + params.softness / 100.0 * 1.2 + 0.02, dist) * params.amount / 100.0;
    let tint = mix(vec3<f32>(0.0), ACID * 0.25, params.acid / 100.0);
    return vec4<f32>(mix(c.rgb, tint, clamp(m, 0.0, 1.0)), c.a);
}
