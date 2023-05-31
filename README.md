# REWARDS ON RSK FOR POSTING ON NOSTR.
_And a general in-bound data service for RSK from NOSTR._

***

Nostr is a messaging network like Twitter, supporting various use cases, but with a focus on micro-blogging. Unlike Twitter or Facebook, Nostr doesn't use a centralized algorithm to boost specific content or ads. Instead, peers donate to quality content using Lightning Network through a protocol extension called Zaps.

This direct economic reward for quality content, and the absense of a centralized ad network, means that bot armies that fake engagement and discourse are unnecessary. Follower counts and impressions become less important, making it harder for a few users to hoard followers.

But as Nostr grows, the scattered attention it captures becomes valuable for advertising.

We plan to use transparent market mechanisms to drive attention where it's most valuable and build a decentralized ad network. Anyone can advertise or boost their content by paying peers to post, like, or repost.

Nostriches can take on the role of advertisers of various sizes, they decide which product gets the spotlight and how often they want to participate in ads. The rewards they receive are public, which may help with holding people accountable for what they advertise.

## PRIOR ART

Some users have created robots to send zaps to users who post something or comment on a specific post. However, these interactions are not seen as a contractual obligation, and there is no guarantee of payment if a bot breaks, runs out of money, or misses a message.

The centralized "pay-for-publicity" bot approach has several shortcomings: Participants may not be paid if the robot runs out of money or stops working, messages may be missed due to network splits, and all participants are paid the same even if some have better reach. Additionally, robots are vulnerable to sybil attacks and spam, and only Lightning Network Bitcoin is supported, which is susceptible to high fees on the Bitcoin network. No stablecoins are supported yet.

## PLAN &amp; RATIONALE

We plan to create tools that enable transparent and automated advertising campaigns on Nostr, using smart contracts to ensure that Nostr users are paid for their participation and advertisers only pay for the publicity they receive. Our tools will also include an open appraisal algorithm and database to assess the value of a Nostr user's account, backoffice tools for advertisers, a customized Nostr relay, and a chatbot for advertisers to communicate with Nostr users.

We have chosen RSK as the smart contract platform for Nostr due to its popularity among Bitcoin users, who make up a significant portion of Nostr's user base. Using well-known units of account like RBTC and DOC stablecoin will provide certainty when budgeting and assessing liquidity or cash-out options.

Our team has experience with the RSK ecosystem and decentralized technology, and we have been eager to work with Nostr for some time. We plan to fund campaigns for popular RSK projects at no charge, and in the future, we may provide technology for advertisers for a fee and participate in their campaigns.

Our account "appraisal" algorithm will consist of mapping and weighting the entire nostr social graph. This kind of large dataset is costly to build and maintain, and can possibly be rented out in several ways to continue funding the project.

This is the first step of a deeper integration between Nostr and RSK, with the ultimate goal of building a bridge that allows anyone to use natural language messages in a decentralized network to interact with RSK, rather than relying on centralized http and browsers.

## THE SMART CONTRACT

The smart contract represents an advertisement campaign, acting as an escrow between Advertisers and Participants. To prevent spam, Nostriches must be pre-approved (appraised) by the Advertiser.

The worflow is as follows:

### Step 1
The Advertiser creates a Campaign, by deploying the contract which should have:
  - A template Nostr post, which is the content and tags participants should post.
  - An event id and reccommended relay for a message announcing this campaign. (This event ID will be known as the campaign id).
  - An entry limit date: Participants may claim their reward before this date.
  - An end date for the campaign, which is when participants get paid and may delete the post if they wish.
  - A list of hex encoded pubkeys belonging to the advertiser and helpers.
  - The funds to pay participants, which may be increased at any point by the advertiser.

The contract also keeps a list of all the rewards claimed.

### Step 2
A Nostrich willing to participate requests an appraisal from the Advertiser, this is done off-chain, via nostr direct messages.
The appraisal is digitally signed by the advertiser, non transferable and usable only in this specific campaign.
The Nostrich may suggest the reward they expect to receive for participating.
An appraisal may be denied if the Advertiser deems the Nostrich account to be fraudulent or not in the campaign audience target.
An appraisal does not lock funds in the contract, participants are encouraged to not speculate and post as soon as possible.

An appraisal may be requested with a type 1 note sent to the advertiser.
```
{
  "id": "...",
  "pubkey": <Participant pubkey>,
  "created_at": "...",
  "kind": 1,
  "tags": [
    ["p", <pubkey of the advertiser>],
    ...
  ],
  "content": "<any text><any whitespace><participants payout account><any whitesapce><campaign id>",
  "sig": "...",
}
```

