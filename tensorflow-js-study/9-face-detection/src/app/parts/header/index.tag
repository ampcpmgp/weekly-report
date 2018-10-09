<parts-header>
  <span onclick={addCount}>menu</span>
  <span>{clickCount} / {clickHistory.length}</span>
  <style type="less">
    :scope {
      display: flex;
      flex-direction: column;
      width: 100%;
      color: white;
      background-color: #555;
    }
  </style>
  <script>
    import Mixin from './mixin.js'
    this.mixin(Mixin)
  </script>
</parts-header>
