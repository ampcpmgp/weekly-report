import * as cocoSsd from '@tensorflow-models/coco-ssd'
import sleep from '../util/sleep'

export default {
  status: '',
  model: null,
  image: '',
  result: [],
  isValid: true,
  fps: 0,
  canvas: document.createElement('canvas'),

  async analyze () {
    const context = this.canvas.getContext('2d')
    const width = this.canvas.width
    const height = this.canvas.height

    context.drawImage(this.refs.video, 0, 0, width, height)

    const imageData = context.getImageData(0, 0, width, height)
    const now = Date.now()

    this.update({
      result: await this.model.detect(imageData),
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
    // mobilenet_v1 or mobilenet_v2 or lite_mobilenet_v2
    const modelPromise = cocoSsd.load('lite_mobilenet_v2')

    this.on('mount', async () => {
      const videoPromise = navigator.mediaDevices.getUserMedia({ video: true })
      const mediaStream = await videoPromise

      this.refs.video.srcObject = mediaStream

      this.update({ status: 'model loading...' })
      this.model = await modelPromise
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
