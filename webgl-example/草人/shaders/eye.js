const vertexShader = `
varying vec2 vUv;

void main() {
  vUv = uv;
  gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}
`

const fragmentShader = `
varying vec2 vUv;
uniform float timeMsec;

void main() {
  float pct = distance(vUv, vec2(0.5));
  vec3 color = vec3(0.5, pct, 0.1);

  gl_FragColor = vec4(color, 1.0);
}
`

window.AFRAME.registerShader('eye', {
  schema: {
    timeMsec: { type: 'time', is: 'uniform' }
  },

  vertexShader: vertexShader,
  fragmentShader: fragmentShader
})
