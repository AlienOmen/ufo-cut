// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { curve: f32, lines: f32, mask: f32, flicker: f32 }

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
    var d = uv * 2.0 - vec2<f32>(1.0);
    let k = params.curve / 100.0 * 0.18;
    d = d * (vec2<f32>(1.0) + vec2<f32>(d.y * d.y, d.x * d.x) * k * 2.0);
    let p = d * 0.5 + vec2<f32>(0.5);
    if (p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    var rgb = sample(p).rgb;
    var e = to_display(rgb);
    let row = p.y * frame.size.y;
    let scan = 0.5 + 0.5 * cos(row * 3.14159265 * 2.0 / max(frame.size.y / 270.0, 1.0));
    e = e * (1.0 - params.lines / 100.0 * 0.6 * scan);
    let px = floor(p.x * frame.size.x);
    let m = i32(px) % 3;
    var tri = vec3<f32>(1.0 - params.mask / 100.0 * 0.5);
    if (m == 0) { tri.r = 1.0; } else if (m == 1) { tri.g = 1.0; } else { tri.b = 1.0; }
    e = e * tri * (1.0 + params.mask / 100.0 * 0.25);
    let vg = 1.0 - smoothstep(0.55, 1.15, length(d)) * 0.7;
    let fl = 1.0 - params.flicker / 100.0 * 0.08 * (0.5 + 0.5 * sin(frame.time * 37.0));
    return vec4<f32>(from_display(e * vg * fl), 1.0);
}
