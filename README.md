# ASAMI
Autonomous Social Ads Marketplace Initiative
> It's also a japanese girl name meaninig 'future'.

Self Sovereign Social Networks – like nostr and the upcoming bluesky – give freedom and power back to the community.

They almost erase the role of the "social media platform".

You can choose your own client software, your servers, read exactly who you want to read – and never see an ad.

But ads are still a content creator's main revenue stream, and may remain so in the future.

Companies and projects will always be interested in leveraging influencer's attention.

And the general audience that pays for content with their attention would like less intrusive ads.

Moreso, if ads were truly relevant and chosen by content creators, they may even find them useful.

TV ads are annoying, but sport team fans wear their team's shirt and sponsorships with pride.

<video src='https://github.com/constata-eu/bitcoin-scaling-hackathon/raw/main/nostr_ads_powered_by_rsk/rsk-nostr.mp4' width="500"></video>

We think we can replace the "social media platform" and their ads program with self sovereign tools.

## An escrow service between brands and content creators.

The social media platform charges brands to serve their ads, and *may* pay content creators. 

We are replacing that with a smart contract in RSK.

A unique feature of SSSNetworks like NOSTR is that you can prove you have published a message to a smart contract.

RSK is relatively cheap for this use case, and features the most resilient and decentralized stablecoin: Dollar on Chain.

> Status:
> * On track to staging before may.
> * Verifying a nostr message costs 4 USD in fees which renders smaller campaigns useless. We're exploring optimizing the cryptographic verification routines or making proof verification optional.

## A marketplace of brands and content creators.

A social media platform gets their cut from charging brands and paying content creators. 

They never meet, this affects the relevance and overall effectiveness of the ads.

When content creators do meet brands, the collaborations are usually genuine and better received.

We want more of that, reducing the friction and helping them find each other.

We're reaching out to brands and content creators and influencers to run a pilot program in nostr.

We're building a chat bot were a brand representative can set up a new campaign and select relevant content creators as part of a conversation.

The chatbot will also update the brand with some stats on the campaign.

It will also onboard content creators into the program, offer them collaboration opportunities and help them submit their proof and receive payment.

> Status:
> * We only have RSK as the interested brand yet. Some interest from influencers. Pilot program has not started yet.
> * Chatbot architecture is defined and feasibility is confirmed, but implementation has not started yet. Will probably wait until the smart contract is done.

## A SSSNetwork analizer for reach assesment and campaign monitoring.

The social media platform knows exactly how many impressions an ad gets, and how many subscribers a content creator has.

Both brands and content creators trust this data to be correct.

In the SSSNetwork model impressions cannot be reliably counted.

Subscriber counts may be inflated by bots, same as with the traditional model.

To make campaigns fair, we need to know which users are bots and which content creators have the most reach.

So we're indexing the entirety of the nostr network to perform social graph analysis.

> Status:
> * We have the indexer running and indexed 45gb of nostr messages from public relays, thats over a third of the network.
> * We need to start including paid relays as well.
> * We have designed some bot filtering and reach assesment algorithms, and will start implementing them soon.

