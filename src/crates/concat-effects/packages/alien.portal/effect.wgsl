// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

// A.L.I.E.N. acid (#C8FF00) and void (#0A0B0A), in linear light.
const ACID: vec3<f32> = vec3<f32>(0.578, 1.0, 0.0);
const VOID: vec3<f32> = vec3<f32>(0.003, 0.0033, 0.003);

fn ease(p: f32) -> f32 {
    let q = clamp(p, 0.0, 1.0);
    return q * q * (3.0 - 2.0 * q);
}

fn bell(p: f32) -> f32 {
    return sin(3.14159265 * clamp(p, 0.0, 1.0));
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let p = ease(progress);
    let shape = vec2<f32>(frame.size.x / max(frame.size.y, 1.0), 1.0);
    let off = (uv - vec2<f32>(0.5)) * shape;
    let far = length(off);
    let reach = length(shape) * 0.5 + 0.1;
    let radius = p * reach;
    // The picture bends a little as it nears the ring.
    let pull = exp(-pow((far - radius) / 0.08, 2.0)) * 0.03 * bell(progress);
    let bent = uv + normalize(off + vec2<f32>(0.0001)) * pull / shape;
    let inside = 1.0 - smoothstep(radius - 0.004, radius + 0.004, far);
    let base = mix(from_at(bent), to_at(bent), inside);
    let ring = exp(-pow((far - radius) / 0.018, 2.0)) * bell(progress);
    return vec4<f32>(base.rgb + ACID * ring * 1.6, base.a);
}
