### Discovery Phase
1. **Awareness**:
  Before even exploring deals or prices, participants often first need to become aware of the marketplace itself. This may involve various marketing tactics like SEO, social media advertising, or word-of-mouth.
  Addressed with: Asami Landing. Ads. Events. Seminars.
2. **Exploration**: Browse market depth, read reviews, compare prices of offerings.
  Addressed with: Asami Landing, showcasing both brands and influencer reviews, and metrics.

### Decision-making Phase
3. **Evaluation**: Participants filter and shortlist options, read reviews, see quotes an prices, and previous campaigns.
4. **Counterparty Selection**: This is the point where a participant makes a decision to reach out to bidding/asking counterparties.

### Transaction Phase
5. **Negotiation**: Before the transaction is confirmed, there might be some back-and-forth on the terms, price, or specifics of the service.
6. **Payment Escrow**: The customer funds an escrow account, providing security for both parties.

### Fulfillment Phase
7. **Service Delivery**: The service provider performs the service. This could be a single event or a series of milestones.
8. **Monitoring and Quality Check**: Some marketplaces have a built-in quality assurance step or progress tracking.
9. **Dispute Resolution**: If there are any issues, this is the stage where disputes are settled.

### Post-Transaction Phase
10. **Release of Funds**: Assuming there are no disputes, the escrowed funds are released to the service provider.
11. **Review and Ratings**: Both parties may be asked to rate each other, which becomes part of their profile for future transactions.
12. **Follow-up/Retention**: Marketplaces often engage with customers post-transaction for feedback, upselling, or offering related services.

### Analytic Phase
13. **Data Collection and Analysis**: Although not directly involving the customer, marketplaces typically analyze transactions for market research, user experience improvements, or fraud detection.

### Re-Engagement Phase
14. **Retargeting**: Former participants are re-targeted via various channels to encourage them to return to the marketplace for future needs.


# Ways in which we add value we may be able to monetize:

## Reduce friction for brand massive hiring.
  Friction points:
    - Briefing: Overcome with verbatim campaigns + contextual message from them.
    - Pricing. 
    - Campaign type:
      - How long?
      - What network?
      - What format?
    - Influencer selection / targetting.

## Make price discovery easy for growing the ecosystem.
  - Nice to have: Serves when comparing the other types of influencer marketing.

## Make exchanges safe and trustworthy.
  We use RSK blockchain to track funding, campaigns, disputes, reputation and payouts.
  We use RSK blockchain tech to make raffles transparent.

# Objections and choices:

## End user experience: Genuine influencer vs pushing a message.
  Genuine message may get better conversions but has more friction.
  Verbatim ads may retain some genuinity advantage.
  Program includes tokens + raffle so it still comes across.

-----------
# Workflow for advertiser.
  1) Brief.
  Asami is a broadcast engine, sending your message to as many people as possible.
  You can currently reach, at most,500 to 1000K people for 9000 USD.
  Tell us your campaign brief and budget, and we'll give you recommedations.
  [Campaign topics]
  [Medium (asset) ]
  [Budget] - We're just getting started, so this one is on us.

  2) By your brief, we can recommend you:
    - [ ] Spend 200USD to reach 600 - 2000 people talking about Bitcoin.
            ( See who's going to be in this campaign )
    - [ ] Spend 9000USD to reach 200-300 people in general.
            ( See who's going to be in this campaign )

  3) You OK with this?
    Then give us your campaign brief
    What text should they post?
  
  4) And you're done!

  5) In Back office: We fund the campaign and send it to all the influencers.


# Onboarding for influencer // from scratch.
  1) Brief
    Asami is a broadcast engine, where you get rewarded for sharing messages from brands, NGO's, or even other individuals.
    You can always actively opt-out, and get rewarded even so.
    Rewards are asami tokens, which you can convert directly to Bitcoin, Dollar on Chain, or use them to participate in our monthly raffles for cash prices.
    If you're interested, we'll sign you up we just need to know which kind of sharing you want to do.
    [ ] Reposting X posts.
    [ ] Reposting Nostr Posts.
    [ ] Sharing instagram posts in your stories.

  2) Your networks
    Great then, we need to know.
    Your instagram handle [ ]
    How much would you charge per instagram video? [ ] (we suggest something like 10 USD)
    Your X handle [ ]
    Your Nostr handle [ ]
  
  3) Awesome. We'll add you on [twitter, instagram, nostr] and contact you when we have something.
    You can always come back here to change these settings, or reach out to us via private message.

