<app-root>
  <header>
    <span if="{status}">{status}</span>
    <span if="{fps}">{fps}fps</span>
  </header>
  <section class="capture">
    <video ref="video" autoplay="true"></video>
    <section>
      <img src="{image}">
      <div
        class="box"
        each="{result}"
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

  <style type="less">
    >section.capture {
      display: flex;

      > section {
        position: relative;

        > .box {
          color: white;
          position: absolute;
          border: solid 1px yellow;
        }
      }
    }
  </style>
  <script>
    import Mixin from './mixin.js'

    this.mixin(Mixin)
  </script>
</app-root>
