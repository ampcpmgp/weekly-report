import { observe } from 'dob'
import ui, * as uiAction from '../../../state/ui'

export default {
  clickCount: ui.clickCount,
  clickHistory: ui.clickHistory,
  addCount: uiAction.addCount,

  init () {
    const signal = observe(() => {
      this.update({
        clickCount: ui.clickCount
      })
    })

    this.on('unmount', () => {
      signal.unobserve()
    })
  }
}
