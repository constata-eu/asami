# Asami Club Whitepaper

This whitepaper describes the system and ecosystem as envisioned and was last revised on july 2025.

---

## 1. Overview

In Web3, where openness, decentralization, and community are core values, spreading ideas through top-down media campaigns is expensive, often viewed with suspicion, and ultimately ineffective for long-term user adoption. While traditional advertising can create short-term visibility, it rarely builds trust or fosters belonging. Most meaningful growth in this space has come from early users and explainers — people who share what they’ve discovered out of curiosity, belief, or a desire to help others.

These individuals, whom we call **Advocates**, act as curators, interpreters, and amplifiers. They repost, translate, and contextualize emerging projects. They bring visibility, legitimacy, and early traction. But because their contributions are informal and often unpaid, they are easily overlooked. Many drift away. Some are quietly absorbed by ecosystems that offer more recognition. Others simply give up.

Not because they were wrong — but because the ecosystem gave them no reason to continue.

---

### A Structural Gap

Web3 projects do care about visibility, but the tools available to them are limited. Most marketing budgets end up:

* On platforms that are **misaligned with Web3 values**, or that restrict blockchain-related content outright.
* Or in **incentivized usage schemes**, paying individuals to complete tasks, register accounts, or simulate engagement — often attracting users who vanish as soon as the rewards stop.

What these approaches lack is a way to recognize and support **genuine advocacy** — not usage, not artificial engagement, but public expression of interest and belief.

Open source and decentralized projects succeed when they find and retain **voluntary advocates**. But most have no systematic way to identify them, thank them, or encourage them to continue.

---

### The Asami Protocol

**Asami.club is a decentralized protocol for recognizing and rewarding Web3 advocates.** It helps Projects:

* Reach real people who publicly support them,
* Thank them transparently,
* And build a durable network of aligned voices.

Before participating in any campaign, each user is scored and tagged by Asami through a **periodic influence measurement process**. This baseline scoring evaluates tweet impressions, engagement patterns, verified follower ratios, language, and Web3 relevance. Only those who meet the quality threshold are admitted into the pool of eligible Advocates.

Once curated into this pool, Advocates can **discover campaigns and opt in voluntarily** by reposting content they believe in. Projects create campaigns by selecting a post (typically a tweet) and allocating a budget using DOC, a Bitcoin-backed stablecoin.

When an eligible Advocate reposts, they may receive:

* A modest DOC reward, as a public thank-you.
* ASAMI tokens, which offer protocol revenue sharing and governance participation.

This payment is **not an incentive to repost** — it is a way to **recognize and acknowledge** visibility already given. It also serves as a **signal of responsibility**: by accepting a public reward for a public action, the Advocate demonstrates transparency and moral ownership over the message they amplified.

From that point, **Projects can begin curating their own advocate community**. They may observe who participated, maintain allowlists and blacklists, and create campaigns that are fully open, semi-restricted, or exclusive to a trusted group. This lets them balance **exploration** (finding new potential Advocates) with **exploitation** (working with known, effective voices). Some projects may even follow up outside Asami for deeper engagement, such as content creation or ambassadorships.

### Benefits to the Ecosystem

Asami is one possible solution to the challenge of growing Web3 through trust, not noise. Unlike platforms that pay people to use a product or perform arbitrary tasks, Asami is focused on **recognizing genuine, voluntary advocacy**.

It provides a structure for:

* Encouraging Advocates to keep sharing the things they already care about.
* Helping Projects discover and appreciate these early supporters.
* Keeping the interaction public, modest, and mutually beneficial — without compromising independence or integrity.

Because all interactions are on-chain and participation is permissionless, there are no contracts, no private deals, and no hidden influence. And because each campaign helps Projects identify aligned voices, the outcome is not just visibility — it is **community formation**.

This creates a **circular economy of reputation and support**: Projects fund campaigns, Advocates receive recognition, and both sides help the Web3 ecosystem grow in alignment with its values.

---

## 2. Project History, Discoveries, and Current State

In 2023, **Nostr** was reimagining social media as a decentralized platform — one built around simple cryptography and permissionless participation. It was being promoted by passionate advocates, much like other movements before it, including Bitcoin. What set Nostr apart was that it wasn’t just a neutral protocol — it actively supported its early users through **social signaling and micro-rewards** like likes and zaps.

This sparked the original idea behind Asami: what if we combined this culture of **social advocacy** with **smart contracts**, allowing people to be rewarded — not for usage, but for amplification? If someone chose to repost a message on Nostr, a smart contract could verify that action and automatically send them a reward. No centralized ad broker, no negotiation — just a trustless thank-you.

### 2.1 From Protocol Experiments to Practical Advocacy

The first Asami prototype implemented exactly this idea: a smart contract that received funds from a Project and let anyone claim a reward by reposting a message on Nostr. While technically elegant and fully decentralized, two critical limitations emerged:

* Verifying reposts cryptographically on-chain was **too expensive to scale**, especially for micropayments.
* Nostr’s **user base was still too small** to attract significant campaign interest from Projects.

This led to a pivot: Asami integrated with **X (formerly Twitter)**, embracing a hybrid model. Reposts are now verified off-chain by a third party — the Campaign Manager — and recorded on-chain. This reduced cost while enabling wider reach, without compromising the protocol’s values of transparency and traceability.

### 2.2 Market Assumptions and What We Learned

Asami was built on two core assumptions:

* That **Web3 users would be willing to repost content they believe in**, if they felt respected and acknowledged.
* That **Web3 Projects would prefer authentic visibility from real Advocates**, rather than fake engagement or inflated metrics.

Both proved true — but execution required nuance.

Advocates joined enthusiastically, often reposting without expectation of high payouts. Projects appreciated seeing organic support. But it quickly became clear that **a fair and stable reward system** required:

* Accurate influence measurement,
* Healthy pricing expectations,
* And mechanisms for Projects to **build their own communities**, not just rent attention.

It also became obvious that the goal was never to “pay people to post.” Instead, the reward had to be framed — and designed — as a **thank-you**, delivered publicly, and modestly, only after the act of advocacy.

### 2.3 Early Challenges and Solutions

Several early frictions informed the protocol's evolution:

* **Unreliable scoring**: Initial models based on follower count were easy to exploit. The current influence scoring algorithm uses recent tweet impressions, engagement ratios, mentions, verified follower-to-following ratios, and optional manual input. Scores are updated periodically, independent of campaign participation. This creates a baseline filter that ensures only authentic, relevant accounts can participate in campaigns.

* **Limited advocate growth**: A small number of early Advocates dominated early rewards. To encourage new voices, Asami added **referral incentives** and tracks long-term participation.

* **Frictions in campaign creation**: While Advocates could receive crypto easily, many Projects — especially those with Web2-native marketing teams — preferred using credit cards. To support them, Asami integrated **Stripe payments**, allowing campaigns to be funded without crypto. (A 20% fee applies to offset costs and slippage.)

* **From reach to relationship**: Projects didn’t just want impressions — they wanted connection. Campaigns began to function as discovery tools: ways to find potential Advocates, observe them in action, and then invite them into curated allowlists for future campaigns. Projects now balance **exploration** (testing interest) with **exploitation** (working with known supporters), and can contact trusted Advocates outside the platform if desired.

### 2.4 Web3-Only Focus and Long-Term Vision

Originally, Asami left open the possibility of broader use. But over time, it became clear that **Web3 Projects are best aligned** with the protocol’s values: transparency, decentralization, and community-based growth.

Asami is now focused exclusively on the Web3 ecosystem. Its mission is to help Projects:

* Reach aligned Advocates,
* Recognize them fairly,
* And grow their communities sustainably.

As of mid-2025, the system is live and fully operational. Campaigns run daily. Scoring is active. Advocates sign up with X or email, and link wallets when they’re ready. Stripe onboarding works. Gasless withdrawals are available. The protocol is open-source, evolving, and welcoming new Campaign Managers and integrations.

To reinforce the tone of appreciation rather than compensation, Asami adopted the **peanut** as its mascot — a playful reminder that rewards may be modest, but they’re sincere. Visibility, not wealth, is what Advocates contribute. The DOC and ASAMI tokens they receive are simply **a public thank-you**.

Looking ahead, Asami’s focus remains on:

