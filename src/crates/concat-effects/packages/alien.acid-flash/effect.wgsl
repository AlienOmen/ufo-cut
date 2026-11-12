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
    let p = clamp(progress, 0.0, 1.0);
    let base = mix(from_at(uv), to_at(uv), smoothstep(0.42, 0.58, p));
    let flash = pow(bell(p), 2.0);
    let burnt = mix(base.rgb, ACID * 1.3, flash);
    return vec4<f32>(burnt, base.a);
}
