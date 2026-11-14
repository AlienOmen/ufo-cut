// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
// Original code written for UFO Cut; no third-party shader code.

struct Params { size: f32, angle: f32, colour: f32, paper: f32, soft: f32 }

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
    let a = radians(params.angle);
    let cs = vec2<f32>(cos(a), sin(a));
    let px = uv * frame.size;
    let rp = vec2<f32>(cs.x * px.x + cs.y * px.y, -cs.y * px.x + cs.x * px.y) / params.size;
    let cell = floor(rp) + vec2<f32>(0.5);
    let centre_rot = cell * params.size;
    let centre = vec2<f32>(cs.x * centre_rot.x - cs.y * centre_rot.y, cs.y * centre_rot.x + cs.x * centre_rot.y) / frame.size;
    let s = sample(clamp(centre, vec2<f32>(0.0), vec2<f32>(1.0)));
    let l = clamp(luma(to_display(s.rgb)), 0.0, 1.0);
    let radius = sqrt(l) * 0.72;
    let dist = length(rp - cell);
    let soft = params.soft / 100.0 * 0.35 + 0.03;
    let m = 1.0 - smoothstep(radius - soft, radius + soft, dist);
    let paper = params.paper > 0.5;
    let mono_ink = select(ACID, vec3<f32>(0.01), paper);
    let ink = mix(mono_ink, s.rgb, params.colour / 100.0);
    let ground = select(VOID, vec3<f32>(0.85), paper);
    return vec4<f32>(mix(ground, ink, m), c.a);
}
