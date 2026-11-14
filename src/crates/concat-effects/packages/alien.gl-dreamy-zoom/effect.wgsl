// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/DreamyZoom.glsl
// Author: Zeh Fernando
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let ratio = gl_ratio();
  let rotation = 6.0;
  let scale = 1.2;
  let DEG2RAD = 0.017453292519943295;
  let first = progress < 0.5;
  var phase = (progress - 0.5) * 2.0;
  if (first) { phase = progress * 2.0; }
  var angle_offset = mix(-rotation * DEG2RAD, 0.0, phase);
  var new_scale = mix(scale, 1.0, phase);
  if (first) {
    angle_offset = mix(0.0, rotation * DEG2RAD, phase);
    new_scale = mix(1.0, scale, phase);
  }
  var p = (uv - vec2<f32>(0.5)) / new_scale * vec2<f32>(ratio, 1.0);
  let angle = atan2(p.y, p.x) + angle_offset;
  let dist = length(p);
  p = vec2<f32>(cos(angle) * dist / ratio + 0.5, sin(angle) * dist + 0.5);
  var c = getToColor(p);
  var flash = mix(1.0, 0.0, phase);
  if (first) {
    c = getFromColor(p);
    flash = mix(0.0, 1.0, phase);
  }
  return vec4<f32>(c.rgb + vec3<f32>(flash), c.a);
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
