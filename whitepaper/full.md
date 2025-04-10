# Asami Club Whitepaper

_This whitepaper describes the design, operation, and intended direction of Asami.club—a decentralized collaboration protocol for monetizing genuine social media influence. It includes components that are already deployed as well as features currently under development. The document is designed to be both human-readable and usable as input for language models that can answer questions about how Asami works._

---

## 1. Overview

Asami.club is a decentralized protocol that allows anyone to fund or earn from trend creation on social media. At the heart of the system is a simple but powerful mechanism: an Advertiser sets aside a USD-denominated budget to amplify a message, and Collaborators earn a portion of that budget by reposting it on their own X (formerly Twitter) accounts.

This budget is held and paid out in DOC (Dollar on Chain), a Bitcoin-backed stablecoin, ensuring payments are reliable and not exposed to crypto market volatility. DOC is the main economic reward that Collaborators receive—it's how they are paid for their work, directly, and in real time.

Alongside DOC payments, all participants also receive ASAMI tokens. These tokens serve multiple purposes: they act as a signal of alignment with the club, grant governance rights over the protocol's fee rate, and provide revenue sharing from the fees collected. This dual reward system gives both immediate (DOC) and long-term (ASAMI) incentives to participate.

The protocol is implemented as a smart contract on Rootstock (RSK), a Bitcoin sidechain that supports Ethereum-compatible smart contracts. Rootstock was chosen for its security model (merged-mined with Bitcoin), its uptime record, and its use of Bitcoin as native currency. All campaign rewards and budgets are in DOC, while ASAMI tokens are distributed based on the fees retained.

---

## 2. History and Motivation

The Asami project was created in response to two simultaneous shifts: the growth of decentralized social media and the increased accessibility of on-chain microtransactions. Together, these developments made it possible to imagine a new kind of media economy—one where individuals are rewarded for the influence they already exert in their communities.

Today, most social media platforms reward content creators who can generate attention, but ignore or exploit the role of others in spreading that content. Likes, reposts, and endorsements—which are fundamental in shaping what becomes visible—are rarely compensated.

Worse, social media platforms often place ads on creators' content without giving them control over the brands being promoted. The advertiser always maintains brand safety (they avoid creators who could damage their image), but the creators themselves have no equivalent safety in return. They may end up promoting brands they disagree with, just because the platform inserts the ad.

Asami inverts this model. It starts from the premise that every user who amplifies content is creating value, and should be compensated for that value. And it does so in a way that lets each Collaborator choose exactly what messages they want to support. This ensures alignment between the message, the creator, and the advertiser—all without requiring a central platform to arbitrate who gets what.

---

## 3. Roles and Stakeholders

### 3.1 Advertisers
**Who they are:** Anyone who wants to promote a post. This can include individuals, projects, startups, agencies, brands, or fans supporting someone else.

**What they do:** An Advertiser creates a "campaign" by selecting a post (usually from X), defining a total DOC budget, setting a duration, and specifying how rewards are distributed among Collaborators based on their influence scores.

**Rights:**
- Decide how much to pay per unit of influence.
- Set minimum and maximum payouts to ensure the budget is distributed widely.
- Add more funds or extend a campaign at any time.
- Withdraw unused funds once a campaign ends.
- Choose a Campaign Manager for each campaign and switch freely from one campaign to another.
- Decide which accounts are allowed or blocked from participating in their campaign, using allow/reject lists with the help of the Campaign Manager.

**Obligations:**
- Accept that once a campaign is live, it cannot be canceled.
- Trust the Campaign Manager to fairly register reposts and apply the influence scoring method.

**Additional Notes:**
Advertisers are not guaranteed Collaborators will repost their message. Collaborators choose campaigns voluntarily, creating a self-filtering system where only aligned users amplify a message.

---

### 3.2 Collaborators
**Who they are:** Social media users with real audiences who choose to amplify messages and earn for doing so.

**What they do: Collaborators browse the list of active campaigns for which they are eligible and decide which messages they want to repost. When they repost a campaign’s post, their action is registered by the Campaign Manager and they receive a payment in DOC.

