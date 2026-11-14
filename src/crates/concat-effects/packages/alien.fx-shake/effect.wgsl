// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, speed: f32, tilt: f32, blur: f32 }

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

fn cam(uv: vec2<f32>, dt: f32) -> vec2<f32> {
    let amt = params.amount / 100.0;
    let rate = 2.0 + 14.0 * params.speed / 100.0;
    let t = (frame.time + dt) * rate;
    let off = vec2<f32>(smooth_noise(t, 1.0) * 0.8 + smooth_noise(t * 2.7, 2.0) * 0.2,
                        smooth_noise(t, 3.0) * 0.8 + smooth_noise(t * 2.7, 4.0) * 0.2) * 0.06 * amt;
    let ang = smooth_noise(t * 0.9, 5.0) * 0.08 * amt * params.tilt / 50.0;
    let zoom = 1.0 + 0.06 * amt + abs(ang) * 1.2;
    let d = (uv - vec2<f32>(0.5)) * frame.size;
    let cs = vec2<f32>(cos(ang), sin(ang));
    let r = vec2<f32>(cs.x * d.x + cs.y * d.y, cs.x * d.y - cs.y * d.x) / zoom;
    return fold(r / frame.size + vec2<f32>(0.5) - off);
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let open = params.blur / 100.0 / 20.0;
    var acc = vec4<f32>(0.0);
    for (var k = 0; k < 6; k++) {
        acc += sample_premultiplied(cam(uv, (f32(k) / 5.0 - 0.5) * open));
    }
    let m = acc / 6.0;
    return vec4<f32>(m.rgb / max(m.a, 1e-6), m.a);
}
