import {
  html,
  render
} from 'https://unpkg.com/lighterhtml@0.9.4/esm/index.js?module'
import { rotation } from './utils.js'

void rotation

const COLORS = {
  BASE: '#67E231',
  LEAF: '#D15320'
}

function hand () {
  return html`
    <a-box color="${COLORS.BASE}" width="0.2" height="1.5" depth="0.2"></a-box>
    <a-sphere color="#FF8349" radius="0.15" position="0 -0.7 0"></a-sphere>
  `
}

function foot () {
  return html`
    <a-box color="${COLORS.BASE}" width="0.2" depth="0.2"></a-box>
    <a-box
      color="${COLORS.BASE}"
      width="0.24" depth="0.32" height="0.2" position="0 0.5 0">
    </a-box>
    <a-box color="#FF8349" width="0.24" depth="0.1" height="0.1" position="0 0.5 -0.2"></a-box>
    <a-box color="#FF8349" width="0.24" depth="0.05" height="0.1" position="0 0.5 0.2"></a-box>
  `
}

function leaf ({ position, rotation }) {
  return html`
    <a-cone color="${
  COLORS.LEAF
}" radius-bottom="0.2" position="${position}" rotation="${rotation}"></a-cone>
  `
}

function eye () {
  return html`
    <a-circle
      material="shader: eye;"
      scale="1.2 1.2 1.2"
      radius="0.1"
    ></a-circle>
  `
}

function mouse () {
  return html`
    <a-circle
      color="#755033" theta-start="180" radius="0.08" theta-length="180" position="0 0.05 1.02">
    </a-circle>
    <a-circle
      color="white" theta-start="180" radius="0.06" theta-length="180" position="0 0.04 1.02">
    </a-circle>
  `
}

function scene () {
  return html`
    <a-scene style="height: 400px" embedded>

      <a-sky color="#ECECEC"></a-sky>
      <a-light type="hemisphere" color="#ccc" position="-1 1 0"></a-light>

      <a-entity id="whole" position="0 1.5 -4">

        <a-entity id="head" position="0 0.8 0" rotation="0 0 0">
          <a-box
            material="shader:leaf-flow;"
            width="2"
          ></a-box>
          ${leaf({ position: '1 0.5 0.5' })}
          ${leaf({ position: '-1 0.5 0.5', rotation: '0 90 0' })}
          <a-entity position="0.3 -0.1 0.51">
            ${eye()}
          </a-entity>

          <a-entity position="-0.3 -0.1 0.51">
            ${eye()}
          </a-entity>

          <a-entity position="0 -0.4 -0.5">
            ${mouse()}
          </a-entity>
        </a-entity>

        <a-entity id="body" position="0 0 0">
          <a-cone
            material="shader:leaf-flow;"
            rotation="0 90 0"
            height="1.5"></a-cone>

          <a-entity position="0.8 -0.2 0" rotation="0 0 35">
            ${hand()}
          </a-entity>

          <a-entity position="-0.8 -0.2 0" rotation="0 180 35">
            ${hand()}
          </a-entity>

          <a-entity position="0.5 -1 0.5" rotation="90 0 0">
            ${foot()}
          </a-entity>

          <a-entity position="-0.5 -1 0.5" rotation="90 0 0">
            ${foot()}
          </a-entity>
        </a-entity>
      </a-entity>
    </a-scene>
  `
}

render(window.main, scene)
