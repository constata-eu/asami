const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");
const SchnorrLib = artifacts.require("Schnorr");
const schnorr = require('bip-schnorr');
const assert = require('assert');
const { expectRevert, } = require('@openzeppelin/test-helpers');
const crypto = require('crypto');

const oneHour = 60 * 60;
const oneDay = oneHour * 24;
const rewardAmount = web3.utils.toWei("50", "ether");

const toWei = web3.utils.toWei;

contract("Asami", function (accounts) {
  it("Signs up accounts, makes a campaign, has some people participate", async () => {
    const {
      asami,
      doc,
      aliceId,
      bobId,
      carolId,
      dennisId,
      admin,
    } = await setupAccounts(accounts);

    let aliceHandle = await makeNostrHandle(asami, admin, aliceId, "100", 200, [1, 2, 3]);
    let bobHandle = await makeNostrHandle(asami, admin, bobId, "200", 200, [1, 4, 5]);
    let carolHandle = await makeNostrHandle(asami, admin, carolId, "300", 200, [1, 6, 7]);
    let campaign = await makeCampaign(
      asami,
      doc,
      admin,
      dennisId,
      1,
      toWei("700"),
      "1716421161867710954",
      toWei("1.5"),
      [1]
    );

    let aliceCollab = await makeCollab(asami, admin, aliceHandle.id, campaign.id);
    let bobCollab = await makeCollab(asami, admin, bobHandle.id, campaign.id);
    let carolCollab = await makeCollab(asami, admin, carolHandle.id, campaign.id);

    assert.equal(await asami.feePool(), toWei("60"));
    assert.equal((await asami.accounts(aliceId)).unclaimedAsamiTokens, toWei("1.5"));
    assert.equal((await asami.accounts(bobId)).unclaimedAsamiTokens.toString(), toWei("3"));
    assert.equal((await asami.accounts(carolId)).unclaimedAsamiTokens.toString(), toWei("4.5"));
  });
});

async function makeCollab(asami, admin, handleId, campaignId) {
  return (await asami.adminMakeCollabs(
    [{ handleId, campaignId }],
    { from: admin }
  )).receipt.logs.find((x) => x.event == "CollabSaved").args.collab;
}

async function makeCampaign(
  asami,
  doc,
  admin,
  accountId,
  site,
  budget,
  contentId,
  priceScoreRatio,
  topics,
  validUntil
) {
  await doc.approve(asami.address, budget, { from: admin });

  return (await asami.adminMakeCampaigns(
    [{
      accountId,
      attrs: {
        site,
        budget,
        contentId,
        priceScoreRatio,
        topics,
        validUntil: validUntil || Math.floor(new Date().getTime() / 1000) + 5000,
      }
    }],
    { from: admin }
  )).receipt.logs.find((x) => x.event == "CampaignSaved").args.campaign;
}

async function makeNostrHandle(asami, admin, accountId, price, score, topics) {
  const nostrPubkey = "e729580aba7b4d601c94f1d9c9ba5f37e6066c22d1351ef5d49a851de81211b9";
  const nostrP = schnorr.math.liftX(Buffer.from(nostrPubkey, 'hex'));
  const nostrPx = schnorr.convert.intToBuffer(nostrP.affineX);
  const nostrPy = schnorr.convert.intToBuffer(nostrP.affineY);

  return (await asami.adminMakeHandle({
    id: 0,
    accountId: accountId,
    site: 1,
    price: toWei(price),
    score: score,
    topics: topics || [],
    username: "",
    userId: nostrPubkey,
    nostrAffineX: nostrPx,
    nostrAffineY: nostrPy
  }, { from: admin })).receipt.logs[0].args.handle;
}

async function makeXHandle(asami, accountId, xHandle, xUserId, price, score, topics) {
  return (await asami.adminMakeHandle({
    id: 0,
    accountId,
    site: 0,
    price: web3.utils.toWei(price, "ether"),
    score,
    topics,
    xHandle,
    xUserId,
    instagramHandle: "",
    instagramUserId: "",
    nostrHexPubkey: "",
    nostrAffineX: 0,
    nostrAffineY: 0,
    nostrSelfManaged: false,
    nostrAbuseProven: false
  })).receipt.logs[0].args.handle;
}

