const axios = require('axios')

const trainigDir = '/tmp/random-agent-results'

async function sleep (ms) {
  return new Promise(resolve => {
    setTimeout(resolve, ms)
  })
}

async function start () {
  const response = await axios.post('http://127.0.0.1:5000/v1/envs/', {
    env_id: 'CartPole-v0' // CartPole-v0 or Breakout-v0
  })

  const response1 = await axios.get('http://127.0.0.1:5000/v1/envs/')
  console.log(1, response1.data)

  const response2 = await axios.get(
    `http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/action_space/`
  )

  console.log(2, response2.data.info)

  await axios.post(
    `http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/monitor/start/`,
    {
      directory: trainigDir,
      force: true
    }
  )

  // test用
  // await axios.post(`http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/auto_step/`)
  // return

  const episodeCount = 100 // 100
  const maxSteps = 20000

  for (let index = 0; index < episodeCount; index++) {
    await axios.post(
      `http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/reset/`
    )

    for (let index2 = 0; index2 < maxSteps; index2++) {
      await sleep(0)

      const response4 = await axios.get(
        `http://127.0.0.1:5000/v1/envs/${
          response.data.instance_id
        }/action_space/sample`
      )

      const action = response4.data.action

      const response5 = await axios.post(
        `http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/step/`,
        {
          action,
          render: true
        }
      )

      if (response5.data.done) break

      // todo: maxSteps を超えた場合は 終了 (done) するまで待つ
    }
  }

  await axios.post(
    `http://127.0.0.1:5000/v1/envs/${response.data.instance_id}/monitor/close/`
  )
  // await axios.post(`http://127.0.0.1:5000/v1/upload/`, {
  //   training_dir: trainigDir,
  //   api_key: 'TODO'
  // })
}

start()
