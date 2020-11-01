import IPFS from "ipfs";

const nodeP = createNode();

async function createNode() {
  const node = await IPFS.create({ repo: String(Math.random() + Date.now()) });
  // generating 2048-bit (rsa only) RSA keypair...

  return node;
}

async function start() {
  await functionMap[location.search]?.();
}

const functionMap = {
  async ""() {
    const node = await nodeP;
    const version = await node.version();

    console.log(version);
  },

  "?BITSWAP"() {
    console.log("BITSWAP");
  },
};

start();
