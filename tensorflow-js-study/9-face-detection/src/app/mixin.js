import * as cocoSsd from '@tensorflow-models/coco-ssd'
import sleep from '../util/sleep'

export default {
  model: null,
  image: null,
  result: [],
  isValid: true,
  canvas: document.createElement('canvas'),

  async analyze () {
    console.log('analyze start')
    const context = this.canvas.getContext('2d')
    const width = this.canvas.width
    const height = this.canvas.height

    context.drawImage(this.refs.video, 0, 0, width, height)

    const imageData = context.getImageData(0, 0, width, height)

    this.update({
      result: await this.model.detect(imageData),
      image: this.canvas.toDataURL()
    })

    if (this.isValid) {
      await sleep(0) // 描画させてから実行
      this.analyze()
    }
  },

  async init () {
    // mobilenet_v1 or mobilenet_v2 or lite_mobilenet_v2
    const modelPromise = cocoSsd.load('mobilenet_v2')

    this.on('mount', async () => {
      const videoPromise = navigator.mediaDevices.getUserMedia({ video: true })
      const mediaStream = await videoPromise

      this.refs.video.srcObject = mediaStream

      this.model = await modelPromise

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