**Rights:**
- Choose which campaigns to repost. No one is forced to participate.
- Get paid in DOC for successful reposts, based on their influence level and the campaign's payout rules.
- Receive ASAMI tokens as part of protocol revenue sharing.
- Request a review of their influence score or category assignment (e.g., language), following a defined process.

**Obligations:**
- Maintain reposts for a minimum period. Premature deletions may affect future eligibility.
- Operate a legitimate X account with genuine activity. Accounts that only repost without original contributions, or show signs of artificial engagement, may be excluded.

**Limitations and Important Details:**
- Collaborations are registered on a first-come, first-served basis. Once a campaign runs out of funds, no further payments are made.
- If a repost is valid but unfunded, it may be marked as "unpaid" and still earn ASAMI tokens, or be eligible for future rewards if funds are added later.
- Collaborators may be filtered out from certain campaigns due to low scores, missing categories, or other factors set by Advertisers or the Campaign Manager.

**Disputing Influence Scores:**
If a Collaborator believes their influence score is inaccurate, they must:
- Read the Influence Measurement section of this whitepaper.
- Identify the exact part of the scoring algorithm they believe has been incorrectly applied.
- Compare their score to similar accounts.
- Provide evidence (e.g., engagement metrics, poll results, referral stats).

Vague complaints such as "my score is too low" will be redirected to this process. No review will be initiated without a clear, evidence-based request.

---

### 3.3 Campaign Managers
**Who they are:** Operators who manage campaigns, calculate influence, register reposts, and optionally provide additional services to the other parties.

**What they do:** The Campaign Manager is the main actor connecting Advertisers and Collaborators. They register reposts, run the influence scoring algorithm, enforce rules, and manage payments (including gasless withdrawals).

**Rights:**
- Implement and change their own influence measurement algorithms.
- Determine which Collaborators are eligible for each campaign.
- Register reposts submitted by eligible Collaborators and assign DOC rewards according to the campaign's influence-based payout structure.
- Maintain allow/deny lists of users, assign categories (e.g., language or region), and validate engagement.
- Set an optional fee for their service, on top of the protocol's fee.
- Offer extra services (e.g., content writing, strategy) to Advertisers and Collaborators.

**Obligations:**
- Act within the boundaries of each Advertiser’s campaign rules.
- Use clear, reproducible influence measurement methods, and disclose the logic behind them.
- Be responsive to Collaborator inquiries, especially when questions arise about scoring or eligibility.

**Limitations:**
- Once a Collaboration is registered and funds paid, the action is final. They cannot claw back rewards, even if abuse is later discovered.
- Campaign Managers have no control over the Asami smart contract itself.
- If their decisions are considered unfair or unclear, they may lose the trust of both Advertisers and Collaborators, which may cause others to emerge as alternative Campaign Managers.

---

### 3.4 Token Holders
**Who they are:** Anyone holding ASAMI tokens. This includes all Collaborators, Advertisers, and Campaign Managers who have earned tokens.

**What they do:** ASAMI token holders participate in the long-term direction of the protocol by receiving revenue and voting on the fee rate.

**Rights:**
- Earn a proportional share of fees retained by the protocol.
- Vote on changes to the protocol’s fee rate using a weighted average system.
- Hold, transfer, or sell their tokens freely.

**Obligations:**
- None enforced by the protocol, but responsible voting and alignment with long-term value are expected.

**Additional Notes:**
- Token holders do not have direct influence over campaigns or roles.
- The only governance power currently available is the ability to vote on the protocol fee rate. See the Tokenomics section for more details on how ASAMI issuance and governance work.

## 4. Influence Measurement

Influence within Asami is calculated using a structured model that defines influence as the product of **audience size** and **authority**. This model ensures fairness and scalability while offering Campaign Managers flexibility to tailor scoring methods.

### 4.1 Audience Size

Audience Size is a quantitative measure of how many people actually see a user's posts. Initially, follower counts were used, but the current system relies on the number of tweet impressions over the past 45 days. This gives a much more accurate and real-time view of actual reach.

This measurement is pessimistic: no audience is assumed unless it can be proven via impressions. Impression counts can still be manipulated, so the system checks for statistical correlation between impressions and engagements (likes, comments, reposts). Accounts with abnormal ratios or signs of manipulation may have their Audience Size set to zero.

