import { observable, Action } from 'dob'

const ui = observable({
  clickCount: 0,
  clickHistory: []
})

export const addCount = () =>
  Action(() => {
    ++ui.clickCount
    ui.clickHistory.push(1)
  })

export default ui
