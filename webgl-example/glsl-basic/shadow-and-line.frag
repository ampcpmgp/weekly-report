// https://thebookofshaders.com/05/?lan=jp

precision mediump float;

# define PI 3.14159265359

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

float plot(vec2 st, float pct) {
  return smoothstep(pct - 0.02, pct, st.y) -
    smoothstep(pct, pct + 0.02, st.y);
}

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution;

  float y = pow(st.x, 2.0);
  // float y = exp(st.x * 0.5) - 1.0;
  // float y = sqrt(st.x);

  vec3 color = vec3(y);

  float pct = plot(st, y);
  color = (1.0 - pct) * color + pct * vec3(0.0, 1.0, 0.0);

  gl_FragColor = vec4(color, 1.0);
}
