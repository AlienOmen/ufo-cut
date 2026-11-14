// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { strength: f32, scale: f32, speed: f32, shine: f32, disp: f32 }

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

fn field(p: vec2<f32>, t: f32) -> f32 {
    return sin(p.x * 1.0 + t * 1.1 + sin(p.y * 0.7 + t * 0.6)) * 0.5
         + sin(p.y * 1.3 - t * 0.9 + sin(p.x * 0.8 - t * 0.4) * 1.5) * 0.35
         + sin((p.x + p.y) * 0.6 + t * 0.5) * 0.25;
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let asp = frame.size.x / frame.size.y;
    let p = uv * vec2<f32>(asp, 1.0) * params.scale;
    let t = frame.time * params.speed / 100.0 * 2.0;
    let e = 0.01;
    let h = field(p, t);
    let gx = field(p + vec2<f32>(e, 0.0), t) - h;
    let gy = field(p + vec2<f32>(0.0, e), t) - h;
    let n = vec2<f32>(gx, gy) / e;
    let k = params.strength / 100.0 * 0.09 / max(params.scale * 0.35, 0.6);
    let dsp = params.disp / 100.0 * 0.35;
    let c = sample(fold(uv - n * k));
    let r = sample(fold(uv - n * k * (1.0 + dsp)));
    let b = sample(fold(uv - n * k * (1.0 - dsp)));
    var rgb = vec3<f32>(r.r, c.g, b.b);
    let nn = normalize(vec3<f32>(-n.x * 0.5, -n.y * 0.5, 1.0));
    let light = normalize(vec3<f32>(-0.5, -0.6, 0.65));
    let spec = pow(max(dot(nn, light), 0.0), 40.0);
    let rim = pow(1.0 - nn.z, 2.0);
    rgb += vec3<f32>(spec * 0.9 + rim * 0.25) * params.shine / 100.0;
    return vec4<f32>(rgb, c.a);
}
