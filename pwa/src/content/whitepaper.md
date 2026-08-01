# Asami Club Whitepaper

This whitepaper describes the current design and intended direction of Asami.club as of May 2026.

---

## 1. Overview

Asami.club is the club for Rootstock communities that appreciate their members.

Its purpose is simple: help communities recognize people who already create value for them. Sometimes that value is public advocacy. Sometimes it is participation, curation, trust, or long-term support. Asami exists to make that recognition transparent, modest, and economically legible on Rootstock.

The club is an invitation-first network centered on existing real projects, relationships, community knowledge, and reputation.

Every accepted club member is a **Rootstocker**. A Rootstocker may be:

* a community,
* a representative acting for a community,
* an individual community member,
* or any combination of the above over time.

The same Rootstocker may create campaigns, participate in campaigns, hold ASAMI, invite others into the club, and help evaluate other members.

Asami is built on **Rootstock** and uses **DOC** as its reward currency. The smart contract remains intentionally simple: it receives campaign funds, allows campaign administrators to register collaborations and payouts, and issues ASAMI according to protocol rules. New product behavior is implemented off-chain by changing how campaigns are administered, not by changing the contract.

---

## 2. History and Current Direction

Asami began as an experiment in recognizing online amplification. The original intuition was correct: people who explain projects, recommend them, defend them, and help them spread really do create value.

But the first live product versions also revealed a problem. If membership is too open and rewards are framed too transactionally, the system attracts extractive behavior. People optimize for tasks and payouts rather than alignment, reputation, or durable community membership.

That is why Asami changed direction.

The current product direction is:

* membership is mostly by invitation,
* waitlist applications exist but do not imply acceptance,
* communities should be able to recognize members they already know,
* repost campaigns remain useful, but they are only one campaign type,
* score is based on qualified member judgment, not automated social metrics.

In short, Asami is Rootstock’s club for community appreciation.

---

## 3. Roles and Membership

### 3.1 Rootstockers

Every accepted club member is a **Rootstocker**.

Rootstocker is the umbrella identity inside Asami. It replaces any separate in-club label. The point is to reinforce that the club belongs inside the Rootstock ecosystem rather than fragmenting it into a parallel social identity.

Most new Rootstockers should enter the club through invitation from:

* an existing community,
* an existing trusted member,
* or another approved Rootstocker with invitation capacity.

People without invitations may still apply to a waitlist, but acceptance is discretionary and not guaranteed.

Membership acceptance is separate from wallet sovereignty. A person may be accepted in the club while still using a managed account, or may already control their own wallet while being inactive at the club level.

### 3.2 Campaign Creators

Campaign creators are Rootstockers who fund campaigns, usually on behalf of a community.

They can:

* create repost campaigns around an X post,
* create giveaway campaigns for their community,
* define campaign budgets,
* choose recipient scope where the campaign type allows it,
* and build long-term community relationships through repeated appreciation.

“Campaign creator” is the preferred user-facing term for the role that funds campaigns.

### 3.3 Participants

Participants are Rootstockers who receive appreciation through a campaign.

In repost campaigns, they participate by reposting or quote-tweeting the target X post and remaining eligible until the reward assignment phase.

In giveaway campaigns, they participate by belonging to the selected community scope. No repost is required.

### 3.4 Campaign Administrators

Campaign administrators operate the off-chain logic around an unchanged smart contract.

They:

* determine campaign eligibility,
* gather campaign participants,
* calculate rewards,
* register collaborations on-chain,
* maintain reputation data,
* and operate the public frontend and backend services.

They do not need to change the smart contract in order to introduce new campaign behavior. Their power comes from software, discretion, transparency, and competition.

### 3.5 Token Holders

ASAMI token holders receive revenue sharing and vote on the protocol fee rate.

Token holders do not directly control campaign outcomes, club admission, or community scores. Governance is intentionally narrow.

---

## 4. Club Reputation

Asami no longer uses automated X analytics to compute member score. X is now used only where it is operationally necessary, mainly:

* login and account linking,
* and checking whether a user reposted or quote-tweeted a target post.

The score that matters in the club is a **reputation score** built from human evaluations recorded inside community relationships.

### 4.1 Score Categories

Each evaluation may include the following scored attributes, each from **1 to 10**:

* **Alignment**: fit with Rootstock and Asami values.
* **Trust**: honesty, reliability, and safety to work with.
* **Contribution**: useful value created for communities.
* **Advocacy**: effectiveness when helping spread a message.
* **Judgment**: quality of judgment about what deserves support.

