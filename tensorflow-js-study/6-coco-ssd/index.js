/**
 * @license
 * Copyright 2018 Google LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * =============================================================================
 */

// TODO(ping): Switch to package import when the npm is published.
import * as objectDetection from './src'

import imageURL from './image1.jpg'
import image2URL from './image2.jpg'

if (module.hot) {
  module.hot.dispose(function () {
    window.location.reload()
  })
}

let modelPromise

window.onload = () => {
  modelPromise = objectDetection.load()
}

const button = document.getElementById('toggle')
button.onclick = () => {
  image.src = image.src.endsWith(imageURL) ? image2URL : imageURL
}

const select = document.getElementById('base_model')
select.onchange = async (event) => {
  const model = await modelPromise
  model.dispose()
  modelPromise = objectDetection.load(
    event.srcElement.options[event.srcElement.selectedIndex].value)
}

const image = document.getElementById('image')
image.src = imageURL

const runButton = document.getElementById('run')
runButton.onclick = async () => {
  const model = await modelPromise
  const result = await model.detect(image)

  const c = document.getElementById('canvas')
  const context = c.getContext('2d')
  context.drawImage(image, 0, 0)
  context.font = '10px Arial'

  for (let i = 0; i < result.length; i++) {
    context.beginPath()
    context.rect(...result[i].bbox)
    context.lineWidth = 1
    context.strokeStyle = 'green'
    context.fillStyle = 'green'
    context.stroke()
    context.fillText(
      result[i].score.toFixed(3) + ' ' + result[i].class,
      result[i].bbox[0],
      result[i].bbox[1] > 10 ? result[i].bbox[1] - 5 : 10)
  }
}
