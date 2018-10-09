export default {
  mediaStream: null,

  init () {
    this.on('mount', async () => {
      const video = navigator.mediaDevices.getUserMedia({ video: true })
      const mediaStream = await video

      this.refs.video.srcObject = mediaStream
    })
  }
}