### 4.2 Authority

Authority reflects how much sway a user has over their audience, regardless of its size. A small but highly devoted audience may be more valuable than a large, indifferent one. The concept of authority is subtle and complex, and Asami approaches it using a multi-criteria system. Each criterion below contributes to the overall Authority percentage, which ranges from 0% to 100%. If no authority can be proven, the score is zero.

A Campaign Manager may apply these metrics automatically, subjectively, or via manual review. Here are the factors that contribute to the authority calculation, along with their intended purpose:

#### Engagement Received on X
This is the foundational requirement. If an account's posts generate no engagement from real users, it likely has no real influence.
- **None**: Suggests posts are ignored, or audience is composed of bots. Results in authority score of 0%, and all other criteria are skipped.
- **Average**: Shows regular interaction. Adds 25%.
- **High**: Indicates strong interest in the account. Adds 50%.

#### Direct Audience Polling
The user may post an anonymous poll asking followers how much they trust their recommendations. This gives insight into how people perceive the user's influence.
- **None**: No useful data. Adds 0%.
- **Average**: Indicates moderate trust. Adds 10%.
- **High**: Demonstrates loyal followers. Adds 20%.
- **Reverse**: Most voters report they would do the opposite of what the user says. This **divides** the "Engagement Received on X" authority score in half.

> Example poll: 
> _"If I recommend something, what do you usually do? a) Follow it blindly, b) Consider it, c) Ignore it, d) Do the opposite."_

#### Engagement Outside X
Some individuals are influential in other contexts: they run podcasts, write books, lead communities, etc. This accounts for off-platform reputation.
- **None**: Adds 0%.
- **Average**: Public presence verified. Adds 5%.
- **High**: Established and respected figure. Adds 10%.

#### Status on X
The account’s operational status affects its credibility.
- **Banned/Shadowbanned**: Influence is set to 0% and all other criteria are skipped.
- **Normal**: No change.
- **Enhanced**: Verified, premium, or trusted status. Adds 10%.

#### Referral Authority
Users who successfully invite others to join Asami demonstrate influence, particularly if those referrals are high-quality.
- **Valid referrals from active accounts**: Adds 10%.

#### Token Holding Behavior
Because Campaign Managers and Advertisers are paid in ASAMI, they may prefer to reward users who are aligned with the club's success.
- **Holding ASAMI tokens instead of selling**: Adds 10%.

### 4.3 Authority Score Calculation

The final Authority percentage is calculated as follows:
- Start with **Engagement on X**. If None, total score is 0%.
- If Average or High, add the base (25% or 50%), then apply:
  - **Audience Polling**: Adds 0%, 10%, 20%, or divides the score in half if Reverse.
  - **Off-X Engagement**: Adds 0%, 5%, or 10%.
  - **X Status**: Adds 0% or 10%, unless Banned, which sets to 0%.
  - **Referrals**: Adds 10% if criteria met.
  - **Token Holding**: Adds 10% if criteria met.

The Authority score is then multiplied by the Audience Size to calculate the final **Influence Level** used in reward distribution.

This layered system aims to be fair, transparent, and resistant to manipulation, while still allowing for evolution as new signals become useful.

---

## 5. Tokenomics and ASAMI Distribution

The ASAMI token is the native asset of the Asami.club protocol. It has a fixed maximum supply of **21 million tokens**, similar to Bitcoin, and is designed to distribute ownership of the protocol’s revenue and growth.

### 5.1 Revenue Sharing and Incentives

Every time a Collaborator is paid in DOC for reposting a campaign, **10%** of that payment is retained by the protocol as a fee. These fees, collected in DOC, are pooled and distributed every 15 days to all ASAMI token holders. The amount each holder receives is proportional to the number of ASAMI tokens they hold.

> 📥 For example, if you hold 1% of all ASAMI tokens, you will receive 1% of the revenue pool collected in that period.

This makes holding ASAMI tokens attractive as a form of **passive income**, since it gives access to future revenue generated by ongoing campaigns.