async function setupAccounts(accounts) {
  const admin = accounts[8];
  const adminTreasury = accounts[9];
  const asami = await Asami.deployed()
  const doc = await MockDoc.deployed()

  await doc.transfer(admin, web3.utils.toWei("10000", "ether"), { from: accounts[0] });
  await asami.setAdmin(admin, adminTreasury);

  for (let s of [ "Cat", "Dog", "Bat", "Sun", "Pen", "Car", "Hat" ] ) {
    await asami.addTopic(s);
  }

  /* User tries to sign up.
   * We create account ids
   *
   */
  //const fees = await asami.calculateCampaignFees(offers.length);
  //const campaignAmount = fees.add(web3.utils.toBN(rewardAmount * offers.length));
  //await doc.transfer(advertiser, campaignAmount, { from: admin });

  // Create and fund a campaign.
  // We trick the campaign into thinking it has 2 collaborators, while it's just the same.
  // But only one of these instances will claim the payout, that way we test the refund to advertiser.
  /*
  await doc.approve(asami.address, campaignAmount, { from: advertiser });
  await asami.nostrCreateCampaign(
    campaignAmount,
    startDate,
    "Remember: If anyone cancels any plan on you this weekend, they're playing the new zelda.",
    offers,
    { from: advertiser }
  );

  assert.equal( (await doc.balanceOf(advertiser)), 0, "creator balance should be empty.");
  assert.equal( (await doc.balanceOf(aliceAddress)), 0, "Alice balance should be empty.");

  const firstOffer = {campaignId: 0, offerId: 0};
  let offer = await asami.nostrGetCampaignOffer(firstOffer);
  assert(offer.state == 0, "Offer should start awarded");

  return {
    firstOffer,
    asami,
    doc,
    advertiser,
    aliceAddress,
    admin
  };
  */
  return {
    asami,
    doc,
    aliceId: 1,
    bobId: 2,
    carolId: 3,
    dennisId: 4,
    admin,
    adminTreasury
  };
}

async function forwardTime(seconds) {
  await web3.currentProvider.send({jsonrpc: "2.0", method: "evm_increaseTime", params: [seconds], id: new Date().getTime()}, () => {});
  await web3.currentProvider.send({jsonrpc: "2.0", method: "evm_mine", params: [], id: new Date().getTime()}, () => {});
}


/*
  it("Pays everyone after campaign is over if no challenges were submitted", async () => {
    const { 
      asami,
      doc,
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
    assert.equal( (await doc.balanceOf(aliceAddress)), rewardAmount, "Alice should have received the money.");
  });
});

contract("Asami Nostr", function (accounts) {
  it("A collaborator is challenged so the reward will be refunded", async () => {
    const { asami, doc, admin, aliceAddress, advertiser, firstOffer } = await makeNostrCampaign(accounts);
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
      (await doc.balanceOf(advertiser)),
      rewardAmount * 3,
      "advertiser should have received 3x rewardAmount."
    );
  });
});

contract("Asami Nostr", function (accounts) {
  it("Submitting proof after a challenge awards the reward again", async () => {
    const { 
      asami,
      doc,
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
    const admin = accounts[0];
    const advertiser = accounts[1];
    const aliceAddress = accounts[2];

    const asami = await Asami.deployed()
    for (let s of ["Instagram", "Twitter", "Youtube", "Facebook", "TikTok"]) {
      await asami.addSocialNetwork(s);
    }

    const doc = await MockDoc.deployed()

    // We will forward the blockchain time in these tests to target these timeframes.
    // Just keep in mind submissions end in 100 seconds. Payouts are in 200 seconds.
    const startDate = Math.floor(new Date().getTime() / 1000) + 100;
    const payoutDate = startDate + 100;

    const offers = [
      {
        "rewardAmount": rewardAmount,
        "rewardAddress": aliceAddress,
        "username": "@alice_alison",
      },
      {
        "rewardAmount": rewardAmount,
        "rewardAddress": aliceAddress,
        "username": "@bob_bobson",
      }
    ];

    const fees = await asami.calculateCampaignFees(offers.length);
    const campaignAmount = fees.add(web3.utils.toBN(rewardAmount * offers.length));
    await doc.transfer(advertiser, campaignAmount, { from: admin });

    // Create and fund a campaign.
    // We trick the campaign into thinking it has 2 collaborators, while it's just the same.
    // But only one of these instances will claim the payout, that way we test the refund to advertiser.
    await doc.approve(asami.address, campaignAmount, { from: advertiser });
    const rulesUrl = "tbd";
    const hash = crypto.createHash("sha256");
    hash.update(rulesUrl);
    const buffer = Buffer.from(hash.digest(), "hex");

    await asami.classicCreateCampaign({
      funding: campaignAmount.toString(),
      startDate: startDate,
      socialNetworkId: 1,
      rulesUrl: "a_long_rules_url",
      rulesHash: buffer,
      oracleAddress: "0xF78A30A396738Ce1a7ea520F707477F8430b9D51",
      oracleFee: web3.utils.toWei("50", "ether"),
      newOffers: offers
    }, { from: advertiser });

    assert.equal( (await doc.balanceOf(advertiser)), 0, "creator balance should be empty.");
    assert.equal( (await doc.balanceOf(aliceAddress)), 0, "Alice balance should be empty.");

    const firstOffer = {campaignId: 0, offerId: 0};
    let offer = await asami.classicGetCampaignOffer(firstOffer);
    assert(offer.state == 0, "Offer should start awarded");

    return {
      firstOffer,
      asami,
      doc,
      advertiser,
      aliceAddress,
      admin
    };
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
*/

