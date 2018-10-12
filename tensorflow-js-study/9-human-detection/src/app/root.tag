<app-root>
  <header>
    <span if="{status}">{status}</span>
    <span if="{fps}">{fps}fps</span>
    <select ref="modelName">
      <option value="lite_mobilenet_v2" selected>lite_mobilenet_v2</option>
      <option value="mobilenet_v2">mobilenet_v2</option>
    </select>
  </header>
  <section class="capture">
    <video ref="video" autoplay="true" onclick="{captureImage}"></video>
    <section>
      <img src="{image}">
      <div
        class="box"
        each="{result}"
        style="
          left: {getLeft(bbox)}%;
          top: {getTop(bbox)}%;
          width: {getWidth(bbox)}%;
          height: {getHeight(bbox)}%;
        "
        >
        {class}: {score}
      </div>
    </section>
  </section>
  <section class="selection">
    <div
      class="rectangle"
      style="
        left: {selectedRectangle.x}px;
        top: {selectedRectangle.y}px;
        width: {selectedRectangle.width}px;
        height: {selectedRectangle.height}px;
      ">
    </div>
    <canvas
      ref="captured"
      onmousedown="{startRectangleSelection}"
      onmousemove="{moveRectanglePoint}"
      onmouseenter="{mouseEnter}"
      onmouseleave="{mouseLeave}"
      onmouseup="{stopRectangleSelection}">
    </canvas>
    <canvas ref="selectedRectangleImage" alt="">
  </section>

  <style type="less">
    > section.capture {
      display: flex;

      > * {
        width: 50%;
      }

      > video {
        cursor: pointer;
      }

      > section {
        position: relative;

        > img {
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
        }

        > .box {
          color: white;
          position: absolute;
          border: solid 1px rgba(255, 255, 0, 0.6);

          &:hover {
            border: solid 2px rgba(255, 255, 0, 1);
          }
        }
      }
    }

    > section.selection {
      position: relative;

      > .rectangle {
        position: absolute;
        border: solid 2px rgba(255, 255, 0, 1);
        pointer-events: none;
      }

      > canvas {
        pointer: cursor;
      }
    }

  </style>
  <script>
    import Mixin from './mixin.js'

    this.mixin(Mixin)
  </script>
</app-root>
