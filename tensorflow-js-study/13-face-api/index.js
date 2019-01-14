const faceapi = window.faceapi
const video = window.video
const results = window.results
const captures = window.captures
const personDescriptos = {}
let lastResults = []

function getLabeledDescriptors () {
  return Object.keys(personDescriptos).map(
    name => new faceapi.LabeledFaceDescriptors(name, personDescriptos[name])
  )
}
function sleep (ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}
function setName (descriptor) {
  const name = window.prompt('input name')
  const descriptors = personDescriptos[name]

  if (descriptors) {
    descriptors.push(descriptor)
  } else {
    personDescriptos[name] = [descriptor]
  }
}
function setCaptures () {
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  canvas.width = video.videoWidth
  canvas.height = video.videoHeight
  canvas.getContext('2d').drawImage(video, 0, 0)

  captures.innerHTML = ''

  lastResults.forEach(result => {
    const capturedImgCanvas = document.createElement('canvas')
    const {
      detection: {
        _box: { _x, _y, _width, _height }
      },
      descriptor
    } = result

    const pixel = ctx.getImageData(_x, _y, _width, _height)
    const capturedImgCtx = capturedImgCanvas.getContext('2d')

    capturedImgCanvas.width = pixel.width
    capturedImgCanvas.height = pixel.height
    capturedImgCanvas.onclick = () => {
      setName(descriptor)
    }
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
    lastResults = await faceapi
      .detectAllFaces(video, options)
      .withFaceLandmarks(true)
      .withFaceDescriptors()
    const labeledDescriptors = getLabeledDescriptors()
    const faceMatcher =
      labeledDescriptors.length === 0
        ? null
        : new faceapi.FaceMatcher(labeledDescriptors)

    results.innerHTML = ''

    lastResults.forEach(result => {
      const {
        detection: {
          _box: { _x, _y, _width, _height }
        },
        descriptor
      } = result
      const frame = document.createElement('div')
      frame.className = 'frame'
      frame.style.left = `${_x}px`
      frame.style.top = `${_y}px`
      frame.style.width = `${_width}px`
      frame.style.height = `${_height}px`

      if (faceMatcher) {
        const bestMatch = faceMatcher.findBestMatch(descriptor)
        const name = bestMatch.toString()
        frame.textContent = name
      }

      results.appendChild(frame)
    })
  }
}
async function start () {
  await setupWebCam()
  await faceapi.loadTinyFaceDetectorModel('./models')
  await faceapi.loadFaceLandmarkTinyModel('./models')
  await faceapi.loadFaceRecognitionModel('./models')
  console.info('load mobilenet v1 model')

  detect()
}

start()