* Advancing the technical and philosophical framework for measuring genuine influence,
* Expanding to new platforms and interaction types (e.g., comments, likes, LinkedIn, Nostr through oracles),
* And enabling **per-action scoring**, where Advocates can be appreciated for the reach of a specific post or quote tweet, not just their overall account.
* Supporting Bitcoin and Lightning Network (LN) payments for campaign funding, with automatic DOC conversion and lower fees than credit cards — expanding accessibility while preserving campaign integrity.

---

Perfect — based on everything you’ve said, including the Spanish text and the nuanced tone you want (welcoming but not naive), here’s a fully rewritten **Section 3: Ecosystem Roles and Stakeholders** that reflects the new language, structure, and mission of Asami.

---

## 3. Ecosystem Roles and Stakeholders

The Asami protocol brings together three main roles:

* **Projects**, who want to increase their visibility and build a network of trusted supporters.
* **Advocates**, who amplify messages they believe in and receive public acknowledgment for doing so.
* **Campaign Managers**, who operate the infrastructure and enforce protocol-level rules like repost registration and influence scoring.

Each participant plays a distinct part in the system, with clearly defined rights, responsibilities, and expectations.

---

### 3.1 Projects

**Who they are:** Anyone running a Web3 initiative — protocols, dApps, DAOs, tooling projects, artists, infrastructure providers, or community-driven movements — who wants to engage real people to amplify their message.

**What they do:** Projects create campaigns by selecting a post (usually on X), defining a budget in DOC, and choosing a Campaign Manager. They can then set parameters for how rewards are distributed and who may participate.

**Rights:**

* Set campaign budget, duration, and payout structure.
* Restrict or allow participation using allowlists and blocklists.
* Review participants and curate an evolving list of trusted Advocates.
* Fund campaigns using DOC, credit card (via Stripe), or soon, Bitcoin and LN.
* Choose and switch Campaign Managers freely.
* Withdraw unused funds once a campaign ends.

**Obligations:**

* Accept that campaigns, once live, are non-reversible.
* Trust the Campaign Manager to apply repost registration and influence scoring fairly.

**Notes:**

* Projects may create open campaigns to discover new Advocates (exploration), or exclusive ones for pre-vetted communities (exploitation).
* Reposts are voluntary — Projects cannot force anyone to amplify their content.
* All payments are public and traceable on-chain.

---

### 3.2 Advocates

**Who they are:** Social media users — primarily on X — who have a genuine audience and choose to responsibly amplify Web3 projects.

Advocates don’t have to be celebrities, founders, or industry insiders. Every genuine voice matters. Some Advocates may just be getting started, sharing new projects with friends, coworkers, or a small but real audience. That’s welcomed. Advocacy is not about being loud — it’s about being aligned.

**What they do:** Advocates browse eligible campaigns (after passing Asami’s baseline scoring) and choose what to repost. If their repost meets campaign requirements, they receive a modest payment in DOC and ASAMI tokens.

**What defines a real Advocate:**
Asami defines an Advocate as a person who:

* Possesses and grows a **genuine audience** on X.
* **Shares Web3 content responsibly**, expressing real interest or insight.
* Is **not reposting blindly** or purely for reward collection.
* Brings **real visibility** to the projects they amplify.

This means reposting alone is not enough — Advocates are expected to show engagement, understanding, and relevance.

**Rights:**

* Choose which campaigns to repost — participation is always voluntary.
* Be rewarded (in DOC and ASAMI) if their repost is valid and impactful.
* Request a review of their influence score or assigned tags (e.g., language, region).
* Use gasless withdrawals to claim earnings.

**Obligations:**

* Maintain reposts for a minimum time period.
* Operate a legitimate X account with organic activity.
* Avoid reposting every campaign without context or judgment.
* Accept that their score may change over time, and not all changes will result in payment.

**About Scoring and Growth:**

Asami's influence scoring is imperfect by design. We are still discovering how to best measure real influence. If you’re starting your journey as an Advocate, and your score is low — don’t be discouraged. Keep learning, discussing, and expressing your thoughts online. Quote tweets, honest takes, critical reviews, and thoughtful engagement all count. Avoid relying on AI-generated content or gimmicks — they tend to lower real reach and reduce long-term value.

Some users may try to increase their score as a matter of pride or recognition — and that’s healthy. But if the score becomes an obsession, or is treated as an income source, disappointment is likely. Rewards in Asami are not guaranteed, nor intended to be a financial livelihood.

We know that our scoring algorithm may not fully recognize your value. If that happens, it’s not necessarily because you lack influence — it may be that we don’t yet have the tools to measure the kind of influence you bring. Over time, as both your audience and our tools improve, we hope your score will reflect your effort.

**Disputes:**
Advocates may request a review of their score, but they must:

* Reference specific parts of the scoring criteria.
* Compare their activity to others with similar profiles.
* Provide supporting links or metrics (e.g., engagement stats, referrals).
* Link to their public Asami profile.

General complaints like “my score is too low” without evidence may be ignored.

---

### 3.3 Campaign Managers

**Who they are:** Independent operators who run the Asami protocol software, process reposts, score influence, manage user tagging, and enforce campaign rules.

**What they do:**

* Detect and register reposts.
* Calculate influence using their own logic.
* Handle gasless withdrawals and user support.
* Manage allowlists/blacklists on behalf of Projects.
* Optionally offer services such as content strategy or analytics.

**Rights:**

* Define and update their own influence scoring logic.
* Decide which Advocates are eligible for campaigns.
* Maintain public leaderboards, tags, and engagement labels.
* Set their own fees (on top of protocol fees, if desired).

**Obligations:**

* Use clear and reproducible logic.
* Be responsive to well-formed user inquiries.
* Operate transparently — all campaign logic should be open to audit.
* Follow campaign rules as defined by each Project.

**Limits:**

* Cannot reverse payments once made.
* Cannot override smart contract logic.
* Can be replaced at any time — Projects and Advocates may choose other managers.

Asami does not give Campaign Managers any privileged access. Their authority comes from the quality of their service and their ability to earn trust. If they fail to act transparently, or if their scoring is unreasonable, others can launch competing Campaign Managers at any time.

### 3.4 Token Holders

**Who they are:** Anyone who holds ASAMI tokens — including Advocates, Projects, and Campaign Managers who received tokens through campaign participation or direct purchase.

**What they do:** ASAMI token holders benefit from long-term alignment with the protocol. They receive a share of collected fees and participate in protocol-level governance.

**Rights:**

* Receive a proportional share of DOC fees retained by the protocol.
* Vote on the protocol’s fee rate via a weighted average mechanism.
* Hold, transfer, or sell ASAMI tokens freely.

**Obligations:**

* None enforced by the protocol. However, responsible governance is expected from those who care about Asami’s future.

**Notes:**

* Holding ASAMI tokens does not grant power over campaign decisions or scoring.
* Token holders **do not control** Campaign Managers or influence individual campaign outcomes.
* The only shared governance mechanism currently live is the ability to vote on the protocol-wide **fee rate**, which affects revenue distribution and token issuance.

> 💡 In practice, this means that token holders help balance two goals:
>
> * Higher fees increase the pool of DOC distributed to holders, but also inflate token supply and may reduce campaign participation.
> * Lower fees grow the ecosystem and reduce inflation, but reduce short-term returns.

Governance is thus minimal, incentive-driven, and transparent. Token holding is a form of **participation in the club’s future**, not a lever of control over others.

---

## 4. Influence Measurement

Influence within Asami is calculated using a structured model that defines influence as the product of **audience size** and **authority**. This model ensures fairness and scalability while offering Campaign Managers flexibility to tailor scoring methods.

### 4.1 Audience Size

Audience Size is a quantitative measure of how many people actually see a user's posts. Initially, follower counts were used, but the current system relies on the number of tweet impressions over the past 45 days. This gives a much more accurate and real-time view of actual reach.

This measurement is pessimistic: no audience is assumed unless it can be proven via impressions. Impression counts can still be manipulated, so the system checks for statistical correlation between impressions and engagements (likes, comments, reposts). Accounts with abnormal ratios or signs of manipulation may have their Audience Size set to zero.

Thanks for the clarification — you're absolutely right. The original **Authority** section (4.2) had a layered, step-by-step breakdown of each criterion contributing to the score, and that level of detail is necessary to maintain transparency and rigor. Splitting it up is a great solution.