> 💡 Holding ASAMI is like owning a share in a decentralized ad economy. The more campaigns run through the protocol, the larger the revenue pool becomes.

This structure incentivizes all stakeholders to grow the platform:
- **Advertisers** fund campaigns and receive ASAMI tokens as part of each campaign payout.
- **Collaborators** earn ASAMI in addition to DOC rewards.
- **Campaign Managers** earn the largest share of ASAMI when they actively register collaborations.

### 5.2 Governance via Fee Voting

ASAMI token holders also have the ability to vote on the protocol's fee rate.

The voting system uses a **weighted average model**: each token holder submits a percentage they believe the fee should be (e.g., 5%, 15%, 30%). The final fee is computed by averaging all votes, weighted by the number of tokens held.

> 🗳️ For example, if one user holding 10,000 tokens votes for 5% and another holding 1,000 tokens votes for 100%, the resulting fee will reflect the weighted average, not the median.

This tug-of-war design creates healthy dynamics:
- **Higher fees** result in more revenue sharing, but also more token inflation.
- **Lower fees** reduce earnings for holders but may attract more advertisers and collaborators.

This governance tool allows token holders to influence the long-term economics of the platform while balancing their own incentives.

### 5.3 Issuance Model and Fairness

The protocol targets issuing **100,000 ASAMI tokens every 15 days**, but this amount is adjusted dynamically based on:
- The DOC fees collected during the previous period
- The total ASAMI supply issued so far

Tokens are issued each time a campaign reward is paid. The fee retained by the protocol (10% of the DOC reward) is converted into ASAMI tokens using the current issuance rate. These tokens are then split among participants:
- **40% to the Campaign Manager**
- **30% to the Collaborator**
- **30% to the Advertiser**

> 🧮 **Example:**
> A campaign pays a Collaborator 20 DOC. The protocol retains 2 DOC (10%).
> If the issuance rate is 1000 ASAMI per DOC, then 2000 ASAMI are minted:
> - 800 go to the Campaign Manager
> - 600 to the Collaborator
> - 600 to the Advertiser

There was no **premine**, and no **special allocation** was made to insiders. The only large holder ("whale") is the current Campaign Manager, who earned roughly 40% of issued tokens simply by being the only manager to date and registering all collaborations.

This transparent and fair issuance system ensures that:
- Participation is rewarded proportionally
- Token distribution follows real platform activity
- Incentives remain aligned for long-term sustainability

By combining revenue sharing, voting power, and fair issuance, ASAMI tokens function as both a yield-generating asset and a tool for decentralized governance.

---

## 6. Technical Architecture

The Asami protocol is implemented as an open-source smart contract deployed on the **Rootstock** blockchain. Rootstock is a Bitcoin sidechain offering full EVM compatibility, which allows developers to use familiar Ethereum tooling while benefiting from Bitcoin’s security model.

### 6.1 Why Rootstock
Rootstock was chosen because it offers:
- **Merged mining with Bitcoin**, which enhances its security.
- **EVM compatibility**, enabling fast smart contract deployment.
- **Uptime and reliability**, with no known downtime and consistent block production.
- **Bitcoin-native environment**, which aligns with Asami’s decentralization goals.

Rootstock uses **RBTC** as its native gas currency and is a battle-tested network with low fees and stable operation, making it a solid foundation for Asami.

### 6.2 Campaign Transactions and Payments
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

### 6.6 Payment Gateway Integration (Planned)
To improve the Advertiser experience, Campaign Managers may integrate fiat onramps like **Stripe** to:
- Accept credit card or bank payments
- Automatically convert funds into DOC
- Fund campaigns directly from Web2 platforms

This will further lower the barrier to entry for new advertisers.

In sum, Asami’s technical architecture balances decentralization, transparency, and ease of use—powered by Rootstock’s stable and secure environment, and built for onboarding the next generation of trend creators and advertisers into Web3.

---

## 7. Governance

Asami operates under a decentralized and pragmatic model of governance. There are **no legal contracts** or off-chain obligations between members of the platform. All relationships between stakeholders—Advertisers, Collaborators, Campaign Managers, and Token Holders—are governed by the logic and constraints of the Asami smart contract and the practical discretion of each party. Participation is entirely voluntary and “as-is.”

