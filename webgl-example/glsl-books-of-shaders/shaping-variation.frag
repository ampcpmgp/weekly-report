// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com

precision mediump float;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution.xy;
  vec3 color = vec3(0.0);

  vec2 pos = vec2(0.5) - st;

  float r = length(pos) * 2.0;
  float a = atan(pos.y, pos.x);

  float f = cos(a * 3.);
  f = abs(cos(a * 3.)); // 大きい花
  f = abs(cos(a * 2.5)) * .5 + .3; // 小さい花
  f = abs(cos(a * 12.) * sin(a * 3.)) * .8 + .1; // 結晶
  f = smoothstep(-.5, 1., cos(a * 10.)) * 0.2 + 0.5; // 歯車

  // 課題: 回転アニメーションを入れる
  // 課題: 輪郭線だけを描く

  color = vec3(1. - smoothstep(f, f + 0.02, r));

  gl_FragColor = vec4(color, 1.0);
}