Here’s the updated **Section 4: Influence Measurement**, with just the **intro** and the full **Subsection 4.1: Audience Size**, rewritten to include everything as discussed:

---

## 4. Influence Measurement

In Asami, influence determines whether an Advocate can participate in campaigns and how much they are rewarded. But unlike follower counts or blue checkmarks, Asami treats influence as a working model — **a dynamic, data-driven estimate** that aims to reflect both reach and trustworthiness.

The system doesn’t claim to measure “true influence” perfectly. Instead, it uses available public data to make the best possible approximation at a given moment in time — and improves over time. Influence scores are recalculated **periodically** (currently every 20 days), regardless of campaign activity. This ensures fairness and avoids gaming or rapid manipulation.

Each Advocate’s influence is defined by the combination of:

* **Audience Size** — how many people likely see their posts
* **Authority** — how meaningful or credible their voice is

These two scores are **multiplied** to produce a final Influence Score, which controls eligibility and reward size for campaigns. The date of each user's **last and next scoring** is displayed on their public Asami profile.

---

### 4.1 Audience Size

Audience Size measures how many people are likely to see a user’s content on X — not based on followers, but on actual tweet impressions.

To calculate Audience Size, Asami:

* Collects **all public posts and replies** from the last 30 days
* Computes the **average impressions** across those posts
* Computes the **median impressions** from the same set
* Then **averages those two values** to produce a final Audience Size.

This hybrid method reduces the effect of outliers (like a one-off viral tweet) while still rewarding consistent activity. By including **replies**, the system captures both profile-based and conversational posting styles — but with some caveats.

> 📝 **Note on Replies:** Replies tend to receive fewer impressions than original posts. So Advocates who rely heavily on replies may show a lower Audience Size, even if they’re active. This behavior may be adjusted in future algorithm versions. However, if a user’s replies drive strong engagement, their **authority score** will likely reflect that — so thoughtful engagement is still rewarded.

Audience Size is always calculated pessimistically — if real visibility cannot be demonstrated through public metrics, no audience is assumed.

### 4.2 Authority

Authority measures **how much weight your voice carries with your audience**. It's not enough to be seen — the system also looks for signs that people pay attention, engage, and possibly trust what you share.

Authority is calculated through a layered evaluation of several public signals. If no meaningful engagement can be detected, Authority is set to 0%, and the user becomes ineligible for campaign rewards — regardless of Audience Size.

Each criterion below adds or modifies the score. The result is a percentage from 0% to 100%, multiplied with Audience Size to produce the final Influence Score.

#### Engagement Received on X

This is the **core criterion**. If a user's posts consistently receive no meaningful engagement from others, they are not considered influential.

Asami evaluates:

* The ratio of **likes, replies, comments, quote tweets, and retweets** to impressions
* Whether the user is **mentioned by others**, especially in **high-profile posts**
* The user’s **verification status** (e.g. X Premium, legacy checkmark)
* The number of **verified followers**, relative to total followers

A high number of impressions with little to no interaction will result in **low authority**, and thus, a low overall Influence Score.

Scoring tiers:

* **None**: Posts are mostly ignored, or engagement is artificial → 0%
* **Average**: Consistent, organic interaction → +25%
* **High**: Strong engagement, public interest → +50%

#### Direct Audience Polling

Advocates may run anonymous polls to ask their followers how much they trust their recommendations. These polls give insight into perceived authority.

Example prompt:

> *“If I recommend a project, do you usually: (a) Follow it blindly, (b) Consider it, (c) Ignore it, (d) Do the opposite?”*

Scoring tiers:

* **None**: No recent polling → +0%
* **Average**: Mixed responses, some trust → +10%
* **High**: Clear positive consensus → +20%
* **Reverse**: Most say they’d do the opposite → cuts Engagement score in half

#### Engagement Outside X

Some people have meaningful influence in other environments (e.g. podcasts, conferences, meetups, DAOs). This criterion accounts for **off-platform reputation**.

Scoring tiers:

* **None**: No notable presence outside X → +0%
* **Average**: Active in adjacent communities → +5%
* **High**: Recognized voice in Web3 circles → +10%

Evidence can be submitted manually during a review.

#### Status on X

The functional status of an account matters.

Scoring rules:

* **Banned, restricted, or shadowbanned**: Authority = 0%
* **Normal operational account**: No change
* **Verified / Premium**: +10%

#### Referral Authority

Advocates who refer others to Asami and help them succeed demonstrate influence.

Scoring rule:

* **Successful referrals from active accounts** → +10%

#### Token Holding Behavior

Advocates who **hold ASAMI tokens** instead of immediately selling them signal alignment with the project and its long-term success.

Scoring rule:

* **Holding tokens during the scoring window** → +10%

#### Final Calculation

To calculate Authority:

1. Start with the base from **Engagement Received on X**

   * If 0%, all other factors are skipped.
2. Add (or modify) the rest:

   * Audience Polling
   * Off-X Presence
   * X Account Status
   * Referrals
   * Token Holding

Maximum possible Authority: **100%**
Minimum: **0%** (with no engagement or ineligible signals)

> 🎯 **Note:** The system is designed to reward behavior that leads to real, earned trust — not vanity metrics. Buying likes, reposting blindly, or creating superficial interactions will not improve your score. Authority is earned slowly and lost quickly.

### 4.3 Final Influence Score

Once an Advocate’s **Audience Size** and **Authority** are calculated, they are **multiplied** to produce their final **Influence Score**.

This score is used to:

* Determine eligibility for participating in campaigns
* Allocate DOC and ASAMI token rewards
* Rank Advocates on public leaderboards

The Influence Score is refreshed **periodically** (currently every 20 days), independently of campaigns. This ensures that all participants are scored fairly based on recent behavior and that new Advocates have a chance to qualify without needing to wait for specific campaigns.

This model rewards both reach and trust. A large audience without engagement produces a weak score. A credible voice with no audience also scores low. The most effective Advocates are those who are **seen and respected** — and ideally, **consistent over time**.

### 4.4 A Note on Transparency and Limits

Asami does not pretend to measure influence perfectly. Influence is contextual, evolving, and hard to quantify — especially when data is incomplete or activity is subtle.

Some Advocates may feel under-scored, particularly if their value lies in quiet networks, private interactions, or off-platform influence that Asami cannot yet detect.

If you are one of them:

* You are welcome to request a scoring review using the standard public process.
* But don’t take your current score as a judgment of your worth, potential, or integrity.

> **Your journey and ours may meet at a destination.**
> If you continue building an organic audience, engaging thoughtfully, and advocating sincerely — and as we keep refining our algorithm to better detect genuine influence — there is a good chance we will eventually align.

We are not trying to capture all types of influence — only the kinds we can measure with confidence, in a way that’s fair and reproducible. As our methods improve, we hope to reward more Advocates in more contexts.

Until then, remember: Asami’s score is just a score. Your voice is yours.

---

## 5. Tokenomics and ASAMI Distribution

The **ASAMI token** is the native asset of the Asami.club protocol. It was designed to provide **equity and upside** to the people who make the club grow: **Advocates**, **Campaign Managers**, and **Projects**.

Unlike platforms that rely on up-front funding or token sales, Asami only issues tokens when there is **real usage** of the protocol. The ASAMI token aligns incentives around sustained participation and responsible advocacy — not speculation.