### 7.1 Roles Are Defined by Action, Not Obligation
- **Advertisers** fund campaigns but are not required to include any specific Collaborators.
- **Collaborators** choose which campaigns to participate in and may decline any without explanation.
- **Campaign Managers** act at their own discretion and are not legally bound to act on behalf of any party beyond the behavior enforced by the smart contract.
- **Token Holders** may participate in governance, but are not owed any direct benefit by other participants.

### 7.2 The “As-Is” Model and Its Consequences
Because all interactions are permissionless and governed by code:
- There are **no service-level agreements** between parties.
- **Advertisers** have no guarantee that a Collaborator will participate in their campaign, or that the Campaign Manager will act in a specific way beyond registering valid collaborations and distributing rewards.
- **Campaign Managers** are under no legal obligation to refund advertisers or re-score Collaborators. Once funds are allocated to a campaign, the Campaign Manager manages them autonomously and is not liable to advertisers for how rewards are distributed, provided the process follows the smart contract rules.
- **Collaborators** are not guaranteed participation, visibility, or payment unless their repost is successfully registered and funded under an active campaign. Even then, payment cannot be reclaimed from them under any circumstances.
- Any **Advertiser** may exclude any Collaborator from their campaigns without justification.
- Any **Collaborator** may decline to participate in any campaign or leave the platform at any time.

### 7.3 Accountability Through Competition
If any party (Advertiser, Collaborator, or Token Holder) is dissatisfied with how a Campaign Manager operates:
- They may stop using that Campaign Manager’s services.
- They may **support or fund** the creation of an alternative Campaign Manager who follows different rules or applies a more agreeable scoring algorithm.

The system self-regulates through choice and transparency rather than coercion or appeal. The current Campaign Manager has no special access or privileges in the protocol. Any new participant who builds compliant tools can become a Campaign Manager and compete for campaigns and Collaborator attention.

### 7.4 Token-Based Parameter Voting
The only shared, protocol-wide governance function is the ability to vote on the **Asami protocol fee**. This fee is currently set at 10% and may be adjusted through a weighted average voting system:
- Token Holders submit a preferred percentage (0–100%) as their vote.
- The final value is calculated by weighting each vote based on the number of ASAMI tokens held.

> 📊 Example: One user holding 5,000 ASAMI votes for a 10% fee. Another user holding 500 ASAMI votes for a 50% fee. The outcome will be closer to 10%, based on the weighted average.

This voting mechanism aligns incentives:
- Raising the fee may increase revenue sharing, but also causes token inflation and may discourage Collaborator participation.
- Lowering the fee reduces revenue sharing but may improve platform growth and campaign appeal.

The protocol’s simplicity is its strength: clear incentives, minimal governance, and decentralized control through voluntary cooperation.

---

## 8. Security and Abuse Prevention

The Asami protocol is implemented as a smart contract. By itself, it cannot be abused, unless a software error or vulnerability is discovered in the contract code. It enforces rules deterministically, holds and disburses funds, and executes the logic of ASAMI issuance, DOC fee retention, and on-chain recordkeeping. It is neutral and agnostic about the quality of campaigns or the actors involved.

On the other hand, **Campaign Managers** operate the systems that evaluate eligibility, measure influence levels, register reposts, and distribute campaign funds. These systems are much more susceptible to manipulation and abuse, because they must rely on off-chain signals and data inputs from platforms like X (Twitter). Since Campaign Managers ultimately decide how campaign budgets are distributed among Collaborators, bad actors pretending to be genuine Collaborators may attempt to **trick or manipulate these systems** to receive funds without delivering value to Advertisers.

Because Campaign Managers aim to remain open to as many Collaborators as possible—encouraging growth and participation—they often rely on automated systems to onboard and evaluate members. This necessary scalability also introduces **attack surfaces** for manipulation, making robust abuse detection essential.

### 8.1 Types of Abuse

The most common and anticipated forms of abuse encountered by Campaign Manager systems include:

