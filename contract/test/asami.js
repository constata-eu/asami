const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");
const SchnorrLib = artifacts.require("Schnorr");
const schnorr = require('bip-schnorr');
const assert = require('assert');
const { expectRevert, } = require('@openzeppelin/test-helpers');

const oneHour = 60 * 60;
const oneDay = oneHour * 24;
const rewardAmount = web3.utils.toWei("50", "ether");

contract("Asami Nostr", function (accounts) {
  it("Pays everyone after campaign is over if no challenges were submitted", async () => {
    const { 
      asami,
      mockDoc,
      admin,
      advertiser,
      aliceAddress,
      firstOffer
    } = await makeNostrCampaign(accounts);

    let offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert.equal(offer.rewardAddress, aliceAddress, "Offer address mismatch");
    assert(offer.state == 0, "Offer should be able to collect");
    assert(!offer.collected, "Offer should not be collected");

    const pointers = await asami.getCollectableCollaboratorOffers(aliceAddress);

    assert(
      pointers[0].length == 0 && pointers[1].length == 0,
      "Collaborator should have nothing to collect"
    );

    await expectRevert(
      asami.collectCollaboratorOffers(aliceAddress, [firstOffer], []),
      "1"
    );

    await forwardTime(oneDay * 15);

    await expectRevert(
      asami.collectCollaboratorOffers(accounts[9], [firstOffer], []),
      "2"
    );

    assert(offer.state == 0, "Offer should be able to collect");
    assert(!offer.collected, "Offer should not be collected");

    await asami.collectCollaboratorOffers(aliceAddress, [firstOffer], [], { from: admin }),

    offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(offer.collected, "Offer should be collected");
    assert.equal( (await mockDoc.balanceOf(aliceAddress)), rewardAmount, "Alice should have received the money.");
  });
});

contract("Asami Nostr", function (accounts) {
  it("A collaborator is challenged so the reward will be refunded", async () => {
    const { asami, mockDoc, admin, aliceAddress, advertiser, firstOffer } = await makeNostrCampaign(accounts);
    await makeNostrCampaign(accounts);
    await makeNostrCampaign(accounts);

    const refundables = [[0,1], [1,0], [1,1]];

    await forwardTime(oneHour);

    for( p of refundables) {
      const o = {campaignId: p[0], offerId: p[1]};
      await asami.nostrChallenge(o, { from: advertiser });
      const offer = await asami.nostrGetCampaignOffer(o);
      assert(!offer.awarded, "Offer should not be awarded");
      assert(!offer.collected, "Offer should not have been collected");
    }

    const pointers = await asami.getRefundableOffers(advertiser);
    assert(pointers.length == 3, "Should have 3 refundable offers");

    await expectRevert(
      asami.collectRefunds(advertiser, pointers, { from: admin }),
      "not_refundable_yet"
    );

    await forwardTime(oneDay * 15);

    await expectRevert(
      asami.collectRewards(aliceAddress, [{campaignId: 0, offerId: 1}], { from: admin } ),
      "this_offers_reward_was_not_awarded"
    );

    await expectRevert(
      asami.collectRefunds(admin, pointers, { from: admin }),
      "this_campaign_does_not_belong_to_that_advertiser"
    );

    await expectRevert(
      asami.collectRefunds(advertiser, [{campaignId: 0, offerId: 0}], { from: admin }),
      "awarded_offers_are_not_refundable"
    );

    await asami.collectRefunds(advertiser, pointers, { from: admin });

    for( p of refundables) {
      const offer = await asami.nostrGetCampaignOffer(
        {campaignId: p[0], offerId: p[1]}
      );
      assert(offer.collected, "Offer should have been collected");
    }

    assert.equal(
      (await mockDoc.balanceOf(advertiser)),
      rewardAmount * 3,
      "advertiser should have received 3x rewardAmount."
    );
  });
});

