// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { threshold: f32, length: f32, jitter: f32, direction: f32 }

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
    let thr = params.threshold / 100.0 * 1.2;
    let vert = params.direction > 0.5;
    let axis = select(vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vert);
    let lane = select(floor(uv.y * 90.0), floor(uv.x * 160.0), vert);
    let tick = floor(frame.time * 12.0);
    let reach = params.length / 100.0 * 0.45 * (1.0 - params.jitter / 100.0 * 0.8 * hash(vec2<f32>(lane, tick), 4.0));
    var best = c.rgb;
    var best_l = luma(c.rgb);
    for (var i = 1; i <= 24; i++) {
        let t = f32(i) / 24.0;
        let s = sample(uv - axis * t * reach).rgb;
        let l = luma(s) * (1.0 - t * 0.7);
        if (luma(s) > thr && l > best_l) { best = s * (1.0 - t * 0.55); best_l = l; }
    }
    return vec4<f32>(best, c.a);
}
