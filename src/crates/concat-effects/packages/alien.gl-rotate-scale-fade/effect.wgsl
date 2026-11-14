// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/RotateScaleFade.glsl
// Author: Fernando Kuteken
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let center = vec2<f32>(0.5, 0.5);
  let rotations = 1.0;
  let scale = 8.0;
  let back_color = vec4<f32>(0.02, 0.02, 0.02, 1.0);
  let difference = uv - center;
  let dir = normalize(difference + vec2<f32>(1e-6));
  let dist = length(difference);
  let angle = 2.0 * GL_PI * rotations * progress;
  let c = cos(angle);
  let s = sin(angle);
  let current_scale = mix(scale, 1.0, 2.0 * abs(progress - 0.5));
  let rotated_dir = vec2<f32>(dir.x * c - dir.y * s, dir.x * s + dir.y * c);
  let rotated_uv = center + rotated_dir * dist / current_scale;
  if (rotated_uv.x < 0.0 || rotated_uv.x > 1.0 || rotated_uv.y < 0.0 || rotated_uv.y > 1.0) {
    return back_color;
  }
  return mix(getFromColor(rotated_uv), getToColor(rotated_uv), progress);
}

const GL_PI: f32 = 3.141592653589793;
fn getFromColor(p: vec2<f32>) -> vec4<f32> { return from_at(vec2<f32>(p.x, 1.0 - p.y)); }
fn getToColor(p: vec2<f32>) -> vec4<f32> { return to_at(vec2<f32>(p.x, 1.0 - p.y)); }
fn gl_ratio() -> f32 { return frame.size.x / max(frame.size.y, 1.0); }
fn gl_mod(x: f32, y: f32) -> f32 { return x - y * floor(x / y); }
fn gl_mod2(x: vec2<f32>, y: f32) -> vec2<f32> { return x - y * floor(x / y); }
fn gl_rand(co: vec2<f32>) -> f32 { return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453); }
// gl-transitions draw with y pointing up; this frame has y pointing down.
fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    return gl_transition(vec2<f32>(uv.x, 1.0 - uv.y), clamp(progress, 0.0, 1.0));
}