/*

Not everyone can participate:
- A collaboration with a handle that is more expensive is not possible.
- A collaboration with someone who's missing the right tags is impossible.
- When the campaign is over-subscribed nobody can participate.

A handle with no tags can participate in a campaign with no tags.

Updating profiles:
- One user tries to update their profile.
- One user claims their account.
- Admin cannot schedule an update for them.
- They can schedule their own update.
- Then all updates are applied.
- The last update period is set.
- Trying to apply updates again fails.

Using self-service nostr:
- A nostr campaign is set up.
- One of the three members has claimed their account, and decided to report their own nostr messages their nostr handle.
- The user submits their own nostr account.

People who claimed their account can also vote.
  - Votes are weighted by their token balance. !!!
  - Anyone with tokens can vote, no need to be an account.
  - After the vote, the tally is calculated and the feeRate changes.

People can vote a new admin.
- Admin gets paid a fixed amount for each new account and new collaboration.
- After the admin gets paid that amount, the fees are distributed.
- If the fees do not cover the expenses the admin does not get owed anything though.
- People can vote for a new admin. If the new admin gets over 50% of the voting power for 3 consecutive periods, the admin changes.

Rogue admin attacks:
- Can stop accepting new accounts.
  Advertisers may not like the low selection of accounts, which will make the protocol fail,
  token holders may not like it either.


What's data protection and privacy like here then?
  - Everyone will know how much a handle got paid in rewards.

  - The smart contract will keep track of which handle received payments,
    this requirement is legitimized by the need of transparency towards
    advertisers and other members of the community.

  - Once on the blockchain, it will be stored on a number of third party servers, some
    of them in jurisdictions where no data protection guarantees can be enforced.
    Exercise caution, and feel free to use a pseudonym, 

    However, when there is no adequacy decision for a particular country, Article 49 of the GDPR provides for a number of derogations under which data may still be transferred. These derogations serve as exceptions to the general rule and can include:

Explicit Consent: The data subject has explicitly consented to the transfer, after being informed of the possible risks.

Contractual Necessity: The transfer is necessary for the performance of a contract between the data subject and the controller, or for pre-contractual steps taken at the data subject's request.

Important Reasons of Public Interest: Data can be transferred if it's based on EU or Member State law, with a clear basis in law and proportionate to the aim pursued.

Legal Claims: The transfer is necessary for the establishment, exercise, or defense of legal claims.

Vital Interests: If it's necessary to protect the vital interests of the data subject or other persons, where the data subject is physically or legally incapable of giving consent.

Occasional Transfers: There is a provision for non-repetitive transfers that only affect a limited number of data subjects, as long as the controller has assessed all the circumstances surrounding the data transfer and has provided suitable safeguards.

The use of these derogations is generally intended to be extraordinary, and they are not meant for routine or mass transfers. Moreover, organizations relying on any of these derogations may need to document their assessments and safeguards to prove compliance with GDPR requirements.

Please consult with a legal advisor for guidance tailored to your specific situation.

  - The admin may be contacted to delete and further ignore personal details stored in the
    smart contract for causes that supersede the community legitimate interest.
    This requests will be honored on its website.

*/
