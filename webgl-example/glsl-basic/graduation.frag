precision mediump float;

uniform vec2 u_resolution;
uniform float u_time;

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution.xy;
  float red = st.x;
  float green = st.y;
  float blue = abs(sin(u_time));
  vec3 color = vec3(red, green, blue);

  gl_FragColor = vec4(color, 1.0);
}
