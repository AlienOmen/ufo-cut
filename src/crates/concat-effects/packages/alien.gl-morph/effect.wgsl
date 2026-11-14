// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Morph.glsl
// Author: paniq
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  let strength = 0.1;
  let ca = getFromColor(p);
  let cb = getToColor(p);
  let oa = ((ca.rg + vec2<f32>(ca.b)) * 0.5) * 2.0 - vec2<f32>(1.0);
  let ob = ((cb.rg + vec2<f32>(cb.b)) * 0.5) * 2.0 - vec2<f32>(1.0);
  let oc = mix(oa, ob, 0.5) * strength;
  let w0 = progress;
  let w1 = 1.0 - w0;
  return mix(getFromColor(p + oc * w0), getToColor(p - oc * w1), progress);
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