- **Artificial Inflation of Impressions**: Would-be Collaborators may attempt to boost their visibility through bots or purchased traffic. If impressions are not statistically consistent with engagement metrics like replies or likes, the Campaign Manager may assign zero audience value.

- **Low-Effort or Blind Reposting**: Some accounts repost every campaign without context or original engagement. These accounts dilute campaign value and are excluded or deprioritized.

- **AI or Script-Based Engagement**: Campaign Managers detect and ignore repetitive, inconsequential comments (“GM”, “🔥”, etc.) generated by scripts or AI. These are not meaningful indicators of real influence.

- **Reverse Influence**: Accounts that routinely provoke negative sentiment or backlash may be flagged using polling and sentiment analysis. Their influence is adjusted accordingly.

- **Off-X Influence Not Captured by Metrics**: Some influential individuals may not have strong X metrics but exert influence through external means (e.g., public speaking, podcasts). These cases require manual evidence review.

- **Referral Manipulation / Sybil Attacks**: Some users may try to gain referral authority by onboarding low-quality or self-controlled accounts. Referrals must lead to active, genuine members to be counted.

### 8.2 The Campaign Manager’s Role in Prevention

Each Campaign Manager implements its own algorithm for scoring, abuse detection, and collaboration eligibility. Their systems typically include:

- **Pessimistic Audience Scoring**: No audience is assigned unless impressions are corroborated by consistent engagement data.
- **Authority Calculation via Multiple Signals**: Engagement, polling, referrals, and token-holding behaviors all contribute to a Collaborator’s score.
- **Eligibility Filtering**: Collaborations are only accepted if the repost meets the manager’s quality, audience, and timing thresholds.
- **Flagging and Blacklisting**: Suspected abusive actors may be deprioritized or barred from participating in future campaigns.

Because Campaign Managers are not bound to accept every participant and must maintain Advertiser trust, they are empowered to use both **automated logic** and **manual review**. Their decisions are not enforced by the smart contract, but by discretion, trust, and competition. Ultimately, a Campaign Manager that allows abuse will lose Advertiser confidence.

### 8.3 Rights and Responsibilities of Collaborators

Collaborators who disagree with their influence score or category assignment may:
- Request a review with specific evidence.
- Compare their score with others via the public leaderboard.
- Provide off-chain evidence (e.g., podcast audience size, referral success).

However, they should understand that:
- Influence scores are **relative** and **not guaranteed rights**.
- Campaign Managers are not required to maintain or explain scoring decisions beyond what is stated in this whitepaper.
- Participation in campaigns is at the Campaign Manager’s discretion.
- Once earned, rewards (DOC or ASAMI) cannot be taken back.

### 8.4 Protocol Transparency and Limits

The Asami protocol enforces on-chain logic only:
- It issues ASAMI tokens
- Collects and distributes fees
- Records DOC payments and collaborations

It **does not validate repost quality or enforce influence measurement**. These decisions happen off-chain within each Campaign Manager's systems. However, all financial transactions are transparent and publicly viewable on Rootstock.

Even if a Collaborator is banned by a Campaign Manager, any previously earned DOC or ASAMI remains in their wallet and cannot be revoked.

### 8.5 Trust Through Competition

Campaign Managers have no privileged status in the Asami protocol. Anyone may create their own Campaign Manager, define their rules, build their own community, and serve Advertisers.

If stakeholders lose trust in a particular Campaign Manager, they can:
- Withdraw from that manager’s campaigns
- Create or fund an alternative manager with clearer standards
- Support Campaign Managers with different scoring philosophies or regions

The Asami ecosystem is self-regulating. Transparency, performance, and open alternatives are the mechanisms that keep abuse in check—not central enforcement or bureaucracy.

---

## 9. Community Growth

The long-term success of Asami.club depends on the sustained growth of its community. Every role in the system—Campaign Manager, Collaborator, Advertiser, and Token Holder—benefits when the network grows, more campaigns are launched, and more users engage.

While the most direct incentives exist for **Campaign Managers**, who are rewarded for every registered collaboration and earn the largest share of ASAMI tokens, other participants also have reasons to help grow the ecosystem:

