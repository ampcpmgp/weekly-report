precision mediump float;

const float PI = 3.141592653589;
uniform vec2 u_resolution;


float polygonshape(vec2 position, float radius, float sides){
  position = position * 2.0 - 1.0;
  float angle = atan(position.x, position.y);
  float slice = PI * 2.0 / sides;

  return step(radius, cos(floor(0.5 + angle / slice) * slice - angle) * length(position));

}

void main(){
 vec2 position = gl_FragCoord.xy / u_resolution;
 float polygon = polygonshape(position, 0.6, 6.0);
 vec3 color = vec3(polygon);

gl_FragColor = vec4(color, 1.0);
}
