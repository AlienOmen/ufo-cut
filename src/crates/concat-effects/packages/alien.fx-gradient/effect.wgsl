// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, speed: f32, angle: f32, hue: f32, mode: f32 }

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

fn palette(g: f32) -> vec3<f32> {
    let a = vec3<f32>(0.578, 1.0, 0.0);
    let b = vec3<f32>(1.0, 0.02, 0.2);
    let c = vec3<f32>(0.0, 0.55, 1.0);
    let x = fract(g) * 3.0;
    let s0 = smoothstep(0.0, 1.0, x);
    let s1 = smoothstep(1.0, 2.0, x);
    let s2 = smoothstep(2.0, 3.0, x);
    return mix(mix(mix(a, b, s0), c, s1), a, s2);
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let c = sample(uv);
    let a = radians(params.angle);
    let d = vec2<f32>(cos(a), sin(a));
    let asp = vec2<f32>(frame.size.x / frame.size.y, 1.0);
    let t = frame.time * params.speed / 100.0 * 0.25;
    let g = dot((uv - vec2<f32>(0.5)) * asp, d) * 0.55 + 0.33 + t + 0.08 * sin(uv.y * 5.0 + t * 8.0);
    let col = turn_hue(palette(g), radians(params.hue));
    let k = params.amount / 100.0;
    let mode = i32(round(params.mode));
    var rgb = c.rgb;
    if (mode == 0) {
        rgb = mix(c.rgb, c.rgb * col * 2.2 + col * 0.04, k);
    } else if (mode == 1) {
        rgb = c.rgb + col * k * 0.6;
    } else {
        rgb = mix(c.rgb, col * (0.3 + luma(c.rgb) * 1.4), k);
    }
    return vec4<f32>(rgb, c.a);
}
