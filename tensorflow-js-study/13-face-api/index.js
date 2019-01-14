const faceapi = window.faceapi
const video = window.video
const results = window.results
const captures = window.captures
let lastDetections = []

function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

function setCaptures () {
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  canvas.width = video.videoWidth
  canvas.height = video.videoHeight
  canvas.getContext('2d').drawImage(video, 0, 0)

  captures.innerHTML = ''

  lastDetections.forEach(detection => {
    const capturedImgCanvas = document.createElement('canvas')
    const {
      _box: { _x, _y, _width, _height }
    } = detection

    const pixel = ctx.getImageData(_x, _y, _width, _height)
    const capturedImgCtx = capturedImgCanvas.getContext('2d')

    capturedImgCanvas.width = pixel.width
    capturedImgCanvas.height = pixel.height
    capturedImgCtx.putImageData(pixel, 0, 0)
    captures.appendChild(capturedImgCanvas)
  })
}

async function setupWebCam () {
  const stream = await navigator.mediaDevices.getUserMedia({ video: true })
  video.srcObject = stream
  video.onclick = setCaptures

  return new Promise(resolve => {
    video.onloadedmetadata = resolve
  })
}

async function detect () {
  const options = new faceapi.TinyFaceDetectorOptions()

  while (true) {
    await sleep(0)
    lastDetections = await faceapi.detectAllFaces(video, options)

    results.innerHTML = ''

    lastDetections.forEach(detection => {
      const {
        _box: { _x, _y, _width, _height }
      } = detection
      const frame = document.createElement('div')
      frame.className = 'frame'
      frame.style.left = `${_x}px`
      frame.style.top = `${_y}px`
      frame.style.width = `${_width}px`
      frame.style.height = `${_height}px`

      results.appendChild(frame)
    })
  }
}

async function start () {
  await setupWebCam()
  await faceapi.loadTinyFaceDetectorModel('./weights')
  console.info('load mobilenet v1 model')

  detect()
}

start()
