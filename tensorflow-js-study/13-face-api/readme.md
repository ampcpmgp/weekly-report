# 使い方

TODO: 上手く動かなかったため、動作確認するをする

https で ライブリロードしたほうが効率が良いため、httpsで使えるようにする。

下記はオレオレ証明書の作り方

```shell
openssl genrsa 2048 > server.key
openssl req -new -key server.key > server.csr
openssl x509 -days 3650 -req -signkey server.key < server.csr > server.crt
```

上記で生成された二つのファイルを、httpsサーバーに設定する。

