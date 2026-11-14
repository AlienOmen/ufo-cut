// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, centre: f32 }

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
    let d = uv - vec2<f32>(0.5);
    let edge = smoothstep(params.centre / 100.0 * 0.7, 0.75, length(d * vec2<f32>(1.0, frame.size.y / frame.size.x)) * 1.6);
    let scale = params.amount / (0.5 * frame.size.x) * (0.25 + 0.75 * edge);
    var acc = vec3<f32>(0.0);
    var w = vec3<f32>(0.0);
    for (var i = 0; i < 7; i++) {
        let t = f32(i) / 6.0;
        let s = sample(vec2<f32>(0.5) + d * (1.0 + (t - 0.5) * 2.0 * scale)).rgb;
        let wt = vec3<f32>(1.0 - t, 1.0 - abs(2.0 * t - 1.0), t);
        acc += s * wt;
        w += wt;
    }
    return vec4<f32>(acc / w, c.a);
}
