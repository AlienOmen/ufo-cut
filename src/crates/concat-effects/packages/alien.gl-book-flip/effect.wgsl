// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/BookFlip.glsl
// Author: hong
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const PERSPECTIVE: f32 = 0.5;
const SHADING: f32 = 0.6;
const CURL: f32 = 0.5;
const EDGE_W: f32 = 0.0015;
const CURL_PART: f32 = 0.45;
const LIGHT: vec2<f32> = vec2<f32>(-0.447214, 0.894427);
fn gl_cross2(a: vec2<f32>, b: vec2<f32>) -> f32 {
  return a.x * b.y - a.y * b.x;
}
fn gl_page_angle(progress: f32) -> f32 {
  let p = clamp(progress, 0.0, 1.0);
  return GL_PI * p * (0.35 + p * (1.95 - 1.3 * p));
}
fn gl_hit_page(eye: vec2<f32>, dir: vec2<f32>, a: f32, k: f32, straight: f32, c: vec2<f32>, w: f32) -> vec3<f32> {
  var best = vec3<f32>(-1.0, 0.0, 0.0);
  let d = vec2<f32>(cos(a), sin(a));
  let den = gl_cross2(d, dir);
  if (abs(den) > 1e-7) {
    let u = gl_cross2(eye, dir) / den;
    let t = gl_cross2(eye, d) / den;
    if (t > 0.0 && u >= 0.0 && u <= straight) { best = vec3<f32>(t, u, a); }
  }
  if (k != 0.0) {
    let r = 1.0 / k;
    let oc = eye - c;
    let qa = dot(dir, dir);
    let qb = dot(oc, dir);
    let disc = qb * qb - qa * (dot(oc, oc) - r * r);
    if (disc >= 0.0) {
      let sq = sqrt(disc);
      for (var i = 0; i < 2; i = i + 1) {
        var root = sq;
        if (i == 0) { root = -sq; }
        let t = (-qb + root) / qa;
        if (t > 0.0 && (best.x < 0.0 || t < best.x)) {
          let v = (oc + t * dir) * k;
          var db = atan2(v.x, -v.y) - a;
          db = db - 2.0 * GL_PI * floor((db + GL_PI) / (2.0 * GL_PI));
          let s = db / k;
          if (s >= 0.0 && s <= w - straight) { best = vec3<f32>(t, straight + s, a + db); }
        }
      }
    }
  }
  return best;
}
fn gl_curl_point(c: vec2<f32>, k: f32, b: f32) -> vec2<f32> {
  return c + vec2<f32>(sin(b), -cos(b)) / k;
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let ratio = gl_ratio();
  let a = gl_page_angle(progress);
  let lift = sin(a);
  let w = 0.5 * ratio;
  let x = uv.x * ratio - w;
  let dist = w * (1.0 + 1.5 / max(PERSPECTIVE, 0.01));
  let eye = vec2<f32>(0.0, dist);
  let dir = vec2<f32>(x, -dist);
  let d = vec2<f32>(cos(a), sin(a));
  var k = -clamp(CURL, 0.0, 1.0) * lift / (CURL_PART * w);
  if (abs(k) * CURL_PART * w <= 1e-4) { k = 0.0; }
  var straight = w;
  var kk = 1.0;
  if (k != 0.0) { straight = (1.0 - CURL_PART) * w; kk = k; }
  let c = straight * d + vec2<f32>(-d.y, d.x) / kk;
  let bend = straight * d;
  var middle = bend;
  var edge = bend;
  if (k != 0.0) {
    middle = gl_curl_point(c, k, a + 0.5 * k * (w - straight));
    edge = gl_curl_point(c, k, a + k * (w - straight));
  }
  let aa = EDGE_W * min(20.0 * lift, 1.0);
  var img = vec4<f32>(0.0);
  var m = 0.0;
  let hit = gl_hit_page(eye, dir, a, k, straight, c, w);
  if (hit.x > 0.0) {
    let t = hit.x;
    let u = hit.y;
    let b = hit.z;
    let pt = eye + t * dir;
    let y = clamp(0.5 + (uv.y - 0.5) * t, 0.0, 1.0);
    let edge_x = edge.x * dist / max(dist - edge.y, 1e-4);
    var limit = 0.5 * w;
    if (k != 0.0) { limit = straight; }
    m = 1.0;
    if (aa > 0.0 && u > limit) { m = clamp(abs(x - edge_x) / aa, 0.0, 1.0); }
    let normal = vec2<f32>(-sin(b), cos(b));
    let front = dot(normal, eye - pt) > 0.0;
    var lit_normal = -normal;
    if (front) { lit_normal = normal; }
    if (front) {
      img = getFromColor(vec2<f32>(clamp(w + u, w, ratio) / ratio, y));
    } else {
      img = getToColor(vec2<f32>(clamp(w - u, 0.0, w) / ratio, y));
    }
    let diffuse = max(dot(lit_normal, LIGHT), 0.0) / LIGHT.y;
    var shade = mix(1.0, 0.25 + 0.75 * diffuse, SHADING);
    shade = shade * (1.0 - 0.35 * SHADING * lift * exp(-u / (0.04 * w)));
    img = vec4<f32>(img.rgb * shade, img.a);
    if (m >= 1.0) { return img; }
  }
  var color = getToColor(uv);
  if (x < 0.0) { color = getFromColor(uv); }
  let to_edge = edge - vec2<f32>(x, 0.0);
  let cos_edge = to_edge.x / max(length(to_edge), 1e-6);
  var side_sign = -1.0;
  if (x < 0.0) { side_sign = 1.0; }
  let blocked = 0.5 * abs(side_sign - cos_edge);
  var shadow = blocked * blocked * (3.0 - 2.0 * blocked);
  let slope = -LIGHT.x / LIGHT.y;
  let reach = vec3<f32>(bend.x, middle.x, edge.x) + slope * vec3<f32>(bend.y, middle.y, edge.y);
  let lo = min(0.0, min(reach.x, min(reach.y, reach.z)));
  let hi = max(0.0, max(reach.x, max(reach.y, reach.z)));
  let soft = max(0.25 * edge.y, 1e-5);
  shadow = max(0.5 * shadow, 0.85 * clamp(min(x - lo, hi - x) / soft + 0.5, 0.0, 1.0) * min(4.0 * lift, 1.0));
  color = vec4<f32>(color.rgb * (1.0 - SHADING * shadow), color.a);
  return mix(color, img, m);
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
