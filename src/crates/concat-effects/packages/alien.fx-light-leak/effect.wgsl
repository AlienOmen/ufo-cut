// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, speed: f32, hue: f32, softness: f32 }

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
    let t = frame.time * params.speed / 100.0 * 0.8;
    let a1 = vec2<f32>(0.15 + 0.35 * sin(t * 1.3), 0.5 + 0.45 * cos(t * 0.9 + 1.0));
    let a2 = vec2<f32>(0.85 + 0.2 * sin(t * 0.7 + 2.0), 0.2 + 0.5 * sin(t * 1.1));
    let a3 = vec2<f32>(0.5 + 0.6 * sin(t * 0.5 + 4.0), 0.95 + 0.2 * cos(t * 1.7));
    let asp = vec2<f32>(frame.size.x / frame.size.y, 1.0);
    let wide = mix(0.12, 0.5, params.softness / 100.0);
    let g1 = exp(-dot((uv - a1) * asp, (uv - a1) * asp) / wide);
    let g2 = exp(-dot((uv - a2) * asp, (uv - a2) * asp) / (wide * 0.8));
    let g3 = exp(-dot((uv - a3) * asp, (uv - a3) * asp) / (wide * 1.4));
    let hue = radians(params.hue);
    let orange = turn_hue(vec3<f32>(1.0, 0.22, 0.02), hue);
    let red = turn_hue(vec3<f32>(1.0, 0.03, 0.08), hue);
    let gold = turn_hue(vec3<f32>(1.0, 0.55, 0.08), hue);
    let leak = orange * g1 * 1.1 + red * g2 + gold * g3 * 0.8;
    let pulse = 0.8 + 0.2 * sin(t * 6.0);
    return vec4<f32>(c.rgb + leak * pulse * params.amount / 100.0 * 1.3, c.a);
}
