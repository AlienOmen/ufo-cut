// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, contrast: f32, mid: f32, whites: f32 }

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
    var l = clamp(luma(to_display(c.rgb)), 0.0, 1.0);
    let m = params.mid / 100.0;
    l = clamp(l * 0.5 / m, 0.0, 1.0);
    let k = params.contrast / 100.0;
    l = mix(l, l * l * (3.0 - 2.0 * l), k);
    let shadow = vec3<f32>(0.0035, 0.0045, 0.0035);
    let hi = mix(ACID, vec3<f32>(1.0, 1.0, 0.85), params.whites / 100.0 * smoothstep(0.7, 1.0, l));
    let duo = mix(shadow, hi, pow(l, 1.4));
    return vec4<f32>(mix(c.rgb, duo, params.amount / 100.0), c.a);
}
