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

import * as tf from '@tensorflow/tfjs';
import {CLASSES} from './classes';

const BASE_PATH = 'https://storage.googleapis.com/tfjs-models/savedmodel/';

export type ObjectDetectionBaseModel =
    'ssd_mobilenet_v1'|'ssd_mobilenet_v2'|'ssdlite_mobilenet_v2';

export interface DetectedObject {
  bbox: [number, number, number, number];  // [x, y, width, height]
  class: string;
  score: number;
}

export async function load(
    base: ObjectDetectionBaseModel = 'ssdlite_mobilenet_v2'
  ) {
  const objectDetection = new ObjectDetection(base);
  await objectDetection.load();
  return objectDetection;
}

export class ObjectDetection {
  private modelPath: string;
  private weightPath: string;
  private model: tf.FrozenModel;

  constructor(base: ObjectDetectionBaseModel) {
    this.modelPath = `${BASE_PATH}${base}/` +
        `tensorflowjs_model.pb`;
    this.weightPath = `${BASE_PATH}${base}/` +
        `weights_manifest.json`;
  }

  async load() {
    this.model = await tf.loadFrozenModel(this.modelPath, this.weightPath);
  }

  /**
   * Infers through the model.
   *
   * @param img The image to classify. Can be a tensor or a DOM element image,
   * video, or canvas.
   * @param maxNumBoxes The maximum number of bounding boxes of detected
   * objects. There can be multiple objects of the same class, but at different
   * locations. Defaults to 20.
   */
  async infer(
      img: tf.Tensor3D|ImageData|HTMLImageElement|HTMLCanvasElement|HTMLVideoElement,
      maxNumBoxes: number = 20
    ): Promise<DetectedObject[]> {
      const batched = tf.tidy(() => {
        if (!(img instanceof tf.Tensor)) {
          img = tf.fromPixels(img);
        }
        // Reshape to a single-element batch so we can pass it to executeAsync.
        return img.expandDims(0);
      })
      const height = batched.shape[1];
      const width = batched.shape[2];

      // model returns two tensors:
      // 1. box classification score with shape of [1, 1917, 90]
      // 2. box location with shape of [1, 1917, 1, 4]
      // where 1917 is the number of box detectors, 90 is the number of classes.
      // and 4 is the four coordinates of the box.
      const result = await this.model.executeAsync(batched) as tf.Tensor[];

      const scores = result[0].dataSync() as Float32Array;
      const boxes = result[1].dataSync() as Float32Array;

      // clean the webgl tensors
      batched.dispose();
      tf.dispose(result);

      // get classification
      const [maxScores, classes] =
          this.calculateMaxScores(scores, result[0].shape[1], result[0].shape[2]);

      const prevBackend = tf.getBackend();
      // run post process in cpu
      tf.setBackend('cpu');

      // get location?
      const indexTensor = tf.tidy(() => {
        const boxes2 =
            tf.tensor2d(boxes, [result[1].shape[1], result[1].shape[3]]);
        return tf.image.nonMaxSuppression(
            boxes2, maxScores, maxNumBoxes, 0.5, 0.5);
      });

      const indexes = indexTensor.dataSync() as Float32Array;
      indexTensor.dispose();

      // restore previous backend
      tf.setBackend(prevBackend);

      return this.buildDetectedObjects(
          width, height, boxes, maxScores, indexes, classes);
    }

  private buildDetectedObjects(
      width: number,
      height: number,
      boxes: Float32Array,
      scores: number[],
      indexes: Float32Array,
      classes: number[]
    ): DetectedObject[] {
      const count = indexes.length;
      const objects: DetectedObject[] = [];

      for (let i = 0; i < count; i++) {
        const bbox = [];
        for (let j = 0; j < 4; j++) {
          bbox[j] = boxes[indexes[i] * 4 + j];
        }
        const minY = bbox[0] * height;
        const minX = bbox[1] * width;
        const maxY = bbox[2] * height;
        const maxX = bbox[3] * width;
        bbox[0] = minX;
        bbox[1] = minY;
        bbox[2] = maxX - minX;
        bbox[3] = maxY - minY;
        objects.push({
          bbox: bbox as [number, number, number, number],
          class: CLASSES[classes[indexes[i]] + 1].displayName,
          score: scores[indexes[i]]
        });
      }
      return objects;
    }

  private calculateMaxScores(
      scores: Float32Array,
      numBoxes: number,
      numClasses: number
    ): [number[], number[]] {
      const maxes = [];
      const classes = [];

      for (let i = 0; i < numBoxes; i++) {
        let max = Number.MIN_VALUE;
        let index = -1;
        for (let j = 0; j < numClasses; j++) {
          if (scores[i * numClasses + j] > max) {
            max = scores[i * numClasses + j];
            index = j;
          }
        }
        maxes[i] = max;
        classes[i] = index;
      }
      return [maxes, classes];
    }

  /**
   * Dispose the tensors allocated by the model. You should call this when you
   * are done with the model.
   */
  dispose() {
    if (this.model) {
      this.model.dispose();
    }
  }
}
