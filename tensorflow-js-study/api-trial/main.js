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
  buffer.set(3, 0, 0) // argument - value, x, y
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
  // tf.tensor4d([6], [1, 1, 1, 1]).as2D().print() // FIXME: 動かし方がわからない
  // tf.tensor4d([6], [1, 1, 1, 1]).as3D().print() // FIXME: 動かし方がわからない
  // tf.tensor6d([[[[[[6]]]]]]).as4D().print() // FIXME: 動かし方がわからない

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

window.classVariable()

window.TensorBuffer = () => {}
