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
  canvas: document.createElement('canvas'),

  drawCapturedImage (width, height, imageData) {
    const context = this.refs.captured.getContext('2d')
    this.refs.captured.width = width
    this.refs.captured.height = height

    context.putImageData(imageData, 0, 0)

    const imageTensor = tf.fromPixels(imageData)

    // TODO: 転移学習をしたい
    console.log(imageTensor)
  },

  captureImage (bbox) {
    const [x, y, width, height] = bbox
    const context = this.canvas.getContext('2d')
    const imageData = context.getImageData(x, y, width, height)

    this.drawCapturedImage(width, height, imageData)
  },

  async analyze () {
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
