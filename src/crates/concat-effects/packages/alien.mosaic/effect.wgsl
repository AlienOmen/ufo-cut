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
    let coarse = bell(p);
    // Tiles across the frame's height: fine at the ends, a dozen in the middle.
    let count = mix(frame.size.y, 12.0, pow(coarse, 0.6));
    let cells = vec2<f32>(count * frame.size.x / max(frame.size.y, 1.0), count);
    let at = (floor(uv * cells) + vec2<f32>(0.5)) / cells;
    let w = smoothstep(0.4, 0.6, p);
    return mix(from_at(at), to_at(at), w);
}
