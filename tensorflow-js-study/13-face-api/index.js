const faceapi = window.faceapi
const video = window.video
const results = window.results

function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function setupWebCam () {
  const stream = await navigator.mediaDevices.getUserMedia({ video: true })
  video.srcObject = stream

  return new Promise(resolve => {
    video.onloadedmetadata = resolve
  })
}

async function detect () {
  const options = new faceapi.TinyFaceDetectorOptions()

  while (true) {
    await sleep(0)
    const detections = await faceapi.detectAllFaces(video, options)

    results.innerHTML = ''

    detections.forEach(detection => {
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
