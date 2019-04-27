// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com

precision mediump float;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

float random (vec2 st) {
  return fract(
    sin(
      dot(
        st.xy,
        vec2(st.xy * 1000.0)
      )
    ) * 1000.0
  );
}

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution.xy;

  float rnd = random(st);

  gl_FragColor = vec4(vec3(rnd), 1.0);
}