Each evaluation may also include a claimed **Rootstock join date**.

The club derives a **Rootstock longevity score** from the earliest credible submitted date for a member. That date is mapped to a value from 1 to 10 on a moving timeline between the beginning of Rootstock and the present date.

### 4.2 Community Relationships

Asami uses the `community_members` relationship as the core record joining three ideas:

* community membership,
* current score from one Rootstocker to another,
* and community activity history such as collaborations and rewards.

There is only one mutable relationship per scorer-target pair. Re-scoring updates that relationship instead of creating an infinite score history.

This means:

* if you score someone, they become part of your community view,
* if you manually add someone to your community, the relationship can exist before scoring,
* and the same record can later accumulate collaboration counts, reward totals, and updated scores.

### 4.3 Priority Entries and Defaults

Some `community_members` rows may be marked as **priority**.

The most recent priority row for a target acts as the current club-wide override input for that target. It is not stored in a separate admin-only table.

Any score field may be null. A null field means “not provided,” not “low.”

If a campaign creator has a local score for someone but leaves one attribute null, that attribute falls back to the club-wide value for that member.

If a club-wide value cannot yet be built for an attribute, Asami defaults to:

* `5` for numeric score fields,
* and `2022-01-01` for the Rootstock join date.

### 4.4 When Someone Can Be Scored

A Rootstocker cannot score the same target more than **once every 30 days**.

If that cooldown is satisfied, scoring opens only under explicit club rules:

* **Invitation**: creating an invitation requires a score.
* **Weekly picks**: anyone may score the selected weekly members, or explicitly skip them.
* **Campaign relationship**: a campaign creator may score a participant only after cumulative rewards paid from that creator to that participant exceed **1 DOC**.

When a valid event opens scoring, it creates a **15-day window** for that scorer-target pair.

If someone does not know enough to score a target, they can create a skip record instead. Skips record the relationship and reason, but not numeric values.

### 4.5 Base Score and Creator Score

Each member has a **base score** and may also have a **creator-specific score** relative to a particular campaign creator.

The base score is built from all `community_members` rows where the user appears as `member_id`:

* 50% from the latest priority entry, if one exists,
* 50% from the average of regular community evaluations,
* or whichever side exists if only one side is present.

For a campaign creator evaluating a participant:

* 50% comes from the participant’s base score,
* 50% comes from that creator’s own current evaluation of the participant.

If the creator has not filled a field, the field falls back to the base value.

This creates a shared club-wide reputation layer while preserving each community’s right to apply local judgment.

### 4.6 Interpretation

Asami’s score is not a universal truth and not an automated estimate of social worth.

It is a structured club reputation signal designed for practical use inside Rootstock communities.

Its purpose is to help communities decide:

* who to invite,
* who to include in campaigns,
* how to split appreciation budgets,
* and how to recognize people who keep showing up.

---

## 5. Campaign Types

Asami currently works with two main campaign types.

### 5.1 X Repost Campaigns

An X repost campaign is created from:

* a budget in DOC,
* an X post URL,
* and a maximum reward per participant.

The duration is fixed at **20 days**.

Members do not see repost campaigns during their full contract lifetime. In the default frontend, they are only shown while they are within the intended public participation window, and they disappear from participant flows during the final settlement phase.

Operationally, repost campaigns work like this:

1. Members express intent by reposting or quote-tweeting the target post.
2. The backend checks repost eligibility using the participant’s own linked X account context.
3. Five days before the campaign due date, the campaign administrator gathers the final eligible participant list.
4. Rewards are prorated across that list according to score, subject to the maximum reward per participant set by the creator.
5. Collaborations are then registered on-chain asynchronously by the background worker.

After participants are gathered and rewards are assigned, the campaign is effectively over for new participants even if the contract due date has not yet arrived.

Any funds left unused remain in the contract and are returned to the creator according to normal protocol behavior after the due date.

Repost campaigns cannot be extended or topped up in the official frontend.

### 5.2 Community Giveaway Campaigns

A community giveaway campaign does not depend on reposts.

Instead, the campaign creator selects:

* all members of their community,
* or a specific subset of community members.

The backend gathers the recipient list immediately from those community relationships, then computes rewards using the same proportional reward logic used by repost campaigns.

The primary difference between repost and giveaway campaigns is how participants are gathered. After the final recipient list exists, reward allocation is the same:

