const NostrAds = artifacts.require("NostrAds");
const schnorr = require('bip-schnorr');
const assert = require('assert');

contract("NostrAds", function (accounts) {
  it("Validates schnorr signature on nostr event", async function () {
    const sample = {
      "content": "Remember: If anyone cancels any plan on you this weekend, they're playing the new zelda.",
      "created_at": 1683916923,
      "id": "34c7b3ba61e984bb370b98d67bd224c68cf6c7866837fdbf81f3b16c9b2ceee2",
      "kind": 1,
      "pubkey": "e729580aba7b4d601c94f1d9c9ba5f37e6066c22d1351ef5d49a851de81211b9",
      "sig":    "7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570",
      "tags": []
    };

    const pubKey = Buffer.from(sample.pubkey, 'hex');
    const signature = Buffer.from(sample.sig, 'hex');
    const message = Buffer.from(sample.id, 'hex');

    const P = schnorr.math.liftX(pubKey);
    const Px = schnorr.convert.intToBuffer(P.affineX);
    const Py = schnorr.convert.intToBuffer(P.affineY);
    const r = signature.slice(0, 32);
    const s = signature.slice(32, 64);
    
    let contract = await NostrAds.deployed();
    const tx = await contract.verify(message, Px, Py, r, s);
    assert.ok(tx, "Invalid Signature");

    const gasEstimate = await contract.verify.estimateGas(message, Px, Py, r, s);
    console.log('Gas estimate:', gasEstimate);

  });
});
