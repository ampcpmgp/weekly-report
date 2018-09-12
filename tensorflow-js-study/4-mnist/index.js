/**
 * @license
 * Copyright 2018 Google LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
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

import * as tf from '@tensorflow/tfjs'
import { MnistData } from './data'
import * as ui from './ui'

window.tf = tf

const model = tf.sequential()

model.add(
  tf.layers.conv2d({
    inputShape: [28, 28, 1],
    kernelSize: 5,
    filters: 6,
    strides: 1,
    activation: 'relu',
    kernelInitializer: 'varianceScaling'
  })
)
model.add(tf.layers.maxPooling2d({ poolSize: [2, 2], strides: [2, 2] }))
model.add(
  tf.layers.conv2d({
    kernelSize: 5,
    filters: 16,
    strides: 1,
    activation: 'relu',
    kernelInitializer: 'varianceScaling'
  })
)
model.add(tf.layers.maxPooling2d({ poolSize: [2, 2], strides: [2, 2] }))
model.add(tf.layers.flatten())
model.add(
  tf.layers.dense({
    units: 10,
    kernelInitializer: 'varianceScaling',
    activation: 'softmax'
  })
)

const LEARNING_RATE = 0.15

const optimizer = tf.train.sgd(LEARNING_RATE)

model.compile({
  optimizer: optimizer,
  loss: 'categoricalCrossentropy',
  metrics: ['accuracy']
})

const BATCH_SIZE = 64
const TRAIN_BATCHES = 45
const TEST_BATCH_SIZE = 1000
const TEST_ITERATION_FREQUENCY = 5

async function train () {
  ui.isTraining()

  const lossValues = []
  const accuracyValues = []

  for (let i = 0; i < TRAIN_BATCHES; i++) {
    const [batch, validationData] = tf.tidy(() => {
      const batch = data.nextTrainBatch(BATCH_SIZE)
      batch.xs = batch.xs.reshape([BATCH_SIZE, 28, 28, 1])

      let validationData

      if (i % TEST_ITERATION_FREQUENCY === 0) {
        const testBatch = data.nextTestBatch(TEST_BATCH_SIZE)
        validationData = [
          testBatch.xs.reshape([TEST_BATCH_SIZE, 28, 28, 1]),
          testBatch.labels
        ]
      }
      return [batch, validationData]
    })

    const history = await model.fit(batch.xs, batch.labels, {
      batchSize: BATCH_SIZE,
      validationData,
      epochs: 1
    })

    const loss = history.history.loss[0]
    const accuracy = history.history.acc[0]

    // Plot loss / accuracy.
    lossValues.push({ batch: i, loss: loss, set: 'train' })
    ui.plotLosses(lossValues)

    if (validationData != null) {
      accuracyValues.push({ batch: i, accuracy: accuracy, set: 'train' })
      ui.plotAccuracies(accuracyValues)
    }

    tf.dispose([batch, validationData])

    await tf.nextFrame()
  }
}

async function showPredictions () {
  const testExamples = 100
  const batch = data.nextTestBatch(testExamples)

  tf.tidy(() => {
    const output = model.predict(batch.xs.reshape([-1, 28, 28, 1]))
    const axis = 1
    const predictions = Array.from(output.argMax(axis).dataSync())
    const labels = Array.from(batch.labels.argMax(axis).dataSync())

    ui.showTestResults(batch, predictions, labels)
  })
}

let data
async function load () {
  data = new MnistData()
  await data.load()
}

async function mnist () {
  ui.init()
  await load()
  await train()
  showPredictions()
}
mnist()
