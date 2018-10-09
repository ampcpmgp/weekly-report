<app-root>
  <parts-header></parts-header>
  <video ref="video" autoplay="true"></video>

  <style type="less">
    :scope {
      display: flex;
      flex-direction: column;
      border: 1px solid;
    }

    > img {
      width: 200px;
    }

    > h2 {
      color: blue;
    }
  </style>
  <script>
    import Mixin from './mixin.js'

    this.mixin(Mixin)
  </script>
</app-root>