All token-related data and stats are available publicly at [asami.club/#/Token](https://asami.club/#/Token).

### 5.1 Revenue Sharing and Incentives

The Asami protocol redistributes its revenue back to the people who grow it — not just through token issuance, but through **revenue sharing** tied to token holding.

Every 15-day cycle, the **protocol fee pool** — collected from all paid collaborations — is distributed among eligible ASAMI holders. This creates an incentive to **hold tokens** and **participate long-term**, rather than speculate or flip.

#### Eligibility Rules

To receive your share of the revenue pool, you must:

* ✅ **Hold at least 4,000 ASAMI tokens**
* ✅ **Not move your tokens during the current 15-day cycle**
* ✅ **Have claimed your tokens into a personal wallet (not a contract)**

Token cycles are tied to the **smart contract periods**, not simply rolling windows. The exact date range for each cycle is visible on the [Token Stats page](https://asami.club/#/Token).

> ⛔ **Important:**
>
> * If you **move your tokens** (transfer, sell, stake, or otherwise send them) during a given period, you become ineligible for that period's rewards.
> * If you haven’t **claimed your tokens**, they won’t count. Only claimed balances are eligible.
> * ASAMI tokens **held by contracts** — including liquidity pools — do **not** receive revenue sharing unless explicitly enabled by their code.

#### What Happens to Unclaimed Shares?

When a user is eligible but **does not claim** their revenue share during the claim window, their unused portion is **returned to the fee pool** and rolled over into the **next cycle’s rewards**.

This creates:

* **Simplicity** — no late claims or dangling obligations
* **Fairness** — rewards go to active participants
* **Composability** — external contracts may implement strategies to distribute to their users, but this is opt-in and out of protocol scope

This system ensures that early and committed participants receive **passive income** in proportion to their contributions, while keeping ASAMI's supply aligned with real protocol usage.

### 5.2 Governance via Fee Voting

ASAMI token holders participate in **a single governance mechanism**: setting the protocol fee rate that determines how much of each campaign’s budget is allocated to the token issuance pool.

This rate directly influences:

* How many ASAMI tokens will be issued in future periods
* The relative value of being an Advocate or Campaign Manager
* The sustainability of the protocol’s reward structure

Rather than managing campaigns, whitelists, or any operational functions, token holders shape the **long-term behavior of the platform** through this lever.

> 🗳 **One Token, One Vote**
> Voting is weighted by each address’s ASAMI balance at the time of snapshot. The final rate is the weighted average of all votes.

#### Example:

If the current protocol fee is 20%, and the community feels that token issuance is too aggressive, large holders can vote for a lower fee — say, 15%. If most votes support this change, the next cycle will adjust the fee accordingly.

This design ensures that:

* The platform remains flexible, but not chaotic
* Token holders express preferences without micromanaging
* Changes happen incrementally, with built-in damping

Governance is implemented **on-chain** in the protocol’s smart contract and can be audited [here](https://github.com/constata-eu/asami/blob/38651663a124c714d2e599661f9abf3976e5f628/contract/contracts/AsamiCore.sol#L711).

### 5.3 Issuance Model and Fairness

Asami’s token issuance is designed to be **proportional, transparent, and fair**. Tokens are only issued when someone **funds a campaign** and another person **collaborates with it** — anchoring the value of ASAMI in real advocacy.

There are no allocations for founders, investors, or insiders. Every token issued comes from actual usage of the platform.

#### How It Works

Every 15 days, the protocol evaluates:

* How many DOC were collected in fees from registered collaborations
* Whether the protocol’s issuance rate needs to increase or decrease
* A maximum issuance cap of **100,000 ASAMI** per period (target, not guarantee)

If campaigns slow down, fewer tokens are minted. If engagement increases, issuance may rise — but never above the cap. This makes ASAMI a **work-based currency**, not a speculative one.

#### Token Distribution Per Collaboration

Each collaboration triggers a **fee-based issuance** of ASAMI tokens. These are distributed as follows:

* 🥜 **40%** to the **Campaign Manager** who registered the campaign
* 🥜 **30%** to the **Advocate** who reposted the campaign
* 🥜 **30%** to the **Project** (Advertiser) that funded it

This model reinforces a healthy dynamic:

* Campaign Managers are incentivized to bring great campaigns and manage quality
* Advocates are rewarded for meaningful reach
* Projects earn long-term equity by showing up early and funding the ecosystem

> ❗ **Note:** Projects only receive ASAMI when funding campaigns **on-chain via Rootstock** using DOC. Campaigns funded through **Stripe** or future **Bitcoin/Lightning** gateways do not result in ASAMI being issued to advertisers — though Advocates and Campaign Managers still earn their share.

#### Example: Earning 1,000 ASAMI

If a campaign resulted in 1,000 ASAMI being issued:

* 400 go to the Campaign Manager
* 300 go to the Advocate
* 300 go to the Project that funded the campaign (if DOC was used on Rootstock)

No matter your role, you earn in proportion to your contribution. There’s no need to stake, lock, or speculate — only to participate.

The only large token holder today is the current Campaign Manager, who has registered all campaigns to date and received \~40% of the ASAMI issued, as per protocol rules.

Here is **Section 5.4: Claiming and Holding Tokens**, incorporating all the key mechanics of how ASAMI tokens are claimed, what it means to hold them, and how holding connects to governance and revenue sharing:

### 5.4 Claiming and Holding Tokens

ASAMI tokens are not sent automatically. To become a token holder, each participant must take the **deliberate step** to claim their tokens.

This design reflects Asami’s philosophy: **ownership should be intentional**, and participation should remain voluntary at all times.

#### Claiming Your Tokens

If you've earned ASAMI by participating in a campaign — whether as an **Advocate**, a **Campaign Manager**, or a **Project** — you can claim them at any time.

To do so:

* Connect a **Rootstock-compatible wallet**
* Visit your public profile or the Token page
* Claim your accumulated tokens to your wallet address

You can also choose to delay claiming — but **unclaimed tokens do not receive revenue sharing**.

#### Holding Tokens

Once claimed, your tokens must be **held untouched** for an entire 15-day cycle to qualify for revenue sharing.

This means:

* No transfers, swaps, or moves during the cycle
* Tokens must reside in a personal wallet (not a liquidity pool or other contract)
* Holding over **4,000 ASAMI** is required to qualify for distribution

If you move your tokens — even momentarily — your share for that cycle is forfeited.

Asami’s claim-and-hold model ties incentives directly to commitment. It ensures that only active, deliberate participants shape the club’s future — and that early supporters are **rewarded modestly and fairly** over time.

## 6. Technical Architecture

The Asami protocol is implemented as an open-source smart contract deployed on the **Rootstock** blockchain. Rootstock is a Bitcoin sidechain offering full EVM compatibility, which allows developers to use familiar Ethereum tooling while benefiting from Bitcoin’s security model.

### 6.1 Why Rootstock
Rootstock was chosen because it offers:
- **Merged mining with Bitcoin**, which enhances its security.
- **EVM compatibility**, enabling fast smart contract deployment.
- **Uptime and reliability**, with no known downtime and consistent block production.
- **Bitcoin-native environment**, which aligns with Asami’s decentralization goals.

Rootstock uses **RBTC** as its native gas currency and is a battle-tested network with low fees and stable operation, making it a solid foundation for Asami.

### 6.2 Why Dollar on Chain (DOC)
Campaign budgets are denominated in **DOC (Dollar on Chain)**, a stablecoin also native to Rootstock and issued by Money on Chain. It is:
- Over-collateralized with Bitcoin
- Widely used in the Rootstock ecosystem
- Pegged 1:1 to the US Dollar

Collaborators are paid in DOC. This gives them a reliable and predictable payout mechanism and ensures that reward values are not affected by crypto market volatility.

> If a Collaborator reposts a campaign message and is eligible, they receive a DOC payment from the campaign budget. The protocol automatically retains 10% of this payment as a fee.

### 6.3 Smart Contract Address and Transparency
The smart contract powering the protocol is publicly verifiable and can be viewed at:
[Asami Contract on Rootstock Explorer](https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954)

The first Campaign Manager currently operates from:
[Campaign Manager Wallet](https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa)

All interactions with the protocol—campaign creation, collaboration registration, and reward distribution—are visible and traceable on-chain.

### 6.4 Campaign Manager Software
Campaign Managers interact with the protocol using software built in **Rust**, powered by the **Rocket** web framework. This application handles:
- Collaboration detection
- Influence scoring
- Repost registration
- Web2/Web3 wallet bridging
- Gasless claim requests

The full code is open source and maintained at:
[https://github.com/constata-eu/asami](https://github.com/constata-eu/asami)

This architecture ensures:
- Modularity: other Campaign Managers can deploy their own version
- Transparency: behavior and scoring can be independently reviewed
- Extensibility: new features such as category gating, referral tracking, and off-chain polling are easily supported

### 6.5 Gasless Claims and User Onboarding
To onboard users who are new to Rootstock, the Campaign Manager offers a **gasless withdrawal** feature. Collaborators:
- Approve a 1 DOC fee deduction
- Have their rewards claimed by the Campaign Manager on their behalf
- Receive both their DOC payout and a small RBTC amount to cover future gas costs

This mechanism makes onboarding seamless for non-crypto users and serves as a type of RBTC faucet while preserving full ownership over rewards.

### 6.6 Payment Gateway Integration
To improve the Advertiser experience, Campaign Managers may integrate fiat onramps like **Stripe** to:
- Accept credit card or bank payments
- Automatically convert funds into DOC
- Fund campaigns directly from Web2 platforms
- A 20% processing fee applies to cover costs and discourage off-chain dependency.
- Campaigns funded via Stripe do not issue ASAMI tokens.

This further lowers the barrier to entry for new advertisers.

---

## 7. Governance

Asami is not governed by a company, a DAO, or a social contract. It is governed by what is **enforceable in software** and what is **voluntarily chosen by participants**.

There are no obligations between participants beyond what is encoded in the smart contract. The protocol has no built-in authority, committee, or process for making decisions on behalf of others. All roles are opt-in, and all behavior is bounded by code and incentives.

This minimalist approach avoids the complexity, politics, and ambiguity that typically emerge in systems with layered governance structures. Instead, it embraces transparency, modularity, and economic alignment. If someone wants something to work differently, they must:

* Write software that does it
* Convince others to use it
* Or fork the protocol and offer an alternative

Asami’s governance model is narrow by design. Its goal is to support **open collaboration without expectation**, and to **keep power and responsibility localized** to the role that exercises them.

### 7.1 Voluntary Participation and Role Boundaries

Every participant in Asami.club chooses their role freely and assumes no obligations beyond what is expressed in the smart contract or the public interface. These roles — **Project**, **Advocate**, **Campaign Manager**, and **Token Holder** — are **independent**, **replaceable**, and **non-hierarchical**.

There is no interpersonal governance, no central team, and no support obligation between roles. This section outlines what each role can expect, and what they are **not** entitled to.

#### Projects

* Projects can create campaigns to reward visibility and recruit advocates.
* They can choose whether to whitelist, blacklist, or remain open to all eligible Advocates.
* They are responsible for selecting a **Campaign Manager** to register their campaign on-chain.
* By using the default website (asami.club), they are selecting the **default Campaign Manager** currently operating the platform.
* They cannot demand custom scoring, explanations, or changes to campaign manager decisions.
* They are not entitled to refunds, even if a campaign yields few or no reposts.
* Projects funding campaigns through off-chain methods (e.g., Stripe, LN) may not receive ASAMI tokens.

#### Advocates

* Advocates can choose to repost campaigns that match their interests and values.
* Their influence is assessed using a public algorithm and recalculated periodically.
* Advocates are **not guaranteed participation**, payment, or visibility in any given campaign.
* If they feel miscategorized or under-scored, they may submit a detailed report via the Asami robot — but the Campaign Manager retains discretion over scoring.
* Advocates are free to stop participating or pursue other platforms at any time.

#### Campaign Managers

* Campaign Managers are **not part of the smart contract**. They are **off-chain entities** that perform operational responsibilities:
  * Scoring Advocates
  * Registering campaign and collaboration data
  * Managing frontends and providing support
* **Any person or group** with technical capabilities can become a Campaign Manager and run their own frontend on top of the smart contract.
* There is currently **only one Campaign Manager** running on asami.club. Its operation is subsidized, as the infrastructure costs exceed what is collected in fees.
* Campaign Managers are **not required** to explain scoring decisions or provide support. They retain full discretion over how to run their operations.

#### Token Holders

* Token holders vote on the protocol fee rate, which governs the flow of new ASAMI tokens.
* They have no authority over campaigns, scoring, or platform operations.
* Their only influence is over the **economic balance** of the protocol through voting.

### 7.2 The As-Is Model and Its Consequences

There are no service-level agreements between parties. The protocol exists as-is, and participants agree to operate within its boundaries. The roles of Project, Advocate, and Campaign Manager each have clear limitations:

#### Projects

* Are not entitled to support from Campaign Managers.
* Are not entitled to refunds or explanations if a campaign receives no reposts.
* Can be denied service at the sole discretion of the Campaign Manager.
* Are not entitled to token issuance when funding campaigns through Stripe or non-rootstock methods.

#### Advocates

* Are not entitled to participate in any particular campaign.
* Are not entitled to explanations or adjustments to their score.
* Are not entitled to compensation unless they have successfully participated in a campaign as defined by the smart contract.
* Can leave the platform at any time.

#### Campaign Managers

* Are under no obligation to refund advertisers or re-score Advocates.
* Are not expected to respond to complaints.
* May change scoring criteria and structure as they see fit.
* May stop operating at any time.

### 7.3 Complaint Resolution

Asami.club minimizes obligations and keeps interactions transparent. Because there are no off-chain guarantees or interpersonal commitments, most complaints cannot be formally “resolved.” However, well-reasoned reports can still help improve the system over time.

If you believe something is wrong — for example, a mis-scored Advocate, an ineligible campaign, or unfair campaign visibility — you may submit a report for public review. To be considered:

* Reports must include **specific comparative evidence** with public links (e.g., X profiles, Asami profile pages, tweets, or stats).
* Vague, emotional, or purely subjective claims will not be addressed.
* Reports should focus on relative fairness: “This account received a reward for X, while mine did not, despite Y.”

Campaign Managers **may** consider these reports but are **under no obligation** to respond, explain, or make changes. The scoring algorithm is periodically updated, and constructive input may influence future versions — but no retroactive adjustments will be made.

If any participant — Project, Advocate, or Token Holder — is dissatisfied with how a Campaign Manager operates:

* They may stop using that Campaign Manager’s services.
* They may support or fund the creation of an **alternative Campaign Manager** who applies different rules, curates differently, or uses another scoring model.

The system **self-regulates through choice and transparency**, not coercion or appeal. The current Campaign Manager has **no special access or privileges** in the smart contract. Anyone with the technical means may register campaigns and collaborations, or build their own front-end to serve different users and preferences.

### 7.4 Fee Rate Voting

Asami does not support general-purpose governance. Token Holders have no say over who participates, how scores are assigned, or which Advocates or Projects are allowed.

However, they do vote on **one key parameter**: the **protocol fee rate**. This is the percentage charged to advertisers when they fund a campaign, and it determines how many ASAMI tokens are minted in response to those fees. A higher fee results in more tokens being issued, and a larger fee pool to be distributed.

#### How Voting Works

* Every 15 days, a new “contract cycle” begins.
* During each cycle, Token Holders may signal their preferred fee rate by voting directly on-chain.
* The system calculates a **weighted average** of all votes based on each voter’s token balance.
* At the end of the cycle, the selected fee rate is locked in and applied to the next period’s activity.

Example:

* If most Token Holders vote for a 20% fee, but a large whale votes for 10% with 60% of the token supply, the resulting fee rate will move closer to 10%, but still reflect the weighted average of all input.

Token holders can view the current fee rate, past votes, and upcoming cycles at: [https://asami.club/#/Token](https://asami.club/#/Token)

This is the **only governance mechanism in Asami**, and it is designed to:

* Let token holders indirectly influence issuance volume and incentive alignment.
* Avoid open-ended proposals, subjective interpretation, or social pressure.
* Keep governance minimal, mechanical, and predictable.

---

## 8. Communication, Transparency & Security

Asami is built to be open by default — not just in its on-chain protocol, but in the way it engages with Advocates, Projects, and Campaign Managers. Every aspect of the system is designed to encourage visibility, accountability, and public discourse.

Transparency fosters trust. But openness also creates opportunities for abuse, confusion, and edge cases. That’s why Asami balances its transparency with a modest but thoughtful layer of security measures, quality filters, and support guidelines — all shaped to protect the ecosystem without undermining its permissionless nature.

In this section, we describe how Asami communicates, how its data can be verified, and how it mitigates manipulation or misuse — all while maintaining the club’s spirit of voluntary, public collaboration.

### 8.1 Public-First Design

Asami.club communicates openly by design. Instead of relying on private chats or closed forums, the club maintains its presence and most of its discussion in public spaces — especially on X (formerly Twitter). This encourages accountability, reduces ambiguity, and helps new users understand how the system works simply by observing it in action.

Support requests are handled through clearly defined channels, and only for specific technical issues. The public Telegram support groups exist for cases where:

* The website is not working as expected.
* You cannot link your X account, email, or wallet.
* You registered a campaign or reposted one, but it doesn’t appear or hasn’t been paid after a reasonable delay.
* You attempted to create a campaign that never appeared.

All other questions must first be directed to the [Asami Robot](https://robot.asami.club), which can help advocates explore public data, understand influence scoring, and prepare a proper report. General commentary or feedback should take place publicly by mentioning the official club account (e.g., `@asami_club_es`) on X.

Support channels are not general-purpose helpdesks, nor are they places to debate influence scores or request campaign access. Moderators may ignore or remove messages that do not follow the pinned support guidelines.

This public-first approach is intentional. It mirrors the values of Web3 — where participants take initiative, explore open data, and interact with code and contracts rather than centralized authorities.

### 8.2 Transparency by Design

Asami.club is built on the principle of verifiability. While not every individual data point can be made public due to platform restrictions — such as X’s API terms — the system is designed to ensure that all structural decisions, core logic, and resulting outcomes are transparent and open to scrutiny.

Key components available for public inspection include:

* **Campaign metadata**: who created the campaign, which Advocates participated, what rewards were promised and paid, and how many impressions were delivered.
* **Advocate profiles**: publicly accessible on Asami, showing their influence score, campaign history, and earnings.
* **Token and revenue data**: total ASAMI tokens issued, fee pool balances, distribution events, and historical stats at [asami.club/#/Token](https://asami.club/#/Token).
* **Collaboration records**: including the campaign, the repost, the payout, and the timestamp — all verifiable and stored on-chain.

The **influence scoring algorithm is fully open source and versioned**, so anyone can understand how scores are calculated and how the system evolves. While the raw data used in scoring — such as impressions and engagement metrics from X — may not be shareable due to X’s terms, the methodology and results are auditable within those limits.

Campaign performance, token issuance, and participant contributions are all verifiable through public interfaces or on-chain inspection. There is no private or privileged backend; all users, projects, and third parties interact with the same transparent data.

This design encourages accountability without depending on trust in any specific actor.

### 8.3 Preventing Abuse and Misuse

Asami includes multiple layers of protection to reduce manipulation, spam, and exploitative behavior — particularly around influence scoring, campaign participation, and reward distribution.

Asami’s design does not eliminate abuse entirely, but it discourages low-effort engagement and rewards sincerity. Campaign managers and projects are empowered to maintain quality through both code and judgment.

#### Algorithmic Safeguards

* **Baseline scoring**: Advocates are pre-screened through an influence algorithm that filters out inactive or non-genuine accounts. Only those above a minimum score threshold can see and participate in campaigns.
* **Engagement ratios**: Influence scoring penalizes accounts with high impressions but little real engagement, discouraging artificial visibility or bot-driven metrics.
* **Verified signals**: The algorithm rewards verified followers and interactions with high-visibility posts. This makes it harder to inflate one’s score through fake accounts or superficial metrics.
* **Manual scoring inputs**: The Campaign Manager may supplement scoring with offline influence indicators (e.g., event organizing, community leadership), allowing for more nuanced and context-aware evaluation.

#### Platform Constraints

* **Score gating is mandatory**: Only accounts above a minimum influence score — as determined by the Campaign Manager — may participate in campaigns. This restriction is enforced globally and is not currently configurable by advertisers.
* **Budget and duration are one-way mutable**: A campaign’s budget and duration can be extended after deployment, but never shortened or reduced. Campaigns cannot be ended early, nor can funds be withdrawn once committed.
* **Off-chain campaign details**: Other campaign parameters (such as message content, language or category restrictions) are not stored on-chain. They are managed by the Campaign Manager and may be modified, within reason, for clarity, quality, or moderation purposes.

#### Social and Structural Checks

* **Advertiser discretion**: Projects may blocklist or whitelist specific Advocates after they engage with a campaign. Over time, this lets advertisers refine who can interact with their future campaigns.
* **Public incentives**: All rewards and reposts are public, encouraging moral responsibility. When an Advocate accepts payment, they acknowledge their role in amplifying the message — unlike traditional influencer marketing where compensation is often hidden.

### 8.4 Responsibility and Disclosure

Asami encourages responsibility and ethical behavior by making reposts and rewards public. When an Advocate republishes a campaign post, their public profile clearly displays:

* The content they chose to amplify.
* The fact that a reward was (or will be) received for doing so.
* The exact amount and type of that reward (DOC and ASAMI).

This level of disclosure is important for trust and accountability. Unlike “stealth” influencer marketing or undisclosed endorsements, participation in Asami campaigns is **transparent by design**. Projects and Advocates both benefit from a system where visibility and compensation are openly linked.

This structure has a secondary effect: by accepting a public reward, Advocates **implicitly take some responsibility** for what they help promote. The reward is not just a thank-you — it’s a signal that the Advocate made a considered choice to associate with a project. This sets Asami apart from platforms that allow anonymous or hidden promotions with no accountability.

Advocates are encouraged to only promote what they truly believe in. While the reward is modest, the act of accepting it signals trust and intent, not blind endorsement — a distinction that strengthens the credibility of the entire ecosystem.

---

**9. Competitive Landscape**

Web3 projects have a wide variety of tools at their disposal to reach new users, increase visibility, and incentivize participation. From airdrops to quests to viral social campaigns, the ecosystem for growth is rich and varied — and for good reason. Different goals require different tactics.

Asami is not a replacement for these tools. It does not attempt to onboard users through gameplay, or reward one-time interactions. Instead, it focuses on a specific but often overlooked aspect of Web3 adoption: **advocacy**. People who learn, believe, and share — not for money, but because they want others to understand what they’ve discovered. These are the explainers, the popularizers, the recommenders. Their role in project discovery is essential, and Asami gives them a modest, transparent way to be appreciated and acknowledged for it.

Importantly, Asami complements other growth tools. For example, a project might use Galxe or TaskOn to run incentivized campaigns that reward users for trying a product. At the same time, it can use Asami to reach real advocates who **share the campaign** with their audience, adding credibility and reach. This layered approach can amplify the effectiveness of traditional strategies while grounding them in authentic community support.

Asami’s unique value lies in how it turns advocacy into a visible and traceable interaction — creating a public record of who is supporting what, and when, while keeping incentives humble and reputationally binding.

### 9.1 Web3 Marketing Today

Marketing in the Web3 space faces unique challenges. Traditional advertising channels often impose restrictions on crypto-related content, with some platforms outright banning or shadowbanning blockchain promotions. Even when permitted, ads for decentralized projects can appear untrustworthy or irrelevant to audiences unfamiliar with Web3 concepts.

To work around this, many projects rely on **incentive-driven platforms** that reward users for specific actions: installing a wallet, trying out a dApp, joining a community, or completing a task. These strategies can generate large numbers of interactions and users in a short amount of time. However, they rarely result in **long-term retention** or **authentic interest**. Participants often complete tasks to earn rewards and leave shortly afterward, inflating metrics without building lasting communities.

Meanwhile, most of the actual growth seen in foundational Web3 projects — from Bitcoin to open-source infrastructure — has been driven not by advertising, but by early **advocates**: individuals who took the time to explain, recommend, and promote what they believed in. These Advocates shaped discourse, answered questions, and helped others take their first steps into Web3. Yet, historically, this labor of belief and communication has gone largely unrecognized.

Web3 needs both kinds of effort: scalable user onboarding and long-term, credible community building. Asami focuses on the latter.

### 9.2 Incentivized Actions vs. Genuine Advocacy

Platforms like Galxe, TaskOn, and Zealy reward users for completing predefined actions — such as signing up, using a feature, or joining a community. These tools are useful for onboarding and can quickly boost engagement, but the resulting activity is often transactional and short-lived.

Asami offers a complementary approach: it supports those who **already advocate** for projects — not because they were told to, but because they believe in the ideas. Rather than incentivizing a specific behavior, Asami publicly acknowledges and modestly rewards existing supporters who amplify campaigns through reposts.

This distinction matters. Incentivized actions are effective at sparking participation; Asami focuses on **recognizing belief**. The two approaches can work together: for example, a project can launch a quest and use Asami to spread the word through trusted voices — giving the campaign both reach and credibility.






































---

## 9. Community Growth

The long-term success of Asami.club depends on the sustained growth of its community. Every role in the system—Campaign Manager, Collaborator, Advertiser, and Token Holder—benefits when the network grows, more campaigns are launched, and more users engage.

While the most direct incentives exist for **Campaign Managers**, who are rewarded for every registered collaboration and earn the largest share of ASAMI tokens, other participants also have reasons to help grow the ecosystem:

- **Advertisers** may find better value in Asami compared to traditional platforms, especially when promoting products in industries that are restricted or under-served by Web2 ad networks. Some Advertisers may act as intermediaries or agencies, facilitating campaign creation for others, earning a margin and collecting ASAMI tokens in the process.

- **Advocates** benefit from an expanding platform with more campaigns, better-paying opportunities, and more visibility. They may also collectively operate community-owned accounts, growing influence as a group and splitting rewards. They can use Asami as one of multiple revenue streams tied to their online presence.

These entrepreneurial possibilities make it so **any participant can effectively become a business unit** within Asami: attracting campaigns, onboarding collaborators, or even offering adjacent services like content creation or social media analytics.

### 9.1 Incentives and the Freerider Problem

Like many decentralized systems, Asami faces the classic challenge of growing a shared, open resource: **those who promote or fund its growth create value that benefits all participants**, including those who did not contribute.

This "freerider" problem is a common limitation in open networks. However, Asami introduces an elegant partial solution:

- By purchasing and holding **ASAMI tokens**, any stakeholder funding community growth is also gaining a proportional claim on future fee-based revenue from the protocol.
- If their efforts are successful, and more campaigns and collaborations are processed, the value of their tokens increases both through price appreciation and DOC-based revenue sharing.

This means that **promoting Asami is not charity—it can be an investment**. Stakeholders who build audiences, attract campaigns, or onboard collaborators will often see a return on their efforts if they also hold tokens.

In this way, Asami blends altruistic community building with financial incentives, creating an ecosystem where growth is in everyone’s interest, but not everyone has to contribute equally. This ensures flexibility while maintaining a path toward sustainability.

### 9.2 Supporting Infrastructure

Community growth efforts are also being supported by external partners like **Rootstock Labs** and **Rootstock Collective**, who have contributed infrastructure, promotional support, and funding.

The current Campaign Manager is actively seeking traditional investment based on a clear business model, further professionalizing this critical role within the protocol. At the same time, the door remains open for additional Campaign Managers to emerge and contribute to growth in parallel or in competition.

---

## 10. Competitive Landscape

Asami operates in a space where multiple strategies for growth, influence, and marketing overlap. While its focus is on recognizing and rewarding Web3 advocacy through transparent, modest incentives, it may intersect with tools designed for advertising, onboarding, monetization, or influencer management.

This section outlines the categories of platforms Asami is often compared to, and clarifies to what extent they represent competition, complement, or simply serve a different purpose. In many cases, Asami can be used in addition to these other methods — especially when a campaign requires both visibility and credibility among audiences familiar with Web3.


### 10.1 Decentralized Influencer Marketing Platforms

Projects like **Chirpley** and the now-defunct **D-Cent** attempted to decentralize influencer marketing by connecting brands with social media users in a peer-to-peer way. Chirpley, in particular, focuses on enabling advertisers to select from a marketplace of pre-categorized micro-influencers who define their own target demographics and niches, such as beauty, gaming, or travel.

In contrast, **Asami emphasizes organic discovery**. Advertisers don’t select participants upfront — instead, campaigns are shown only to pre-scored Advocates, and anyone who reposts a campaign becomes visible to the advertiser. This allows projects to identify aligned advocates over time and selectively whitelist them in future campaigns. The approach balances **exploration and exploitation** — discovering new supporters while cultivating a trusted group.

Chirpley prioritizes high-resolution audience targeting and broad industry support, while Asami focuses exclusively on Web3 and leverages its influence scoring algorithm to maintain a filtered, high-quality pool of potential Advocates. Targeting within Asami is done manually by Campaign Managers, based on public behavior and language use — not through self-reported interests.

Campaigns in Chirpley are executed through defined tasks, while Asami prioritizes advocacy, visibility, and long-term community growth. In joint strategies, a project might use Chirpley to execute a specific set of influencer engagements, while using Asami to **identify and reward the Advocates who genuinely believe in the mission**.

Finally, the **ASAMI token is not used as a payment method**, but as an equity-like reward for early participants. It has a fixed supply of 21 million and is only distributed through protocol activity during the issuance phase. Payments to Advocates are done in stablecoins (DOC), ensuring stability for projects and clarity for recipients.

### 10.2 Social Media Platforms with Monetization Features

Traditional platforms like **Facebook**, **Instagram**, **X (formerly Twitter)**, and **TikTok** have built-in monetization tools, allowing creators to earn revenue through sponsored content, ad revenue sharing, or direct support from followers. These platforms dominate attention and offer scalable opportunities for creators aiming to turn their audience into a full-time income stream.

Asami operates differently. It does not aim to provide primary monetization for creators. Instead, it targets those who act as **advocates and educators** — individuals who share meaningful content about Web3 projects out of interest, belief, or a desire to spread awareness. These users are rewarded modestly in DOC for reposting campaigns that align with their values. The goal is not to generate a livelihood, but to offer acknowledgment and token-based participation in growing communities.

Advocates using Asami may still rely on mainstream monetization platforms to support their broader work, such as YouTube ad revenue or TikTok sponsorships. In this sense, **Asami complements other channels**, offering a lightweight and transparent layer of incentive for sharing content they already believe in.

### 10.3 Blockchain-Based Social Media Networks

Several platforms blend blockchain infrastructure with social media to enable tokenized content monetization, censorship resistance, and community ownership. Here’s how they compare to Asami:

* **Hive** (formerly a fork of Steem) is a decentralized blockchain social network where users earn HIVE or HBD tokens through content creation and curation via the “Proof-of-Brain” mechanism. Its core focus is on rewarding on-chain publishing and voting, not on issuing crypto rewards for reposting third-party content. Unlike Hive, Asami operates on **existing social networks** (like X), rewarding passionate advocates for amplifying Web3 campaigns with modest stablecoin (DOC) payouts and token rewards. It does **not** require users to migrate to another network or publish original content on-chain.

* **DeSo** (formerly BitClout) allows users to trade creator coins tied to personal reputation; it emphasizes speculative behavior and creator-to-token speculation, rather than structured advocacy. Asami, in contrast, measures and rewards **practical social impact** via reposts and calculated influence.

* **Lens Protocol**, **Farcaster**, and similar platforms enable decentralized identity and future potential for social content, but still lack the scale and campaign framework Asami has on mainstream platforms. Asami focuses on generating measurable impact on major social networks and could eventually extend to decentralized platforms when they reach sufficient scale.

By integrating with established social channels like X and using open influence scoring, Asami delivers **campaign-aligned advocacy rewards** without redefining social structures — unlike Hive or DeSo, which aim to rebuild entire social ecosystems on-chain.


### 10.4 Platforms Selling Reposts to Advertisers

Platforms like **WeAre8**, **SocialBoost**, **InstaFollowers**, and similar services provide a marketplace for paid reposts, likes, and engagements—often prioritizing volume over quality.

* **WeAre8** compensates users for interacting with sponsored content, distributing ad revenue among participants. It focuses on scale and broad engagement, but does not vet for genuine interest or domain expertise.
* Other services, such as **SocialBoost** and **InstaFollowers**, offer repost packages, follower growth, or engagement enhancement—generally without transparency, accountability, or regard for audience relevance.

**Asami differs fundamentally**:

* Engagements are **not purchased en masse**—they are earned through the authentic reposting of campaigns by verified Advocates.
* All reposts and rewards are **public and traceable** on-chain, making manipulative activity far less likely and easier to identify.
* Each Advocate is score-gated and screened for actual influence—not just accepting payment for visibility.

For advertisers, Asami offers **quality over quantity**—modest, targeted amplifications from people genuinely interested in Web3. It can serve as a **pre-screening layer** before using mass engagement tools—or as a **complementary layer** that adds credibility to broader marketing efforts.


### 10.5 Projects Measuring Social Media Influence

Platforms like **FeiXiaoHao** and research initiatives such as **TweetBoost** analyze social metrics to quantify influence, sentiment, or content virality. FeiXiaoHao provides comprehensive social data tracking in the crypto ecosystem — including mentions, engagement rates, and keyword sentiment — helping users and investors spot trends and market mood. TweetBoost examines how Twitter metrics like follower reach, retweets, and likes impact NFT valuation — finding that Twitter activity improves prediction models by about 6%.

These projects overlap with Asami in their interest in measuring influence, but differ in scope and intent. FeiXiaoHao and TweetBoost focus on **analytics and insight**, serving researchers, investors, and teams seeking broader signals. Their influence models are generalized tools for sentiment analysis, not campaign-specific scoring engines.

By contrast, **Asami’s scoring algorithm is purpose-built**: it evaluates advocates on X using metrics tied directly to reposts, impressions, engagement ratios, and periodic scoring. The outcome drives **active campaigns and transparent rewards** — not just data insight. While a project might use analytic tools for strategic planning, Asami lets them **turn influence measurements into real-world outreach**, identifying which voices can actually amplify their message through advocacy.

### 10.6 Centralized Social Media Ad Networks

Traditional ad networks—such as **Google Ads**, **Meta (Facebook/Instagram Ads)**, and **X’s promoted posts**—offer broad reach and sophisticated targeting tools. However, they also pose specific challenges for Web3 projects:

* **Regulatory limitations and compliance burdens**: Many platforms impose strict restrictions on crypto-related ads. For example, Google and Meta require advertisers to meet licensing and country-level guidelines before allowing crypto promotions.
* **Platform-level bans**: TikTok, in particular, has repeatedly prohibited crypto ads globally due to consumer protection concerns.
* **High costs and limited trust signals**: As projects contend with distrust or ad fatigue, platforms like X have seen reduced quality in paid content, and campaigns are growing more expensive with little assurance of engagement.

**Asami offers a complementary alternative**: rather than buying impressions, it facilitates **trusted amplification** through known-and-verified advocates. A repost from a credible advocate is more likely to be seen as genuine than a promoted post. This approach can improve **ad delivery**, **engagement rates**, and **brand trust**, especially during uncertain times. For Web3 projects, Asami provides a **last-mile trust layer** that traditional ads cannot replicate due to both technical and policy constraints.

---

## 11. Future Directions

Asami remains a work in progress, guided by the evolving needs of Web3 projects and the growing community of advocates that support them. This section outlines areas of planned or ongoing development that aim to improve fairness, flexibility, and reach — while remaining true to Asami’s purpose: to reward genuine advocacy modestly and transparently.

### 11.1 Influence Scoring Improvements

Future versions of the scoring algorithm will refine how authority and audience are measured, especially on a per-action basis. Quote tweets, mentions, and post-specific engagement may soon be weighted independently. This will make it possible to reward advocates not only for their overall presence but also for the performance of individual reposts, replies, or mentions. Further techniques to detect reverse influence or manipulation are also under consideration.

### 11.2 Expanded Advocate Activity Types

Asami currently rewards reposts and quote tweets equally, but other content types — such as replies or mentions — may become eligible in the future. These will be considered for their reach, intent, and verifiability, and may be scored or gated differently based on campaign requirements.

### 11.3 Support for Other Platforms

While the current focus is on X (Twitter), Asami aims to integrate other platforms such as Nostr, LinkedIn, and Instagram. Additional targets include Farcaster, Threads, and Telegram — provided they allow public post visibility and permit reliable data collection. Asami will prioritize platforms aligned with Web3 ideals and open data access.

### 11.4 Multi-currency Support (BTC & Lightning)

Campaign funding via BTC and Lightning Network is under development. These payment methods will be converted transparently into DOC to maintain consistency in reward logic and fee attribution. Compared to Stripe, they will incur lower processing fees, but campaigns funded this way will not trigger ASAMI token issuance to participants.

### 11.5 Advocacy Culture and Outreach

Beyond technical improvements, Asami is committed to fostering a culture of advocacy. This includes educational initiatives to help people become better advocates, support for public discussions about scoring and fairness, and content explaining how to participate effectively. Asami will also invest in outreach and promotion to grow awareness and bring more people into the club — especially those who are already acting as organic amplifiers of Web3 ideas.

---

## 12. Legal Considerations

Asami is a protocol and platform that facilitates the voluntary collaboration between advertisers and social media users ("advocates") in a transparent, decentralized environment. It is not a company, foundation, or legal entity. While software and infrastructure may be maintained by identifiable individuals or organizations, the protocol itself operates on immutable smart contracts and open governance through token issuance.

### 12.1 No Investment Offering

ASAMI tokens are not offered as an investment. They represent participation in the Asami.club protocol and are issued to contributors who support the ecosystem by either promoting content (advocates), managing campaigns (campaign managers), or providing funding (advertisers). The token has a fixed supply and is distributed according to predetermined rules encoded in a smart contract. There is no promise of profit, nor any entity guaranteeing appreciation in value.

### 12.2 No Guarantees or Warranties

Asami provides no service-level guarantees to any participant. The behavior of the protocol is defined entirely by its code, and all interactions with it are at the discretion and risk of the participants. There is no legal obligation for campaign managers to score fairly, respond to complaints, or issue refunds. Complaints must be resolved socially, reputationally, or by migrating to alternative infrastructure built on the protocol.

### 12.3 Compliance and Local Laws

Users are responsible for complying with their local regulations, including but not limited to:

* Disclosing paid promotions when legally required.
* Paying taxes on income received through campaigns.
* Respecting platform terms of service when interacting with third-party sites like X (Twitter).

Asami does not and cannot enforce these requirements but encourages responsible participation and compliance.

### 12.4 Privacy and Data

Asami itself does not collect user data beyond what is published on-chain. Campaign managers and other participants may collect or analyze public social media data to assess influence. All such data use must respect the terms and limitations of the platforms where the data originates (e.g. X’s API terms). Internal datasets used for scoring may not be shareable, even if the scoring algorithm is public. Campaign managers may store personal or pseudonymous data to improve services or maintain scoring history, but this data is deletable upon request. If you have questions or concerns about your data, do not hesitate to contact **[dpo@asami.club](mailto:dpo@asami.club)** for guidance.


## 13. Final Notes

Asami.club is a continuously evolving experiment in building a more transparent, modest, and community-oriented form of social media advocacy. It doesn't claim to solve every marketing or influencer problem, nor to replace the massive infrastructure of centralized ad networks. Instead, it offers an alternative — one that starts small, rewards real people, and encourages genuine participation in the growth of Web3 projects.

The protocol is public. The smart contracts are permissionless. The club is open to all.

If you're an advocate, try participating in a campaign.

If you're an advertiser, try funding one.

If you're a builder, try creating your own interface to the protocol.

The system will become what we collectively make of it.

---

## Appending A: Contract Addresses and Links { .no-cols }

**Asami Smart Contract** (Rootstock):  
  https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954

**Current Campaign Manager Address**:  
  https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa

**DOC Stablecoin Info (Money on Chain)**:  
  https://moneyonchain.com/

**Open Source Campaign Manager Software**:  
  https://github.com/constata-eu/asami

---

## Appendix B: Contact and Support { .no-cols }
Members are encouraged to ask questions or request help via public channels. We aim to keep all communication transparent and community-facing.

**X (Twitter):**
- English: [@asami_club](https://twitter.com/asami_club)
- Spanish: [@asami_club_es](https://twitter.com/asami_club_es)

**Telegram:**
- English group: `@asami_club`
- Spanish group: `@asami_club_es`

Please note that all reviews and scoring requests must follow the evidence-based process described in this whitepaper, which also contains an FAQ appendix.

---

## Appendix C: FAQ and Known Issues { .no-cols }

**Q: Why am I not getting any campaigns?**
- Your influence score may be low, or you may be missing required categories. You can request a review.

**Q: Why does my score seem too low?**
- Read the Influence Measurement section and follow the dispute resolution process outlined in Section 3.2.

**Q: I used to get paid more than I do now. Why?**
- This may be due to lower advertiser budgets or changes in your score/category. It could also be due to reduced platform activity or increased competition.

**Q: I was banned or excluded by the Campaign Manager. Can I still participate?**
- Yes. Asami is permissionless. You can work with another campaign manager or use your ASAMI tokens normally.

**Known Issue:** Collaborations may be registered after a campaign runs out of funds. The platform will show them as "failed", but may compensate later if funds are added.

---