The appraisal is sent back as a reply event to that message.
```
{
  "id": "...",
  "pubkey": <the advertiser pubkey>,
  "created_at": "...",
  "kind": 1,
  "tags": [
    ["p", <pubkey of the participant>],
    ["e", <event id of the appraisal request>],
    ["t", "accepted"], # Only if the appraisal was accepted.
    ["amount", <the reward amount>], # Only if the appraisal was accepted.
  ],
  "content": <Further instructions from the advertiser or a rejection reason>,
  "sig": "...",
}
```

### Step 3
The Nostrich makes a post following the campaign's instructions for content and tags, then invokes the contracts "claim reward" function,
which receives the JSON payload of the three events:
  - The appraisal request.
  - The appraisal.
  - The post event json.

The contract verifies the received events and adds the claim amount and rsk payout account to the claimed reward list.

Claim may be denied if: 
  - The signatures are wrong, or the three events are incoherent.
  - The Nostrich already claimed a reward in this campaign.
  - The entry limit date is over.

The Advertiser monitors claims to further relay the post. 
The Advertiser also monitors relevant public relays for delete events on the claimed post.
Early deletions can be reported to the contract, and will remove the participant's claim.

Appraisals do not lock funds to prevent DoS &mdash; participants should check the contract's balance before issuing a claim to avoid this.
A successful campaign may receive claims in excess of its funding, if so, participants may either delete their post, or may wait and see if the advertiser increases the campaign budget.
  
### Step 4
At the end date the contract pays the participants in the same order the claims were received, and they may delete their post.


### IMPROVED UX FOR NOSTR USERS

Both Constata and Advertisers will work on improving the Nostr User user experience.

The advertiser has their own Nostr account where they announce new campaigns and provide instructions on how to participate. They may also directly invite relevant participants to the campaign.

Participants can also participate in a campaign without requesting an appraisal if they are willing to do so out of goodwill. In this case, the advertiser can message the participant with an appraisal message and wait for the participant's RSK account as a reply.

Additionally, the advertiser can monitor all accounts that have requested an appraisal and offer to claim the bounty on their behalf as soon as they detect that the required message has been posted. The contract will provision a refund of transaction fees deducted from the participant's reward.

Finally, Constata will provide a web frontend that allows any user to claim rewards on any campaign from their own wallet. The participant only needs to enter their pubkey, and the tool will scan for rewards they may be ready to claim.

### ADVERTISERS DELEGATING I.T. WORK

The advertiser may choose to delegate administration of the smart contract to a third party, which creates a new trust relationship. To mitigate trust issues, potential solutions include a deposit made by the service provider, a reputation system for both parties, or involvement of an arbitration court. However, these solutions are not part of the initial version of this proposal. Nevertheless, the contract will assume the advertiser is not a single RSK account and allow appraisals to be done by pre-approved third parties. Additionally, tools will be provided to allow advertisers to run their own infrastructure to avoid the need for delegation.

### CAVEATS AND OPEN QUESTIONS

##### We'll try to make do with current nostr event types, but may need to implement new ones.
The Nostr events are not optimized for smart contracts, but they are the native format for Nostr clients. The proposed protocols prioritize usability and compatibility, allowing Nostriches to use their existing clients and event types. This could result in higher costs and a less attractive appearance. Using type 1 Nostr events for appraisals may create unnecessary noise in the advertiser and participant's timeline. Private direct messages cannot be used for smart contract validation. We are exploring ways to make these public messages less noisy, possibly through a new NIP.

##### On-chain verification of nostr messages is expensive. We may need to build an oracle.
Nostr signs using Schnorr Signatures, which are expensive to verify on-chain. Our (yet naive) implementation spends 2300801 gas for a single message verification, which is currently around 4 USD with a gas price of 0.06 Gwei. That cost is prohibitive for micro payment campaigns. If we can't get the on-chain verification cost significantly down, we will need to create a distributed oracle for nostr messages. The interactions with the oracle will still resemble the protocol described above for the on-chain contract.

#### Nostr uses Schnorr signatures, we can have MuSig chatbots from the start.
At the protocol level, Nostr supports multisig, so whenever we mention a chatbot, we can work on making them multi-sig and run by a federation. But nostr does not provide other types of signature scripting, timelocks and key rotation or recovery, which call for a new NIP.
