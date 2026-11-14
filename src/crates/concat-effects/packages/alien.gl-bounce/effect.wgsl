// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Bounce.glsl
// Author: Adrian Purser
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let shadow_colour = vec4<f32>(0.0, 0.0, 0.0, 0.6);
  let shadow_height = 0.075;
  let bounces = 3.0;
  let stime = sin(progress * GL_PI / 2.0);
  let phase = progress * GL_PI * bounces;
  let y = abs(cos(phase)) * (1.0 - stime);
  let d = uv.y - y;
  let to = getToColor(uv);
  let fade = mix(((d / shadow_height) * shadow_colour.a) + (1.0 - shadow_colour.a), 1.0, smoothstep(0.95, 1.0, progress));
  let amount = step(d, shadow_height) * (1.0 - fade) * to.a;
  let under = mix(to, vec4<f32>(shadow_colour.rgb, 1.0), amount);
  return mix(under, getFromColor(vec2<f32>(uv.x, uv.y + (1.0 - y))), step(d, 0.0));
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
