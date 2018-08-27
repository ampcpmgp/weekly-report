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
