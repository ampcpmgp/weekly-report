const tf = window.tf

window.tensor = () => {
  tf.scalar(3.14).print()

  tf.tensor([1, 2, 3, 4]).print()
  tf.tensor([[1, 2], [3, 4]]).print()
  tf.tensor([1, 2, 3, 4], [2, 2]).print()
  tf.tensor([[[1], [2]], [[5], [6]]]).print()

  tf.tensor1d([1, 2, 3]).print()
  tf.tensor2d([1, 2, 3, 4], [2, 2]).print()
  tf.tensor3d([1, 2, 3, 4], [2, 2, 1]).print()

  tf.tensor4d([1, 2, 3, 4], [1, 2, 2, 1]).print()
  tf.tensor5d([1, 2, 3, 4, 5, 6, 7, 8], [1, 2, 2, 2, 1]).print()
  tf.tensor6d([1, 2, 3, 4, 5, 6, 7, 8], [1, 1, 2, 2, 2, 1]).print()
}

// window.tensor()

window.tensorSpecial = () => {
  const buffer = tf.buffer([2, 2])
  buffer.set(3, 0, 0) // value, x, y
  buffer.set(5, 1, 1)
  buffer.toTensor().print()

  buffer
    .toTensor()
    .clone()
    .print()

  tf.eye(4, 2, [2]).print()

  tf.fill([2, 3], 4).print()

  const image = new window.ImageData(1, 1)
  image.data[0] = 100
  image.data[1] = 150
  image.data[2] = 200
  image.data[3] = 255
  tf.fromPixels(image).print()

  tf.linspace(0, 9, 10).print()

  tf.oneHot(tf.tensor1d([0, 1], 'int32'), 3).print()

  tf.ones([2, 2]).print()

  tf.onesLike(tf.tensor([1, 2])).print()

  tf.range(0, 9, 2).print()

  tf.truncatedNormal([2, 2]).print()

  const x = tf.variable(tf.tensor([1, 2, 3]))
  x.assign(tf.tensor([4, 5, 6]))
  x.print()
}

// window.tensorSpecial()

window.classTensor = async () => {
  const baseTensor = tf.tensor3d([1, 2, 3, 4], [2, 2, 1])

  baseTensor.flatten().print()

  tf.tensor1d([4])
    .asScalar()
    .print()
  tf.tensor2d([5], [1, 1])
    .as1D()
    .print()
  tf.tensor4d([6], [1, 1, 1, 1])
    .as2D(1, 1)
    .print()
  tf.tensor4d([7], [1, 1, 1, 1])
    .as3D(1, 1, 1)
    .print()
  tf.scalar(8)
    .as4D(1, 1, 1, 1)
    .print()

  tf.tensor1d([4.5])
    .asType('int32')
    .print()

  console.log(tf.tensor1d([5]).buffer())

  const data = await tf.tensor1d([2]).data()
  console.log(data)

  console.log(tf.tensor1d([3]).dataSync())

  tf.dispose() // メモリ破棄

  tf.tensor1d([4], 'bool')
    .toFloat()
    .print()
  tf.tensor1d([4.5])
    .toInt()
    .print()
  tf.tensor1d([4])
    .toBool()
    .print()

  tf.tensor1d([1, 2, 3, 4])
    .reshape([2, 2])
    .print()
  // tf.tensor1d([1, 2, 3, 4]).reshapeAs().print() // FIXME: 使い方がわからない

  tf.tensor1d([1, 2, 3, 4])
    .expandDims(1)
    .print()

  tf.tensor1d([1, 2, 3, 4])
    .cumsum()
    .print()

  tf.tensor2d([[5]])
    .squeeze()
    .print()

  tf.tensor2d([[5]])
    .clone()
    .print()

  console.log(tf.tensor1d([1, 2, 3, 4]).toString())
}

// window.classTensor()

window.classVariable = () => {
  const x = tf.variable(tf.tensor([1, 2, 3]))
  x.assign(tf.tensor([4, 5, 6]))
  x.print()
}

// window.classVariable()

window.TensorBuffer = () => {
  const buffer = tf.buffer([2, 2])
  buffer.set(3, 0, 0) // value, x, y
  buffer.set(5, 1, 0)

  console.log(buffer.get(1, 0)) // x, y

  buffer.toTensor().print()
}

