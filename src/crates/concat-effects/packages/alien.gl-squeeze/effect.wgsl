// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Squeeze.glsl
// Author: gre
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let color_separation = 0.04;
  let y = 0.5 + (uv.y - 0.5) / max(1.0 - progress, 1e-4);
  if (y < 0.0 || y > 1.0) { return getToColor(uv); }
  let fp = vec2<f32>(uv.x, y);
  let off = progress * vec2<f32>(0.0, color_separation);
  let c = getFromColor(fp);
  let cn = getFromColor(fp - off);
  let cp = getFromColor(fp + off);
  return vec4<f32>(cn.r, c.g, cp.b, c.a);
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
