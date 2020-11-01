import IPFS from "ipfs";

let node;

async function init() {
  node = await IPFS.create({ repo: String(Math.random() + Date.now()) });
  // generating 2048-bit (rsa only) RSA keypair...
}

async function start() {
  await init();
  functionMap[location.search]?.();
}

const functionMap = {
  async ""() {
    console.log(node);
  },

  "?BITSWAP"() {
    console.log("BITSWAP");
  },
};

start();
