export function rotation ({ x = false, y = false, z = false, dur = 4000 }) {
  const to = `${x ? 360 : 0} ${y ? 360 : 0} ${z ? 360 : 0}`

  return `property: rotation; to: ${to}; loop: true; dur: ${dur}; easing: linear;`
}