* score-weighted,
* budget-constrained,
* capped when a cap applies,
* and then registered on-chain asynchronously.

### 5.3 Campaign Status Flow

Operationally, campaigns move through these off-chain stages:

* **published**: the campaign exists and may still gather participants,
* **collabs gathered**: the final participant list is closed,
* **rewards assigned**: each collaboration has a final reward amount,
* after that, the background worker may send the collaborations on-chain.

Only campaigns with assigned rewards are eligible for the on-chain collaboration job.

---

## 6. Tokenomics and Incentives

The **ASAMI token** is the native asset of the Asami.club protocol.

It has a fixed maximum supply of **21 million tokens** and is distributed through real protocol usage rather than presales or premine allocations.

### 6.1 Protocol Fee

Whenever DOC rewards are paid through the protocol, a portion is retained as a protocol fee. Those fees fund:

* ASAMI issuance,
* revenue sharing,
* and the club’s economic alignment.

### 6.2 Revenue Sharing

Every 15-day cycle, eligible ASAMI holders receive a share of the protocol fee pool in DOC.

To qualify, a holder must:

* hold at least the required ASAMI threshold,
* avoid moving those tokens during the cycle,
* and have claimed them into an eligible wallet.

The exact current thresholds and windows are visible in the live app and smart contract state.

### 6.3 Issuance Split

ASAMI issuance follows protocol activity. When collaborations are paid, ASAMI is distributed among the three operational roles:

* the **campaign administrator**,
* the **participant**,
* and the **campaign creator**.

The current protocol split remains the one encoded in the smart contract.

### 6.4 Governance

ASAMI governance is intentionally narrow.

Token holders currently vote only on the **protocol fee rate**. They do not vote directly on:

* who enters the club,
* how any specific campaign is administered,
* or how any particular member is scored.

---

## 7. Technical Architecture

### 7.1 Why Rootstock

Asami exists specifically for the Rootstock ecosystem.

Rootstock was chosen because it offers:

* Bitcoin-aligned security,
* EVM compatibility,
* stable operation,
* and a native ecosystem where DOC already makes sense as a community reward currency.

### 7.2 DOC as the Reward Currency

Campaign budgets and payouts are denominated in **DOC (Dollar on Chain)**.

DOC is used because it is:

* stable,
* native to Rootstock,
* and more practical than volatile assets for community appreciation.

### 7.3 The Smart Contract and the Off-Chain Layer

The current smart contract is intentionally narrow. It:

* receives campaign funds,
* records collaborations,
* pays rewards,
* and issues ASAMI under protocol rules.

The smart contract does **not** decide:

* who should be invited,
* who should be scored,
* how community reputation should be built,
* which reposts count,
* or how giveaway recipients are chosen.

Those decisions live in the off-chain campaign administrator software.

### 7.4 Open Source Software

The Asami software stack is open source and built primarily in **Rust** for the backend and a web frontend for member and admin operations.

It handles:

* account management,
* campaign administration,
* community relationships,
* scoring,
* X account linking,
* repost verification,
* and gasless onboarding flows.

### 7.5 Gasless Claims and Onboarding

Asami supports gasless claim flows to help people who are new to Rootstock receive rewards and start using a self-sovereign wallet gradually.

The club’s account model separates:

* wallet sovereignty,
* and club acceptance.

That makes it possible to onboard people before they are fully comfortable with Rootstock wallets, while still giving them a path toward full control.

---

## 8. Governance, Trust, and Abuse Prevention

Asami is not governed by a company or a broad DAO that resolves every dispute.

It is governed by:

* the smart contract,
* the open source software,
* the club’s curation rules,
* and the practical discretion of campaign administrators and community operators.

### 8.1 No Service Guarantees

Participation in Asami is voluntary and offered as-is.

There are no service-level guarantees that:

* any campaign will include any specific participant,
* any waitlist applicant will be accepted,
* any campaign creator will keep funding campaigns,
* or any campaign administrator will operate under one single philosophy forever.

### 8.2 Abuse Prevention

The current abuse surface is different from the early versions of Asami.

The main risks now are:

* extractive membership farming,
* low-effort reposting without community alignment,
* manipulation of relationship-based scores,
* invitation abuse,
* and spammy community creation designed only to open scoring rights.

The club mitigates these risks through:

* invitation curation,
* waitlist discretion,
* scoring windows,
* cooldowns,
* the `> 1 DOC` threshold before campaign-based scoring opens,
* public reputation relationships,
* and administrator oversight.