contract("Asami Nostr", function (accounts) {
  it("Submitting proof after a challenge awards the reward again", async () => {
    const { 
      asami,
      mockDoc,
      admin,
      advertiser,
      firstOffer
    } = await makeNostrCampaign(accounts);

    await asami.nostrChallenge(firstOffer, { from: advertiser });

    const signature = Buffer.from("7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570", "hex");
    const messageProof = {
      createdAt: 1683916923,
      sigR: signature.slice(0, 32),
      sigS: signature.slice(32, 64)
    };
    await asami.submitNostrProof(firstOffer, messageProof, { from: admin });

    offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(offer.awarded, "Offer should be awarded");
  });
});

contract("Asami Nostr", function (accounts) {
  it("Only campaign owner can challenge", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeNostrCampaign(accounts);

    await expectRevert(
      asami.nostrChallenge(firstOffer, { from: admin }),
      "only_campaign_owner_can_challenge"
    );

    await asami.nostrChallenge(firstOffer, { from: advertiser });
  });
});

contract("Asami Nostr", function (accounts) {
  it("Submits deletion proof for a previously challenged offer", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeNostrCampaign(accounts);

    await asami.nostrChallenge(firstOffer, { from: advertiser });
    const msgSig = Buffer.from("7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570", "hex");
    const messageProof = {
      createdAt: 1683916923,
      sigR: msgSig.slice(0, 32),
      sigS: msgSig.slice(32, 64)
    };
    await asami.submitNostrProof(firstOffer, messageProof, { from: admin });

    let offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(offer.awarded, "Offer should be awarded");

    await forwardTime(oneDay * 5);

    const deletionSig = Buffer.from("a5d2005d919780615df0f3a964188dc2da17ac23e0e20df0d432bb1d628655531f26140f5456cd136aab3d24bcfcd6e8e73048a166dadcfd5fcb1051fe3478e7", "hex");
    const deletionProof = {
      content: "deleted",
      tags: '[["e","34c7b3ba61e984bb370b98d67bd224c68cf6c7866837fdbf81f3b16c9b2ceee2"]]',
      createdAt: 1691886155,
      sigR: deletionSig.slice(0, 32),
      sigS: deletionSig.slice(32, 64)
    };

    await asami.submitNostrDeletionProof(firstOffer, deletionProof, { from: admin });

    offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(!offer.awarded, "Offer should not be rewarded");
  });
});

contract("Asami Nostr", function (accounts) {
  it("Submits deletion proof for a previously unchallenged offer", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeNostrCampaign(accounts);

    await forwardTime(oneDay * 5);

    let offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(offer.awarded, "Offer should be awarded");

    const msgSig = Buffer.from(
      "7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570",
      "hex"
    );
    const messageProof = {
      createdAt: 1683916923,
      sigR: msgSig.slice(0, 32),
      sigS: msgSig.slice(32, 64)
    };

    const deletionSig = Buffer.from(
      "a5d2005d919780615df0f3a964188dc2da17ac23e0e20df0d432bb1d628655531f26140f5456cd136aab3d24bcfcd6e8e73048a166dadcfd5fcb1051fe3478e7",
      "hex"
    );
    const deletionProof = {
      content: "deleted",
      tags: '[["e","34c7b3ba61e984bb370b98d67bd224c68cf6c7866837fdbf81f3b16c9b2ceee2"]]',
      createdAt: 1691886155,
      sigR: deletionSig.slice(0, 32),
      sigS: deletionSig.slice(32, 64)
    };

    await asami.submitNostrProofAndDeletionProof(firstOffer, messageProof, deletionProof, { from: admin });

    offer = await asami.nostrGetCampaignOffer(firstOffer);
    assert(!offer.awarded, "Offer should not be rewarded");
  });
});

contract("Asami Legacy", function(accounts) {
  it("runs a legacy social network campaign with mixed results", async () => {
    // 4 offers.
    // 1: pasa derecho.
    // 2: Se hace challenge, y responde bien.
    // 3: Se hace challenge, no responde.
    // 4: Se hace challenge, responde bien, después lo borra.
    // cobran: 1 y 2.
    // Refund: 3 y 4.
    // Penalty: 4.
  });
});