# Onboarding for .
  1) We find someone retweeted a campaign, and is not registered.
    - We try to contact the influencer and tell them their RT will actually get paid.

# Campaign flow? guarantees? Smart contract?
- When advertiser pays, asami creates a campaign in the smart contract with the money.
  Advertiser gets transparency about their campaign and budget.

- As reposts are detected, they are added to the smart contract campaign.
  The advocates topics must match the campaign.
  Advertiser continues to have transparency about the budget.
  Advocates can see if their post was registered.
  The oracle has a "whitelist" so not all posts register.
  Rewards are for the first XYZ participants.
    - Other participants get tokens.
  
  - For nostr, if the oracle fails, users can push their message to get paid.
  
- If a repost is detected as deleted, the oracle marks it so.
  Advertiser and advocates see this. 

- When a repost has been live enough, the advocate gets paid.
  - They get paid in asami ERC20 tokens.

  - The asami ERC20 token 

# Workflows

- Advertiser Landing
  - Current open campaign proposals with markers if they were answered.
    - Rejections may offer replacement? (for now they're just ignored).

  - Influencer shopping cart right after login.
    - Can filter and select several influencers (fav / shopping cart) then goes to "check-out".
    - Check out includes writing down a brief / proposal for them.
      - If anyone rejects the invitation the brand can replace them.
      - They have up until the day the campaign starts to reply, otherwise they miss out.

- Influencer landing

# Modeling

- Advertiser (Profile)

- Collaborator (Profile)
  - The Collaborator has a profile for reputation and history only.
  - They may place Listings (Asks) about work they're willing to do.

- Asks:

- Assets: 
  Assets describe what the collaborator has to offer and what the advertiser wants to receive.
  Payment is always in crypto so we get that out of the way.
  
  
  - One instagram 15 second video.
  - One verbatim X post.
  - One verbatim nostr ublication.

- Advertiser Listing (Bid, Campaign?):
  - Made by an advertiser willing to start a campaign.
  - Assets demanded.
  - Includes a brief to add on the assets.
  - May have a limit on people who can apply (stock?) (this helps manage exhaustion).

- Campaign:
  A campaign groups multiple Advertiser Listings, it may have a "global" brief, but each listing is for a specific asset.

- Collaborator listing (Ask):
  - Price.
  - Asset offered.
  - Brief, or context, to qualify assets. 

- CollaboratorMatch (influencer picks up an advertiser listing as is).
  - A collaborator shows interest in an offer as it is.
  - If collaborator does not find terms they like, they can always make a listing (and we make sure the brands sees them).
  - The advertiser may still reject the collaboration, or pay the escrow to start the campaign.
  - This spans a Collaboration.

- AdvertiserMatch (reverse collaborator match)

- Collaboration (payment/delivery)
  - (The smart contract "campaign" is a set of collaborations).
  - Tracks a collaborator announcement and the campaign internal workflow, publication etc.
  - Gets comments from both parties.

- Orgs:

- Users:
  belongs to org.
  has a role in the organization.

- AuthMethods:
  belongs to user.
  - Metamask public key.
  - RIF Login.
  - Google/facebook/twitter/instagram (and constata bridges them)

- Sessions:
  belong to some auth methods.

- Constata can impersonate people on chain when they don't have their own keys.
  - Should the smart contract include a way to change key ownership and still link to the same reputation?
    - Or is the asami smart contract just about being an escrow in a collaboration?


### ToDo:
- API requests have no business logic validations.
- Converting u256 to database values sucks.
- Idempotency in the SC (do not fail if a resubmission is attempted. Update DB status first?).
- Twitter api usage quotas are obnoxious:
  # retweeted by: 5 requests / 15 mins
  # account confirmations: 5 requests / 15 mins
