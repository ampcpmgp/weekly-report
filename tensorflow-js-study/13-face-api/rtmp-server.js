const { NodeMediaCluster } = require('node-media-server')
const numCPUs = require('os').cpus().length
const config = {
  rtmp: {
    port: 1935,
    chunk_size: 60000,
    gop_cache: true,
    ping: 60,
    ping_timeout: 30
  },
  http: {
    port: 8000,
    allow_origin: '*'
  },
  cluster: {
    num: numCPUs
  }
}

var nmcs = new NodeMediaCluster(config)
nmcs.run()
