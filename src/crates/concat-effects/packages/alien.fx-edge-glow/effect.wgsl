// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { strength: f32, width: f32, dim: f32, white: f32 }

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

fn lum_at(uv: vec2<f32>) -> f32 {
    return luma(to_display(sample(uv).rgb));
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let c = sample(uv);
    let t = texel() * params.width;
    let tl = lum_at(uv + vec2<f32>(-t.x, -t.y));
    let tc = lum_at(uv + vec2<f32>(0.0, -t.y));
    let tr = lum_at(uv + vec2<f32>(t.x, -t.y));
    let ml = lum_at(uv + vec2<f32>(-t.x, 0.0));
    let mr = lum_at(uv + vec2<f32>(t.x, 0.0));
    let bl = lum_at(uv + vec2<f32>(-t.x, t.y));
    let bc = lum_at(uv + vec2<f32>(0.0, t.y));
    let br = lum_at(uv + vec2<f32>(t.x, t.y));
    let gx = (tr + 2.0 * mr + br) - (tl + 2.0 * ml + bl);
    let gy = (bl + 2.0 * bc + br) - (tl + 2.0 * tc + tr);
    let e = clamp(sqrt(gx * gx + gy * gy) * params.strength / 100.0 * 1.6, 0.0, 1.5);
    let line = mix(ACID, vec3<f32>(1.0), params.white / 100.0 * clamp(e - 0.6, 0.0, 1.0));
    return vec4<f32>(c.rgb * (1.0 - params.dim / 100.0) + line * e * 1.4, c.a);
}
