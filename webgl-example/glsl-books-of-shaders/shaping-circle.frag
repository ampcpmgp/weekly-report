// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com

precision mediump float;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution;
  float pct = 0.0;

  // a. The DISTANCE from the pixel to the center
  pct = distance(st, vec2(0.5)) + 0.1;

  // b. The LENGTH of the vector
  //    from the pixel to the center
  vec2 toCenter = vec2(0.5) - st;
  pct = length(toCenter) + 0.1;

  // c. The SQUARE ROOT of the vector
  //    from the pixel to the center
  vec2 tC = vec2(0.5) - st;
  pct = sqrt(tC.x * tC.x + tC.y * tC.y) + 0.1;

  vec3 color = vec3(pct);

  gl_FragColor = vec4(color, 1.0);
}
