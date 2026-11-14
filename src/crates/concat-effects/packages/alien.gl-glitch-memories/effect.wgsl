// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/GlitchMemories.glsl
// Author: Gunnar Roth
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  // The block grid is in frame pixels, 16 to a block, as in the original.
  let block = floor(p * frame.size / vec2<f32>(16.0));
  var uv_noise = block / vec2<f32>(64.0);
  uv_noise = uv_noise + floor(vec2<f32>(progress) * vec2<f32>(1200.0, 3500.0)) / vec2<f32>(64.0);
  var dist = vec2<f32>(0.0);
  if (progress > 0.0) { dist = (fract(uv_noise) - vec2<f32>(0.5)) * 0.3 * (1.0 - progress); }
  let red = p + dist * 0.2;
  let green = p + dist * 0.3;
  let blue = p + dist * 0.5;
  let center = mix(getFromColor(green), getToColor(green), progress);
  return vec4<f32>(mix(getFromColor(red), getToColor(red), progress).r, center.g, mix(getFromColor(blue), getToColor(blue), progress).b, center.a);
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
