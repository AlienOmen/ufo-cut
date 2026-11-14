// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, blocks: f32, speed: f32, split: f32 }

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
    let amt = params.amount / 100.0;
    let tick = floor(frame.time * params.speed);
    let band = floor(uv.y * params.blocks);
    let col = floor(uv.x * 6.0);
    let h = hash(vec2<f32>(band, tick), 3.0);
    let on = h > 1.0 - amt * 0.55;
    let h2 = hash(vec2<f32>(band + col * 37.0, tick), 9.0);
    var shift = 0.0;
    if (on) {
        shift = (hash(vec2<f32>(band, tick), 7.0) - 0.5) * 0.35 * amt;
        if (h2 > 0.7) { shift = shift * -1.6; }
    }
    let p = vec2<f32>(uv.x + shift, uv.y);
    let sp = select(0.0, params.split / 100.0 * 0.03 * (0.5 + h), on);
    let g = sample(p);
    let r = sample(p + vec2<f32>(sp, 0.0));
    let b = sample(p - vec2<f32>(sp, 0.0));
    var rgb = vec3<f32>(r.r, g.g, b.b);
    if (on && h2 < 0.08) { rgb = ACID * 0.9 * luma(g.rgb) * 3.0; }
    return vec4<f32>(rgb, g.a);
}
