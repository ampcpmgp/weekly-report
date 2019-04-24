const vertexShader = `
varying vec2 vUv;

void main() {
  vUv = uv;
  gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}
`

const fragmentShader = `
varying vec2 vUv;
uniform float timeMsec; // A-Frame time in milliseconds.

void main() {
  float time = timeMsec / 1000.0;
  vec2 coord = 6.0 * vUv;

  for(int n = 1; n < 8; n++){
    float i = float(n);

    coord += vec2(
      0.7 / i * sin(i * coord.y + time + 0.3 * i) + 0.8,
      0.4 / i * sin(coord.x + time + 0.3 * i) + 1.6
    );
  }

  vec3 color = vec3(
    0.3 * sin(coord.x) + 0.7,
    0.2 * sin(coord.y) + 0.8,
    sin(coord.x + coord.y)
  );

  gl_FragColor = vec4(color, 1.0);
}
`

window.AFRAME.registerShader('leaf-flow', {
  schema: {
    timeMsec: { type: 'time', is: 'uniform' }
  },

  vertexShader: vertexShader,
  fragmentShader: fragmentShader
})
