precision mediump float;

uniform vec2 u_resolution;

float circleshape(vec2 position, float radius){
  return step(radius, length(position - vec2(0.5)));
}

void main(){
  vec2 position = gl_FragCoord.xy / u_resolution;
  float circle = circleshape(position, 0.3);
  vec3 color = vec3(circle);

  gl_FragColor = vec4(color, 1.0);
}
