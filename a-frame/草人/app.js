import {
  html,
  render
} from 'https://unpkg.com/lighterhtml@0.9.4/esm/index.js?module'

const COLORS = {
  BASE: '#67E231',
  LEAF: '#D15320'
}

const hand = () => html`
  <a-box color="${COLORS.BASE}" width="0.2" height="1.5" depth="0.2"></a-box>
  <a-sphere color="#FF8349" radius="0.15" position="0 -0.7 0"></a-sphere>
`
const foot = () => html`
  <a-box color="${COLORS.BASE}" width="0.2" depth="0.2"></a-box>
  <a-box
    color="${COLORS.BASE}"
    width="0.24" depth="0.32" height="0.2" position="0 0.5 0">
  </a-box>
  <a-box color="#FF8349" width="0.24" depth="0.1" height="0.1" position="0 0.5 -0.2"></a-box>
  <a-box color="#FF8349" width="0.24" depth="0.05" height="0.1" position="0 0.5 0.2"></a-box>
`
const leaf = ({ position, rotation }) => html`
  <a-cone color="${
  COLORS.LEAF
}" radius-bottom="0.2" position="${position}" rotation="${rotation}"></a-cone>
`

const eye = () => html`
  <a-entity scale="1.2 1.2 1.2">
    <a-circle color="#D15320" radius="0.1" theta-start="0" theta-length="180" material="opacity: 0.6"></a-circle>
    <a-circle color="#527638" radius="0.1" theta-start="180" theta-length="180" material="opacity: 0.6"></a-circle>
    <a-circle color="#734931" radius="0.05" scale="0.8 1 1" metalness="1"></a-circle>
  </a-entity>
`

const mouse = () => html`
  <a-circle
    color="white" radius="${1 /
      3}" scale="3 1 1" material="opacity: 0.6" theta-start="0" theta-length="180">
  </a-circle>
  <a-circle
    color="#755033" theta-start="180" radius="0.06" theta-length="180" position="0 0.05 1">
  </a-circle>
  <a-circle
    color="white" theta-start="180" radius="0.04" theta-length="180" position="0 0.04 1">
  </a-circle>
`

const scene = () => html`
  <a-scene style="height: 400px" embedded>
    <a-sky color="#ECECEC"></a-sky>
    <a-light type="hemisphere" color="#ccc" position="-1 1 0"></a-light>

    <a-entity id="whole" position="0 1.5 -4">

      <a-entity id="head" position="0 0.8 0" rotation="0 0 0">
        <a-box
          color="${COLORS.BASE}" width="2"
        ></a-box>
        ${leaf({ position: '1 0.5 0.5' })}
        ${leaf({ position: '-1 0.5 0.5', rotation: '0 90 0' })}
        <a-entity position="0.3 -0.1 0.51">
          ${eye()}
        </a-entity>

        <a-entity position="-0.3 -0.1 0.51">
          ${eye()}
        </a-entity>

        <a-entity position="0 -0.5 0.501">
          ${mouse()}
        </a-entity>
      </a-entity>

      <a-entity id="body" position="0 0 0">
        <a-cone color="${COLORS.BASE}" rotation="0 0 0" height="1.5"></a-cone>

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

render(window.main, scene)