// window.TensorBuffer()

window.TensorTransformation = () => {
  const blockShape = [2, 2]
  const crops = [[0, 0], [0, 0]]
  tf.tensor4d([1, 2, 3, 4], [4, 1, 1, 1]).print()
  tf.tensor4d([1, 2, 3, 4], [4, 1, 1, 1])
    .batchToSpaceND(blockShape, crops) // TODO: このコードの意味の理解
    .print()

  tf.cast(tf.tensor1d([1.5, 2.5, 3]), 'int32').print()

  tf.tensor1d([1, 2, 3, 4])
    .expandDims(1)
    .print()

  tf.tensor1d([1, 2, 3, 4])
    .pad([[1, 2]]) // TODO: このコードの意味の理解
    .print()

  tf.tensor1d([1, 2, 3, 4])
    .reshape([2, 2])
    .print()

  // FIXME: 公式通りに書いても動かない。
  // https://js.tensorflow.org/api/0.12.5/#spaceToBatchND
  // const paddings = [[0,0],[0,0]]
  // tf.tensor4d([1, 2, 3, 4], [4, 1, 1, 1]).spaceToBatchND(blockShape, paddings).print()

  tf.tensor([1, 2, 3, 4], [1, 1, 4])
    .squeeze()
    .print()
}

// window.TensorTransformation()

window.TensorSliceAndJoin = () => {
  tf.tensor1d([1, 2])
    .concat(tf.tensor1d([3, 4]))
    .print()
  tf.concat([tf.tensor1d([1]), tf.tensor1d([3])]).print()
  tf.concat(
    [tf.tensor2d([[1, 2], [1, 10]]), tf.tensor2d([[3, 2], [5, 10]])],
    1
  ).print()

  tf.tensor1d([3, 5, 100])
    .gather(tf.tensor1d([0, 0, 2], 'int32')) // TODO: このコードの意味の理解
    .print()
  tf.tensor2d([3, 5, 100, 200], [2, 2])
    .gather(tf.tensor1d([0, 0, 1], 'int32'))
    .print()

  tf.tensor1d([1, 2, 3])
    .reverse()
    .print()

  tf.tensor1d([1, 2, 3, 4])
    .slice([1], [3])
    .print()
  tf.tensor2d([1, 2, 3, 4], [2, 2])
    .slice([1, 0], [1, 2])
    .print()

  tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]).print()
  tf.split(tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]), 2, 1)[0].print()
  tf.split(tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]), 2, 1)[1].print()
  tf.split(
    tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]),
    [1, 2, 1],
    1
  )[0].print()
  tf.split(
    tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]),
    [1, 2, 1],
    1
  )[1].print()
  tf.split(
    tf.tensor2d([1, 2, 3, 4, 5, 6, 7, 8], [2, 4]),
    [1, 2, 1],
    1
  )[2].print()

  // TODO: このコードの意味の理解
  tf.stack([
    tf.tensor1d([1, 2]),
    tf.tensor1d([3, 4]),
    tf.tensor1d([5, 6])
  ]).print()

  tf.tensor1d([1, 2])
    .tile([3])
    .print()
  tf.tensor2d([1, 2, 3, 4], [2, 2])
    .tile([3, 1])
    .print()

  // TODO: このコードの意味の理解
  tf.unstack(tf.tensor2d([1, 2, 3, 4], [2, 2])).forEach(tensor =>
    tensor.print()
  )
}

// window.TensorSliceAndJoin()

window.TensorRandom = () => {
  tf.multinomial(tf.tensor([0.75, 0.25]), 3).print()

  tf.randomNormal([2, 2]).print()

  tf.randomUniform([2, 2]).print()
}

// window.TensorRandom()

window.ModelCreationSequential = () => {
  const model = tf.sequential()
  model.add(tf.layers.dense({ units: 32, inputShape: [50] }))
  model.add(tf.layers.dense({ units: 4 }))
  console.log(JSON.stringify(model.outputs[0].shape))

  const modelBatchShape = tf.sequential()
  modelBatchShape.add(
    tf.layers.dense({ units: 32, batchInputShape: [null, 50] })
  )
  console.log(JSON.stringify(modelBatchShape.outputs[0].shape))

  const modelConstructedLayer = tf.sequential({
    layers: [
      tf.layers.dense({ units: 32, inputShape: [50] }),
      tf.layers.dense({ units: 4 })
    ]
  })
  console.log(JSON.stringify(modelConstructedLayer.outputs[0].shape))
}

