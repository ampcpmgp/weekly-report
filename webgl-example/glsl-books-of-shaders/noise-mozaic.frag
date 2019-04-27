// Author @patriciogv - 2015
// Title: Mosaic

precision mediump float;

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

float random(vec2 st) {
  return fract(
    sin(
      dot(
        st.xy,
        vec2(st.xy * 1000.0)
      )
    ) * 1000.0);
}


void main() {
  vec2 st = gl_FragCoord.xy / u_resolution.xy;

  st *= 10.0; // Scale the coordinate system by 10
  vec2 ipos = floor(st); // get the integer coords
  vec2 fpos = fract(st); // get the fractional coords

  // Assign a random value based on the integer coord
  vec3 color = vec3(random(ipos));

  // Uncomment to see the subdivided grid
  // color = vec3(fpos, 0.0);

  gl_FragColor = vec4(color, 1.0);
}
