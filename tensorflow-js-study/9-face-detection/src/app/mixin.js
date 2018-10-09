import { observe } from 'dob'
import axios from 'axios'
import ui from '../state/ui'
import Path from '../const/path'

export default {
  title: 'Hello, World',
  apiData: null,
  clickCount: ui.clickCount,

  init () {
    const signal = observe(() => {
      this.update({
        clickCount: ui.clickCount
      })
    })

    this.on('mount', () => {
      this.getApi()
    })

    this.on('unmount', () => {
      signal.unobserve()
    })
  },

  async getApi () {
    try {
      const { data } = await axios.get(Path.TEST)
      this.apiData = JSON.stringify(data)
      this.update()
    } catch (e) {
      console.warn('api error')
    }
  }
}
