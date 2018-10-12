import * as cocoSsd from '@tensorflow-models/coco-ssd'
import * as tf from '@tensorflow/tfjs'
import sleep from '../util/sleep'

export default {
  status: '',
  model: {
    mobilenet_v2: null,
    lite_mobilenet_v2: null
  },
  image: '',
  result: [],
  isValid: true,
  fps: 0,
  selectedX: 0,
  selectedY: 0,
  isDraggable: false,
  isMounseEnter: false,
  selectedRectangle: {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    image: ''
  },
  canvas: document.createElement('canvas'),

  mouseEnter () {
    this.isMounseEnter = true
  },

  mouseLeave () {
    this.isMounseEnter = false
  },

  startRectangleSelection (e) {
    const rect = e.target.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    this.isDraggable = true

    this.selectedRectangle.x = x
    this.selectedRectangle.y = y
    this.selectedRectangle.width = 0
    this.selectedRectangle.height = 0

    this.update()
  },

  moveRectanglePoint (e) {
    if (!this.isDraggable) return

    const rect = e.target.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    const differenceWidth = x - this.selectedRectangle.x
    const differenceHeight = y - this.selectedRectangle.y

    if (differenceWidth < 0) {
      this.selectedRectangle.x = x
      this.selectedRectangle.width += Math.abs(differenceWidth)
    } else {
      this.selectedRectangle.width = Math.abs(differenceWidth)
    }

    if (differenceHeight < 0) {
      this.selectedRectangle.y = y
      this.selectedRectangle.height += Math.abs(differenceHeight)
    } else {
      this.selectedRectangle.height = Math.abs(differenceHeight)
    }

    this.update()
  },

  stopRectangleSelection (e) {
    this.isDraggable = false

    const imageData = this.drawSelectedRectangle()

    // TODO: 選択された矩形を再学習させたい
    console.log(tf.fromPixels(imageData))
  },

  drawSelectedRectangle () {
    const { x, y, width, height } = this.selectedRectangle
    this.refs.selectedRectangleImage.width = width
    this.refs.selectedRectangleImage.height = height

    const capturedContext = this.refs.captured.getContext('2d')
    const selectedImageContext = this.refs.selectedRectangleImage.getContext(
      '2d'
    )
    const imageData = capturedContext.getImageData(x, y, width, height)

    selectedImageContext.putImageData(imageData, 0, 0)

    return imageData
  },

  getLeft (bbox) {
    const [x] = bbox

    return (x / this.refs.video.videoWidth) * 100
  },

  getTop (bbox) {
    const [, y] = bbox

    return (y / this.refs.video.videoHeight) * 100
  },

  getWidth (bbox) {
    const [, , width] = bbox

    return (width / this.refs.video.videoWidth) * 100
  },

  getHeight (bbox) {
    const [, , , height] = bbox

    return (height / this.refs.video.videoHeight) * 100
  },

  drawCapturedImage (width, height, imageData) {
    const context = this.refs.captured.getContext('2d')
    this.refs.captured.width = width
    this.refs.captured.height = height

    context.putImageData(imageData, 0, 0)
  },

  captureImage () {
    const context = this.canvas.getContext('2d')
    const width = this.refs.video.videoWidth
    const height = this.refs.video.videoHeight
    const imageData = context.getImageData(
      0,
      0,
      width,
      this.refs.video.videoHeight
    )

    this.drawCapturedImage(width, height, imageData)
  },

  async analyze () {
    if (this.isMounseEnter) {
      await sleep(1000) // 描画させてから実行
      this.analyze()
      return
    }

    const context = this.canvas.getContext('2d')
    const width = this.canvas.width
    const height = this.canvas.height

    context.drawImage(this.refs.video, 0, 0, width, height)

    const imageData = context.getImageData(0, 0, width, height)
    const now = Date.now()

    this.update({
      result: await this.model[this.refs.modelName.value].detect(imageData),
      image: this.canvas.toDataURL()
    })

    const time = Date.now() - now
    const ms = 1000
    const fps = 1 / (time / ms)
    this.fps = Math.floor(fps * 100) / 100

    if (this.isValid) {
      await sleep(0) // 描画させてから実行
      this.analyze()
    }
  },

  async init () {
    this.on('mount', async () => {
      const videoPromise = navigator.mediaDevices.getUserMedia({ video: true })
      const mediaStream = await videoPromise

      this.refs.video.srcObject = mediaStream

      this.update({ status: 'model loading...' })
      // mobilenet_v1 or mobilenet_v2 or lite_mobilenet_v2
      this.model.lite_mobilenet_v2 = await cocoSsd.load('lite_mobilenet_v2')
      this.model.mobilenet_v2 = await cocoSsd.load('mobilenet_v2')
      this.update({ status: '' })

      const width = this.refs.video.videoWidth
      const height = this.refs.video.videoHeight

      this.canvas.width = width
      this.canvas.height = height

      this.analyze()
    })

    this.on('unmount', () => {
      this.isValid = false
    })
  }
}
