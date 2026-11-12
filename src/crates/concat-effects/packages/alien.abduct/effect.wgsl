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
    let size = max(1.0 - p, 0.0001);
    // The outgoing shot, shrinking towards the top of the frame.
    let centre = vec2<f32>(0.5, mix(0.5, -0.1, p));
    let at = (uv - centre) / size + vec2<f32>(0.5);
    let inside = step(0.0, at.x) * step(at.x, 1.0) * step(0.0, at.y) * step(at.y, 1.0);
    let under = to_at(uv);
    let lifted = from_at(clamp(at, vec2<f32>(0.0), vec2<f32>(1.0)));
    // The light it is lifted in.
    let shaft = exp(-pow((uv.x - 0.5) / (0.08 + 0.3 * size), 2.0)) * bell(progress) * 0.35;
    let mixed = mix(under, lifted, inside * (1.0 - p * p));
    return vec4<f32>(mixed.rgb + ACID * shaft * (1.0 - inside), mixed.a);
}
