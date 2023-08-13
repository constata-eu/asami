const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");
const SchnorrLib = artifacts.require("Schnorr");
const schnorr = require('bip-schnorr');
const assert = require('assert');
const { expectRevert, } = require('@openzeppelin/test-helpers');

const oneDay = 60 * 60 * 24;
const rewardAmount = web3.utils.toWei("50", "ether");

contract("Asami", function (accounts) {
  it("Pays everyone after campaign is over if no challenges were submitted", async () => {
    const { 
      asami,
      mockDoc,
      admin,
      advertiser,
      aliceAddress,
      firstOffer
    } = await makeCampaign(accounts);

    let offer = await asami.getCampaignOffer(firstOffer);
    assert.equal(offer.rewardAddress, aliceAddress, "Offer address mismatch");
    assert(offer.awarded, "Offer should be able to collect");
    assert(!offer.collected, "Offer should not be collected");

    const pointers = await asami.getRewardsPendingCollection(aliceAddress);

    await expectRevert(
      asami.collectRewards(aliceAddress, pointers),
      "campaign_is_not_over_yet"
    );

    await forwardTime(oneDay * 15);

    await expectRevert(
      asami.collectRewards(accounts[9], pointers),
      "that_offer_is_not_for_this_collaborator"
    );

    assert(offer.awarded, "Offer should be able to collect");
    assert(!offer.collected, "Offer should not be collected");

    await asami.collectRewards(aliceAddress, pointers, { from: admin } );
    offer = await asami.getCampaignOffer(firstOffer);
    assert(offer.collected, "Offer should be collected");
    assert.equal( (await mockDoc.balanceOf(aliceAddress)), rewardAmount, "Alice should have received the money.");
  });
});

contract("Asami", function (accounts) {
  it("A collaborator is challenged so the reward will be refunded", async () => {
    const { asami, mockDoc, admin, aliceAddress, advertiser, firstOffer } = await makeCampaign(accounts);
    await makeCampaign(accounts);
    await makeCampaign(accounts);

    const refundables = [[0,1], [1,0], [1,1]];

    for( p of refundables) {
      const o = {campaignId: p[0], offerId: p[1]};
      await asami.challenge(o, { from: advertiser });
      const offer = await asami.getCampaignOffer(o);
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
      const offer = await asami.getCampaignOffer(
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

contract("Asami", function (accounts) {
  it("Submitting proof after a challenge awards the reward again", async () => {
    const { 
      asami,
      mockDoc,
      admin,
      advertiser,
      firstOffer
    } = await makeCampaign(accounts);

    await asami.challenge(firstOffer, { from: advertiser });

    const signature = Buffer.from("7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570", "hex");
    const messageProof = {
      createdAt: 1683916923,
      sigR: signature.slice(0, 32),
      sigS: signature.slice(32, 64)
    };
    await asami.submitProof(firstOffer, messageProof, { from: admin });

    offer = await asami.getCampaignOffer(firstOffer);
    assert(offer.awarded, "Offer should be awarded");
  });
});

contract("Asami", function (accounts) {
  it("Only campaign owner can challenge", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeCampaign(accounts);

    await expectRevert(
      asami.challenge(firstOffer, { from: admin }),
      "only_campaign_owner_can_challenge"
    );

    await asami.challenge(firstOffer, { from: advertiser });
  });
});

contract("Asami", function (accounts) {
  it("Submits deletion proof for a previously challenged offer", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeCampaign(accounts);

    await asami.challenge(firstOffer, { from: advertiser });
    const msgSig = Buffer.from("7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570", "hex");
    const messageProof = {
      createdAt: 1683916923,
      sigR: msgSig.slice(0, 32),
      sigS: msgSig.slice(32, 64)
    };
    await asami.submitProof(firstOffer, messageProof, { from: admin });

    let offer = await asami.getCampaignOffer(firstOffer);
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

    await asami.submitDeletionProof(firstOffer, deletionProof, { from: admin });

    offer = await asami.getCampaignOffer(firstOffer);
    assert(!offer.awarded, "Offer should not be rewarded");
  });
});

contract("Asami", function (accounts) {
  it("Submits deletion proof for a previously unchallenged offer", async () => {
    const { 
      asami,
      admin,
      advertiser,
      firstOffer
    } = await makeCampaign(accounts);

    await forwardTime(oneDay * 5);

    let offer = await asami.getCampaignOffer(firstOffer);
    assert(offer.awarded, "Offer should be awarded");

    const msgSig = Buffer.from("7aa5f6c2cd24f6fd63475fca4d632bf37ef13eb44140f11fcc52f0daabdd6f9e5054148ae0a27315eb0da4642b59ac4e9c83748136751773bf95c7992ea5d570", "hex");
    const messageProof = {
      createdAt: 1683916923,
      sigR: msgSig.slice(0, 32),
      sigS: msgSig.slice(32, 64)
    };

    const deletionSig = Buffer.from("a5d2005d919780615df0f3a964188dc2da17ac23e0e20df0d432bb1d628655531f26140f5456cd136aab3d24bcfcd6e8e73048a166dadcfd5fcb1051fe3478e7", "hex");
    const deletionProof = {
      content: "deleted",
      tags: '[["e","34c7b3ba61e984bb370b98d67bd224c68cf6c7866837fdbf81f3b16c9b2ceee2"]]',
      createdAt: 1691886155,
      sigR: deletionSig.slice(0, 32),
      sigS: deletionSig.slice(32, 64)
    };

    await asami.submitProofAndDeletionProof(firstOffer, messageProof, deletionProof, { from: admin });

    offer = await asami.getCampaignOffer(firstOffer);
    assert(!offer.awarded, "Offer should not be rewarded");
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

async function makeCampaign(accounts) {
  const admin = accounts[0];
  const advertiser = accounts[1];
  const aliceAddress = accounts[2];

  const asami = await Asami.deployed()
  const mockDoc = await MockDoc.deployed()

  // We will forward the blockchain time in these tests to target these timeframes.
  // Just keep in mind submissions end in 100 seconds. Payouts are in 200 seconds.
  const submissionDeadline = Math.floor(new Date().getTime() / 1000) + 100;
  const payoutDate = submissionDeadline + 100;

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

  const campaignAmount = await asami.getCampaignCost(offers);

  await mockDoc.transfer(advertiser, campaignAmount, { from: admin });

  // Create and fund a campaign.
  // We trick the campaign into thinking it has 2 collaborators, while it's just the same.
  // But only one of these instances will claim the payout, that way we test the refund to advertiser.
  await mockDoc.approve(asami.address, campaignAmount, { from: advertiser });
  const campaignIndex = await asami.createCampaign(
    "Remember: If anyone cancels any plan on you this weekend, they're playing the new zelda.",
    campaignAmount,
    submissionDeadline,
    offers,
    { from: advertiser }
  );

  // Verify campaign creation
  const campaign = await asami.campaigns(0);
  assert.equal(campaign.funding.toString(), campaignAmount, "Reward funding mismatch");

  assert.equal( (await mockDoc.balanceOf(advertiser)), 0, "creator balance should be empty.");
  assert.equal( (await mockDoc.balanceOf(aliceAddress)), 0, "Alice balance should be empty.");

  const firstOffer = {campaignId: 0, offerId: 0};
  let offer = await asami.getCampaignOffer(firstOffer);
  assert(offer.awarded, "Offer should start awarded");

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