// -> Cannot challenge out of date.
//
// -> Can collect fees for admin.
// -> Can change the fees address.
// -> Can change the fees amount (per offer and per campaign)
//
// -> Cannot collect if it has unpaid penalties.
//  -> Anyone can pay their penalties.
//  -> So now they can collect rewards.
//
// -> Testear el renounce y el forceReward... (o esperamos a tener el state machine?)
//
// -> Can create a campaign for LSS.
//   -> Sets oracle addresses.
//   -> advertiser challenges.
//   -> Collaborator requests (and pays for) publication check.
//      -> Oracles reply yay or nay.
//      -> When the oracle threshold is met the challenge is responded.
//
//   -> somebody requests deletion check.
//      -> Esto lo paga el advertiser.
//      -> Si el oráculo resuelve que el mensaje se borró, se le pone un penalty al collab.
//      -> El penalty + 10% es a favor de quien haya pedido el deletion check.
//
// -> Asami Legacy: Múltiples oráculos.
// -> Asami Legacy: Los oráculos no responden a tiempo.
//    -> En el challenge, se resuelve para el advertiser.
//    -> En el deletion: Se cancela la campaña y se proratea.

async function makeNostrCampaign(accounts) {
  const admin = accounts[0];
  const advertiser = accounts[1];
  const aliceAddress = accounts[2];

  const asami = await Asami.deployed()
  const mockDoc = await MockDoc.deployed()

  // We will forward the blockchain time in these tests to target these timeframes.
  // Just keep in mind submissions end in 100 seconds. Payouts are in 200 seconds.
  const startDate = Math.floor(new Date().getTime() / 1000) + 100;
  const payoutDate = startDate + 100;

  const alicePubkey = "e729580aba7b4d601c94f1d9c9ba5f37e6066c22d1351ef5d49a851de81211b9";
  const aliceP = schnorr.math.liftX(Buffer.from(alicePubkey, 'hex'));
  const alicePx = schnorr.convert.intToBuffer(aliceP.affineX);
  const alicePy = schnorr.convert.intToBuffer(aliceP.affineY);

  const offers = [
    {
      "rewardAmount": rewardAmount,
      "rewardAddress": aliceAddress,
      "nostrHexPubkey": alicePubkey,
      "nostrAffineX": alicePx,
      "nostrAffineY": alicePy
    },
    {
      "rewardAmount": rewardAmount,
      "rewardAddress": aliceAddress,
      "nostrHexPubkey": alicePubkey,
      "nostrAffineX": alicePx,
      "nostrAffineY": alicePy
    }
  ];

  const fees = await asami.calculateCampaignFees(offers.length);
  const campaignAmount = fees.add(web3.utils.toBN(rewardAmount * offers.length));
  await mockDoc.transfer(advertiser, campaignAmount, { from: admin });

  // Create and fund a campaign.
  // We trick the campaign into thinking it has 2 collaborators, while it's just the same.
  // But only one of these instances will claim the payout, that way we test the refund to advertiser.
  await mockDoc.approve(asami.address, campaignAmount, { from: advertiser });
  await asami.nostrCreateCampaign(
    campaignAmount,
    startDate,
    "Remember: If anyone cancels any plan on you this weekend, they're playing the new zelda.",
    offers,
    { from: advertiser }
  );

  assert.equal( (await mockDoc.balanceOf(advertiser)), 0, "creator balance should be empty.");
  assert.equal( (await mockDoc.balanceOf(aliceAddress)), 0, "Alice balance should be empty.");

  const firstOffer = {campaignId: 0, offerId: 0};
  let offer = await asami.nostrGetCampaignOffer(firstOffer);
  assert(offer.state == 0, "Offer should start awarded");

  return {
    firstOffer,
    asami,
    mockDoc,
    advertiser,
    aliceAddress,
    admin
  };
}

async function forwardTime(seconds) {
  await web3.currentProvider.send({jsonrpc: "2.0", method: "evm_increaseTime", params: [seconds], id: new Date().getTime()}, () => {});
  await web3.currentProvider.send({jsonrpc: "2.0", method: "evm_mine", params: [], id: new Date().getTime()}, () => {});
}