### 8.3 Transparency

Financial actions are public on-chain.

Community relationships, scores, and campaign participation are more social than contractual, but they are still meant to be visible enough for scrutiny, review, and comparison.

### 8.4 Competition

Campaign administrators have no special status in the protocol.

Anyone capable of operating compatible software may build their own frontend, curate their own communities, and administer campaigns differently.

That possibility is an important part of Asami’s governance model.

---

## 9. Market Fit

Asami is not trying to replace quests, ad networks, or broad social promotion marketplaces.

It serves a narrower purpose:

* helping Rootstock communities appreciate known members,
* helping them discover aligned new members gradually,
* and giving public, modest, Rootstock-native rewards to meaningful participation.

This makes Asami complementary to:

* onboarding tools,
* quest systems,
* community programs,
* and traditional marketing channels.

Its distinctive feature is not mass reach. It is **community-shaped trust**.

---

## 10. Future Direction

The next stages of Asami are centered on strengthening the club model, not broadening into a generic social platform.

Likely areas of development include:

* better admin and reviewer tools for waitlist and invitation management,
* richer community campaign types,
* better member scoring workflows,
* stronger community views and history,
* BTC and Lightning-based funding flows that still settle into DOC logic,
* and more refined tooling for Rootstock community operations.

Asami is not currently focused on becoming a multi-platform attention marketplace. Its strategic home is Rootstock.

---

## 11. Legal and Data Considerations

Asami is a protocol and software stack, not a promise of service, investment return, or institutional endorsement.

### 11.1 No Investment Offering

ASAMI is not presented as a security or guaranteed investment return.

It is a protocol-native asset tied to participation, fee sharing, and governance.

### 11.2 No Guarantee of Admission or Reward

Club admission is discretionary.

Campaign participation is discretionary.

Rewards depend on campaign rules, administrator behavior, on-chain settlement, and the limits of the protocol.

### 11.3 User Responsibilities

Members are responsible for:

* complying with local laws,
* handling any necessary disclosure of paid promotion,
* and understanding the tax implications of receiving rewards.

### 11.4 Data Use

Asami uses:

* on-chain data,
* account and community relationship data,
* and limited X data needed for linking accounts and checking repost participation.

It does not depend on premium X-wide analytics for scoring.

If you have concerns about data or moderation, contact the community publicly through:

* [@asami_club](https://twitter.com/asami_club)
* [@asami_club_es](https://twitter.com/asami_club_es)
* [@asami_club](https://t.me/asami_club)
* [@asami_club_es](https://t.me/asami_club_es)

---

## 12. Final Notes

Asami.club is a Rootstock-native experiment in community appreciation.

The protocol is public. The smart contracts are permissionless. The club itself is curated.

If you are a Rootstocker, participate in a campaign when it fits.

If you are a community, create one.

If you are a builder, build on the protocol.

The long-term shape of Asami will come from the quality of the communities that choose to use it.

---

## Appendix A: Contract Addresses and Links { .no-cols }

**Asami Smart Contract** (Rootstock):  
https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954

**Current Campaign Administrator Address**:  
https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa

**DOC Stablecoin Info (Money on Chain)**:  
https://moneyonchain.com/

**Open Source Asami Software**:  
https://github.com/constata-eu/asami

---

## Appendix B: Contact and Support { .no-cols }

Members are encouraged to ask questions through public channels.

**X:**

* English: [@asami_club](https://twitter.com/asami_club)
* Spanish: [@asami_club_es](https://twitter.com/asami_club_es)

**Telegram:**

* English group: [@asami_club](https://t.me/asami_club)
* Spanish group: [@asami_club_es](https://t.me/asami_club_es)

---

## Appendix C: FAQ { .no-cols }

**Q: Why am I not seeing campaigns?**

* You may not be an active club member yet, your community relationships may not make you eligible, or there may be no campaigns open to you right now.

**Q: Why does my score seem too low?**

* Scores are now built from community evaluations and priority entries, not from automated X metrics. Review the reputation section and ask for a review if a relationship or score was updated incorrectly.

**Q: Why did I receive less than another participant?**

* Rewards depend on campaign budget, participant list, score, and any maximum-per-user cap set by the creator.

**Q: I was marked inactive or left on the waitlist. Can I still use Asami?**

* Not necessarily in the default club context. Club acceptance is curated.
