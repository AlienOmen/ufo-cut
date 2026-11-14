// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { wobble: f32, bleed: f32, noise: f32, band: f32 }

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
    let tick = floor(frame.time * 30.0);
    let row = floor(uv.y * frame.size.y * 0.5);
    var dx = (hash(vec2<f32>(row, tick), 1.0) - 0.5) * 0.004 * params.wobble / 100.0;
    let by = fract(frame.time * 0.17 + 0.35);
    let near = exp(-pow((uv.y - by) / 0.05, 2.0));
    dx += near * params.band / 100.0 * (0.03 + 0.02 * sin(uv.y * 120.0 + frame.time * 20.0));
    let bl = params.bleed / 100.0 * 6.0 / frame.size.x;
    let p = vec2<f32>(uv.x + dx, uv.y);
    let c = sample(p);
    var acc = vec3<f32>(0.0);
    for (var i = -3; i <= 3; i++) {
        acc += sample(p + vec2<f32>(f32(i) * bl, 0.0)).rgb;
    }
    acc = acc / 7.0;
    let y = luma(c.rgb);
    var rgb = (c.rgb - vec3<f32>(y)) * 0.0 + vec3<f32>(y) + (acc - vec3<f32>(luma(acc))) * 1.05;
    rgb = mix(rgb, c.rgb, 0.35);
    let n = hash(floor(uv * frame.size * vec2<f32>(0.5, 1.0)), tick) - 0.5;
    rgb += vec3<f32>(n) * params.noise / 100.0 * 0.18;
    let streak = step(0.995, hash(vec2<f32>(row, tick), 8.0)) * params.noise / 100.0;
    rgb += vec3<f32>(streak * 0.5);
    rgb = rgb + near * params.band / 100.0 * 0.04;
    return vec4<f32>(rgb, c.a);
}
