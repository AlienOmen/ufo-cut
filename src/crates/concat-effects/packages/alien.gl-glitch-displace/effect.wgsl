// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/GlitchDisplace.glsl
// Author: Matt DesLauriers
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_random(co: vec2<f32>) -> f32 {
  let dt = dot(co, vec2<f32>(12.9898, 78.233));
  let sn = gl_mod(dt, 3.14);
  return fract(sin(sn) * 43758.5453);
}
fn gl_voronoi(x: vec2<f32>) -> f32 {
  let p = floor(x);
  let f = fract(x);
  var res = 8.0;
  for (var j = -1; j <= 1; j = j + 1) {
    for (var i = -1; i <= 1; i = i + 1) {
      let b = vec2<f32>(f32(i), f32(j));
      let r = b - f + gl_random(p + b);
      res = min(res, dot(r, r));
    }
  }
  return sqrt(res);
}
fn gl_displace(tex: vec4<f32>, tc: vec2<f32>, dot_depth: f32, tex_depth: f32, strength: f32) -> vec2<f32> {
  let dis = tex * dot_depth + vec4<f32>(1.0) - tex * tex_depth;
  let dx = (dis.x - 1.0 + tex_depth * dot_depth) * strength;
  let dy = (dis.y - 1.0 + tex_depth * dot_depth) * strength;
  return vec2<f32>(tc.x + dx, tc.y + dy);
}
fn gl_ease1(t: f32) -> f32 {
  if (t == 0.0 || t == 1.0) { return t; }
  if (t < 0.5) { return 0.5 * pow(2.0, 20.0 * t - 10.0); }
  return -0.5 * pow(2.0, 10.0 - t * 20.0) + 1.0;
}
fn gl_ease2(t: f32) -> f32 {
  if (t == 1.0) { return t; }
  return 1.0 - pow(2.0, -10.0 * t);
}
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  var color1 = getFromColor(p);
  var color2 = getToColor(p);
  let disp = gl_displace(color1, p, 0.33, 0.7, 1.0 - gl_ease1(progress));
  let disp2 = gl_displace(color2, p, 0.33, 0.5, gl_ease2(progress));
  let d_color1 = getToColor(disp);
  var d_color2 = getFromColor(disp2);
  let val = gl_ease1(progress);
  let gray = vec3<f32>(dot(min(d_color2, d_color1).rgb, vec3<f32>(0.299, 0.587, 0.114)));
  d_color2 = vec4<f32>(gray * 2.0, d_color2.a);
  color1 = mix(color1, d_color2, smoothstep(0.0, 0.5, progress));
  color2 = mix(color2, d_color1, smoothstep(1.0, 0.5, progress));
  return mix(color1, color2, val);
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
