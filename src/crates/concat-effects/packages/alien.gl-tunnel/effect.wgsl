// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Tunnel.glsl
// Author: gre
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_rotate(v: vec2<f32>, a: f32) -> vec2<f32> {
  let c = cos(a);
  let s = sin(a);
  return vec2<f32>(c * v.x - s * v.y, s * v.x + c * v.y);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let speed = 6.0;
  let twist = 0.5;
  let ratio = gl_ratio();
  let aspect = vec2<f32>(ratio, 1.0);
  let c = (uv - vec2<f32>(0.5)) * aspect;
  let r = length(c);
  let R = length(0.5 * aspect);
  let far = 1.0 + speed;
  let e = mix(progress, progress * progress * (3.0 - 2.0 * progress), 0.7);
  let d = pow(far, 1.0 - e);
  let t = far - d;
  var k = R / (R + t * r);
  if (r * d < R) { k = d / far; }
  let a = twist * min(t * r / R, t / d);
  var walls = getFromColor(vec2<f32>(0.5) + gl_rotate(c * k, a) / aspect);
  walls = vec4<f32>(walls.rgb * (0.4 + 0.6 * sqrt(k)), walls.a);
  let opening = smoothstep(0.0, 0.6, progress);
  let q = vec2<f32>(0.5) + (uv - vec2<f32>(0.5)) * d / max(opening, 1e-4);
  let inside = step(vec2<f32>(0.0), q) * step(q, vec2<f32>(1.0));
  return mix(walls, getToColor(q), inside.x * inside.y * step(r * d, R));
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