// window.ModelCreationSequential()

window.ModelCreation = () => {
  // tf.ones()はどのような過程を経て変換されたのかを理解する
  // input/outputの関係性を重点的に
  const input = tf.input({ shape: [5] })
  const denseLayer1 = tf.layers.dense({ units: 10, activation: 'relu' })
  const denseLayer2 = tf.layers.dense({ units: 4, activation: 'softmax' })
  const output = denseLayer2.apply(denseLayer1.apply(input))
  const model = tf.model({ inputs: input, outputs: output })

  tf.ones([2, 5]).print()
  model.predict(tf.ones([2, 5])).print()
}

// window.ModelCreation()

window.ModelInput = () => {
  const x = tf.input({ shape: [32] })
  const y = tf.layers.dense({ units: 3, activation: 'softmax' }).apply(x)
  const model = tf.model({ inputs: x, outputs: y })
  const tensor = tf.ones([2, 32])
  tensor.print()
  model.predict(tf.ones([2, 32])).print()
}

// window.ModelInput()

window.FrozenModelLoading = async () => {
  const GOOGLE_CLOUD_STORAGE_DIR =
    'https://storage.googleapis.com/tfjs-models/savedmodel/'
  const MODEL_URL =
    GOOGLE_CLOUD_STORAGE_DIR + 'mobilenet_v2_1.0_224/tensorflowjs_model.pb'
  const WEIGHTS_URL =
    GOOGLE_CLOUD_STORAGE_DIR + 'mobilenet_v2_1.0_224/weights_manifest.json'
  const model = await tf.loadFrozenModel(MODEL_URL, WEIGHTS_URL)
  const zeros = tf.zeros([1, 224, 224, 3])
  model.predict(zeros).print()
}

// window.FrozenModelLoading()

window.ModelLoading = async () => {
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [3] })]
  })
  console.log('Prediction from original model:')
  model.predict(tf.ones([1, 3])).print()

  await model.save('localstorage://my-model-1')

  const loadedModel = await tf.loadModel('localstorage://my-model-1')
  console.log('Prediction from loaded model:')
  loadedModel.predict(tf.ones([1, 3])).print()
}

// window.ModelLoading()

window.ModelManagement = async () => {
  const model = tf.sequential()
  model.add(
    tf.layers.dense({ units: 1, inputShape: [10], activation: 'sigmoid' })
  )
  await model.save('localstorage://demo/management/model1')

  try {
    await tf.io.removeModel('indexeddb://demo/management/model1')
  } catch (error) {
    // 初回はモデルが無いため、そのエラーは許容
    console.warn(error)
  }

  console.log(JSON.stringify(await tf.io.listModels(), null, '  '))

  await tf.io.copyModel(
    'localstorage://demo/management/model1',
    'indexeddb://demo/management/model1'
  )

  console.log('---------')

  console.log(JSON.stringify(await tf.io.listModels(), null, '  '))

  await tf.io.moveModel(
    'localstorage://demo/management/model1',
    'indexeddb://demo/management/model1'
  )

  console.log('---------')

  console.log(JSON.stringify(await tf.io.listModels(), null, '  '))
}

// window.ModelManagement()

window.ModelClassSummary = () => {
  const input1 = tf.input({ shape: [10] })
  const input2 = tf.input({ shape: [20] })
  const dense1 = tf.layers.dense({ units: 4 }).apply(input1)
  const dense2 = tf.layers.dense({ units: 8 }).apply(input2)
  const concat = tf.layers.concatenate().apply([dense1, dense2])
  const output = tf.layers
    .dense({ units: 3, activation: 'softmax' })
    .apply(concat)
  const model = tf.model({ inputs: [input1, input2], outputs: output })

  model.summary()
}

// window.ModelClassSummary()

window.ModelClassEvaluate = () => {
  // TODO: evaluate の動作をきちんと理解する
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })

  model.compile({ optimizer: 'sgd', loss: 'meanSquaredError' })

  const result = model.evaluate(tf.ones([8, 10]), tf.ones([8, 1]), {
    batchSize: 4
  })
  result.print()
}

