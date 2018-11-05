# セットアップ

Open AI Gym のインストールをします。

windows 10での参考記事 - https://qiita.com/antimon2/items/b1611dca09edcf93db03

## Opem AI gym binding for node.js

まず、vcxsrvを起動しておきます。(winユーザーのみ)

### python server
```shell
git clone https://github.com/ampcpmgp/gym-http-api.git other-git--gym-http-api
cd other-git--gym-http-api

wsl # for windows
pip3 install -r requirements.txt

python3 ./gym_http_server.py
```

### node.js client
```shell
yarn

npm run sample
```
