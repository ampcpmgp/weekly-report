const cv = window.cv

cv['onRuntimeInitialized'] = () => {
  document.getElementById('status').innerHTML = 'OpenCV.js is ready.'

  let inputElement = document.getElementById('fileInput')
  let imgElement = document.getElementById('imageSrc')

  inputElement.addEventListener('change', (e) => {
    imgElement.src = URL.createObjectURL(e.target.files[0])
  }, false)

  imgElement.onload = function () {
    const mat = cv.imread(imgElement)
    const dst = new cv.Mat()

    cv.cvtColor(mat, dst, cv.COLOR_RGBA2GRAY)
    cv.imshow('canvasOutput', dst)
    mat.delete()
    dst.delete()
  }
}
