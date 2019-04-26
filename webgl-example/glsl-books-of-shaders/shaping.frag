// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com

precision mediump float;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution.xy;
  vec3 color = vec3(0.0);

  // bottom-left
  vec2 bl = step(vec2(0.1), st);
  float pct = bl.x * bl.y;

  // top-right
  vec2 tr = smoothstep(vec2(0.0), vec2(0.3), 1.0 - st);
  pct *= tr.x * tr.y;

  color = vec3(pct);

  gl_FragColor = vec4(color, 1.0);
}
