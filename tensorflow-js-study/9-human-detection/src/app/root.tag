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
    <video ref="video" autoplay="true"></video>
    <section>
      <img src="{image}">
      <div
        class="box"
        each="{result}"
        onclick="{() => captureImage(bbox)}"
        style="
          left: {bbox[0]}px;
          top: {bbox[1]}px;
          width: {bbox[2]}px;
          height: {bbox[3]}px;
        "
        >
        {class}: {score}
      </div>
    </section>
  </section>
  <canvas ref="captured"></canvas>

  <style type="less">
    >section.capture {
      display: flex;

      > section {
        position: relative;

        > .box {
          color: white;
          position: absolute;
          border: solid 1px rgba(255, 255, 0, 0.6);
          cursor: pointer;

          &:hover {
            border: solid 2px rgba(255, 255, 0, 1);
          }
        }
      }
    }
  </style>
  <script>
    import Mixin from './mixin.js'

    this.mixin(Mixin)
  </script>
</app-root>
