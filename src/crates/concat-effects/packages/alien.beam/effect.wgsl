// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

const ACID: vec3<f32> = vec3<f32>(0.784, 1.0, 0.0);

fn ease(p: f32) -> f32 {
    let q = clamp(p, 0.0, 1.0);
    return q * q * (3.0 - 2.0 * q);
}

fn bell(p: f32) -> f32 {
    return sin(3.14159265 * clamp(p, 0.0, 1.0));
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = ease(progress);
    let width = 0.06;
    let line = mix(-width, 1.0 + width, p);
    let side = step(uv.x, line);
    let base = mix(from_at(uv), to_at(uv), side);
    let glow = exp(-pow((uv.x - line) / width, 2.0) * 3.0) * bell(progress);
    return vec4<f32>(base.rgb + ACID * glow * 1.4, base.a);
}