- **Advertisers** may find better value in Asami compared to traditional platforms, especially when promoting products in industries that are restricted or under-served by Web2 ad networks. Some Advertisers may act as intermediaries or agencies, facilitating campaign creation for others, earning a margin and collecting ASAMI tokens in the process.

- **Collaborators** benefit from an expanding platform with more campaigns, better-paying opportunities, and more visibility. They may also collectively operate community-owned accounts, growing influence as a group and splitting rewards. They can use Asami as one of multiple revenue streams tied to their online presence.

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

In sum, Asami’s growth is grassroots, incentive-aligned, and decentralized by design. Anyone can play a part, and those who do have tools to capture the value they help create.

---

## 10. Competitive Landscape

The competitive landscape for Asami encompasses various platforms and initiatives that integrate blockchain technology with social media to enhance influencer marketing, content monetization, and community engagement. Below is an expanded overview of notable competitors and related projects:

### Decentralized Influencer Marketing Platforms

Several blockchain-based platforms attempted to facilitate direct interactions between brands and influencers, aiming to eliminate intermediaries and increase transparency, like the now defunct like D-creator, or 
[https://chirpley.ai/vision-and-mission/](Chirpley), which is more focused in the micro-influencer journey.

### Social Media Platforms with Monetization Features

Traditional social media platforms have integrated features that allow users to monetize their content, indirectly competing with Asami's model:

- **Facebook and Instagram**: Both platforms offer shoppable posts and stories, enabling influencers and brands to tag products directly in their content, facilitating seamless e-commerce experiences.

- **TikTok**: Known for its high engagement with younger audiences, TikTok provides opportunities for influencers to collaborate with brands through sponsored content and its Creator Marketplace.

### Blockchain-Based Social Media Networks

Some platforms combine social media functionalities with blockchain to offer decentralized content sharing and monetization:

- **Steemit**: A blockchain-based blogging and social media website where users earn cryptocurrency (STEEM) for publishing and curating content. This model incentivizes content creation and community engagement through token rewards.

- **BitClout (now DeSo)**: An open-source blockchain-based social media platform where users can buy and sell "creator coins" tied to individuals' reputations. It aims to decentralize social media and provide new monetization avenues for content creators.

### Platforms Selling Reposts to Advertisers

While not always blockchain-based, certain platforms specialize in facilitating the sale of social media reposts and engagements to advertisers:

- **WeAre8**: A social media network that compensates users for watching ads and engaging with content. It shares a significant portion of its revenue with users, promoting a more equitable distribution of advertising profits.

### Projects Measuring Social Media Influence

Accurately assessing social media influence is crucial for effective marketing campaigns. Some projects that focus on this aspect are:

- **FeiXiaoHao**: A blockchain data platform that analyzes social media metrics to provide users with insights into market sentiment, trending projects, and potential movements in the crypto space.

- **TweetBoost**: A study exploring the impact of social media on NFT valuation, highlighting the significant role of social media engagement in determining the value of digital assets.

### Emerging Trends and Considerations

The integration of blockchain technology into social media and influencer marketing is driven by the desire for greater transparency, security, and user empowerment. Decentralized platforms aim to address issues like data privacy, fair compensation, and content authenticity. However, challenges such as regulatory uncertainty, technical complexity, and the need for widespread adoption remain.

Asami distinguishes itself by offering a decentralized collaboration protocol that monetizes genuine social media influence, ensuring fair compensation and transparent interactions among advertisers, collaborators, and campaign managers. By leveraging blockchain technology, Asami aims to address common challenges in influencer marketing, such as fraud, lack of transparency, and inefficiencies in campaign management. 

---

## 11. Why Use Asami

Asami offers a new way for Advertisers and Collaborators to connect in a more authentic, fair, and decentralized environment. Traditional social media platforms prioritize their own revenue through intrusive ad placements, offering limited transparency, and often limiting which industries can advertise. Asami takes a different approach by directly connecting Advertisers with people who want to amplify their message—on their own terms.

### 11.1 For Advertisers

Advertisers use Asami because it allows them to:

- **Reach real people, through real people**: Instead of relying on platform algorithms or faceless banner ads, Asami enables your content to be voluntarily reposted by individuals who believe in what you're offering. This creates genuine endorsement and social proof.

- **Control brand safety**: With Asami, you choose the Campaign Manager and set your own rules for who can participate. You can use allowlists or blocklists to ensure your content only reaches audiences through Collaborators you trust.

- **Advertise where others can't**: Many industries, such as crypto or politically sensitive causes, face censorship or platform limitations. Asami gives you a space to run campaigns that might otherwise be restricted.
- **Get more value per dollar**: Boosting a post on X (formerly Twitter) might cost $100 to reach 24k–57k impressions, but those impressions often lack the endorsement of a trusted peer. Asami creates impressions through trusted voices, which tend to convert better and build long-term brand value.

- **Use it alongside existing strategies**: Asami isn’t here to replace your current marketing playbook—it’s a powerful addition. You can combine traditional ad campaigns with Asami campaigns to get layered visibility and broader reach.

- **Get transparency and cost control**: You set the budget, reward range, and campaign duration. Funds are only used when Collaborators repost and are registered by the Campaign Manager. Unused funds are returned.

### 11.2 For Collaborators

Collaborators join Asami because it allows them to:

- **Monetize their social influence**: Even if you're not a content creator in the traditional sense, you may have real influence over your followers. Asami lets you earn for amplifying messages that resonate with your values.

- **Maintain your independence**: You decide which campaigns you want to support. You don’t have to compromise your voice or reputation—there’s no obligation to participate in any campaign you don’t believe in.

- **Get paid globally, in crypto**: Rewards are paid in DOC, a stablecoin pegged to the US dollar. Payouts go straight to your wallet and are usable from anywhere in the world—no intermediaries, no bank delays.

- **Grow your profile**: Asami rewards not only big influencers, but also genuine micro-influencers. If your audience engages with you authentically, you’ll be recognized and paid accordingly.

- **Participate in a meaningful system**: Every time you collaborate on a campaign, you earn ASAMI tokens. These tokens give you a share of the protocol’s revenue and a voice in how it evolves. You become more than a user—you become a stakeholder.

- **Onboard easily**: The platform supports gasless claims and Web2 onboarding, so even if you’ve never used crypto before, you can join Asami and receive rewards smoothly.

Asami is built for people—people with something to say, people with something to promote, and people who care about how messages are spread. Whether you're here to advertise or collaborate, Asami offers a better way to connect and grow.

---

## 12. Contract Addresses and Links

- **Asami Smart Contract** (Rootstock):  
  https://explorer.rootstock.io/address/0x3150e390bc96f1b4a05cf5183b8e7bdb68566954

- **Current Campaign Manager Address**:  
  https://explorer.rootstock.io/address/0x3e79325b61d941e7996f0a1aad4f66a703e24faa

- **DOC Stablecoin Info (Money on Chain)**:  
  https://moneyonchain.com/

- **Open Source Campaign Manager Software**:  
  https://github.com/constata-eu/asami

## 13. Contact and Support
Members are encouraged to ask questions or request help via public channels. We aim to keep all communication transparent and community-facing.

**X (Twitter):**
- English: [@asami_club](https://twitter.com/asami_club)
- Spanish: [@asami_club_es](https://twitter.com/asami_club_es)

**Telegram:**
- English group: `@asami_club`
- Spanish group: `@asami_club_es`

Please note that all reviews and scoring requests must follow the evidence-based process described in this whitepaper, which also contains an FAQ appendix.

### Appendix A: FAQ and Known Issues

**Q: Why am I not getting any campaigns?**
- Your influence score may be low, or you may be missing required categories. You can request a review.

**Q: Why does my score seem too low?**
- Read the Influence Measurement section and follow the dispute resolution process outlined in Section 3.2.

**Q: I used to get paid more than I do now. Why?**
- This may be due to lower advertiser budgets or changes in your score/category. It could also be due to reduced platform activity or increased competition.

**Q: I was banned or excluded by the Campaign Manager. Can I still participate?**
- Yes. Asami is permissionless. You can work with another campaign manager or use your ASAMI tokens normally.

**Known Issue:** Collaborations may be registered after a campaign runs out of funds. The platform will show them as "failed", but may compensate later if funds are added.