// window.ModelClassEvaluate()

window.ModelClassPredict = () => {
  // TODO: predict の意味をきちんと理解する
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })

  model.predict(tf.ones([8, 10]), { batchSize: 4 }).print()
}

// window.ModelClassPredict()

window.ModelClassPredictOnBatch = () => {
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })
  model.predictOnBatch(tf.ones([8, 10])).print()
}

// window.ModelClassPredictOnBatch()

window.ModelClassFit = async () => {
  // TODO: fit の意味をきちんと理解する
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })
  model.compile({ optimizer: 'sgd', loss: 'meanSquaredError' })

  for (let i = 1; i < 5; ++i) {
    const h = await model.fit(tf.ones([8, 10]), tf.ones([8, 1]), {
      batchSize: 4,
      epochs: 3
    })

    console.log(`Loss after Epoch ${i} : ${h.history.loss[0]}`)
  }
}

// window.ModelClassFit()

window.ModelClassSave = async () => {
  const model = tf.sequential({
    layers: [
      tf.layers.dense({
        units: 1,
        inputShape: [3]
      })
    ]
  })

  model.predict(tf.ones([1, 3])).print()

  await model.save('localstorage://my-model-1')
  const loadedModel = await tf.loadModel('localstorage://my-model-1')

  loadedModel.predict(tf.ones([1, 3])).print()
}

// window.ModelClassSave()

window.SequentialClass = async () => {
  const model = tf.sequential()
  model.add(tf.layers.dense({ units: 1, inputShape: [1] }))
  model.compile({ loss: 'meanSquaredError', optimizer: 'sgd' })

  const xs = tf.tensor2d([1, 2, 3, 4], [4, 1])
  const ys = tf.tensor2d([1, 3, 5, 7], [4, 1])

  await model.fit(xs, ys)
  model.predict(tf.tensor2d([5], [1, 1])).print()
}

// window.SequentialClass()

window.SequentialClassAdd = () => {
  // TODO: randomNormal が 2次元テンソルになってる理由と、
  // 指定したinputShapeがそれに合う理由を理解し、後続のモデル追加を試す。

  // 参考
  // tf.tensor1d([1, 2, 3]).print()
  // tf.tensor2d([1, 2, 3, 4], [4, 1]).print()

  const model = tf.sequential()

  model.add(tf.layers.dense({ units: 8, inputShape: [1] }))
  // model.add(tf.layers.dense({units: 4, activation: 'relu6'}))
  // model.add(tf.layers.dense({units: 1, activation: 'relu6'}))

  const tensor = tf.randomNormal([10, 1])

  tensor.print()
  model.predict(tensor).print()
}

// window.SequentialClassAdd()

window.SequentialClassSummary = () => {
  const model = tf.sequential()

  model.add(
    tf.layers.dense({ units: 100, inputShape: [10], activation: 'relu' })
  )
  model.add(tf.layers.dense({ units: 1, activation: 'sigmoid' }))

  model.summary()
}

// window.SequentialClassSummary()

window.SequentialClassEvaluate = () => {
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })

  model.compile({ optimizer: 'sgd', loss: 'meanSquaredError' })

  // TODO: evaluate がどう動いているのかをちゃんと理解する
  const result = model.evaluate(tf.ones([8, 10]), tf.ones([8, 1]), {
    batchSize: 4
  })

  result.print()
}

// window.SequentialClassEvaluate()

window.SequentialClassPredict = () => {
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })

  const ones = tf.ones([2, 10])

  ones.print()
  model.predict(ones).print()
}

// window.SequentialClassPredict()

window.SequentialClassFit = async () => {
  const model = tf.sequential({
    layers: [tf.layers.dense({ units: 1, inputShape: [10] })]
  })

  model.compile({ optimizer: 'sgd', loss: 'meanSquaredError' })

  const history = await model.fit(tf.ones([8, 10]), tf.ones([8, 1]), {
    batchSize: 4,
    epochs: 3
  })

  console.log(history.history.loss[0])
}

// window.SequentialClassFit()

window.SymbolicTensorClass = () => {
  // 具体的な値を持たないTensorのプレースホルダ
}

// window.SymbolicTensorClass()
