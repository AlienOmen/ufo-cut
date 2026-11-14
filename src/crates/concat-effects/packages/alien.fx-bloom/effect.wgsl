// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { amount: f32, threshold: f32, radius: f32 }

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
    let thr = params.threshold / 100.0;
    var sum = vec3<f32>(0.0);
    var w = 0.0;
    for (var ring = 1; ring <= 3; ring++) {
        let rr = f32(ring) / 3.0;
        for (var k = 0; k < 12; k++) {
            let a = (f32(k) + f32(ring) * 0.5) / 12.0 * 6.2831853;
            let o = vec2<f32>(cos(a), sin(a)) * rr * params.radius / frame.size;
            let s = sample(uv + o).rgb;
            let b = max(s - vec3<f32>(thr), vec3<f32>(0.0));
            let wt = exp(-rr * rr * 1.8);
            sum += b * wt;
            w += wt;
        }
    }
    let own = max(c.rgb - vec3<f32>(thr), vec3<f32>(0.0));
    sum = (sum + own * 1.5) / (w + 1.5);
    return vec4<f32>(c.rgb + sum * params.amount / 100.0 * 2.2, c.a);
}
