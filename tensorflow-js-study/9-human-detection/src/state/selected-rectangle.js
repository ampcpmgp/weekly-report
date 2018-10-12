import { observable } from 'dob'

const selectedRectangle = observable({
  baseX: 0,
  baseY: 0,
  differenceWidth: 0,
  differenceHeight: 0,
  image: ''
})

export default selectedRectangle

export function getX () {
  const { baseX, differenceWidth } = selectedRectangle

  return differenceWidth > 0 ? baseX : baseX + differenceWidth
}

export function getY () {
  const { baseY, differenceHeight } = selectedRectangle

  return differenceHeight > 0 ? baseY : baseY + differenceHeight
}

export function getWidth () {
  const { differenceWidth } = selectedRectangle

  return Math.abs(differenceWidth)
}

export function getHeight () {
  const { differenceHeight } = selectedRectangle

  return Math.abs(differenceHeight)
}

export function setBaseX (x) {
  selectedRectangle.baseX = x
}

export function setBaseY (y) {
  selectedRectangle.baseY = y
}

export function setDifferenceWidth (width) {
  selectedRectangle.differenceWidth = width
}

export function setDifferenceHeight (height) {
  selectedRectangle.differenceHeight = height
}
