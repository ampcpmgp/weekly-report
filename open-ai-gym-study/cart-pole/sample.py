import gym
import numpy as np

env = gym.make('CartPole-v0')

goal_average_steps = 195
max_number_of_steps = 200
num_consecutive_iterations = 100
num_episodes = 5000
last_time_steps = np.zeros(num_consecutive_iterations)

for episode in range(num_episodes):
    # 環境の初期化
    observation = env.reset()

    episode_reward = 0
    for t in range(max_number_of_steps):
        # CartPoleの描画
        env.render()

        # ランダムで行動の選択
        action = np.random.choice([0, 1])

        # 行動の実行とフィードバックの取得
        observation, reward, done, info = env.step(action)
        episode_reward += reward

        if done:
            print('%d Episode finished after %f time steps / mean %f' % (episode, t + 1,
                last_time_steps.mean()))
            last_time_steps = np.hstack((last_time_steps[1:], [episode_reward]))
            break

    if (last_time_steps.mean() >= goal_average_steps): # 直近の100エピソードが195以上であれば成功
        print('Episode %d train agent successfuly!' % episode)
        break
