# Smart contract escrow for collaboration between Advertisers and Content Creators.

This smart contract regulates the relationship between a company willing to run an ad, the "Advertiser", an and one or several indpendent creators with a significant following, the "creators".

The Advertiser will create a campaign, offering DoC rewards to a specific set of preselected creators. The DoC for the rewards will be locked in the contract upfront.

The creators will publish their messages through their nostr accounts with the exact requested content, and will get paid automatically after they've run the message for 14 days.

The creators must publish the message before a certain date, for the Advertiser to see. If the Advertiser cannot see the published message they can challenge the creator to submit proof. Challenges can only be submitted in a 24 hour window after the deadline is reached.

In case of a challenge, the creator has 24 hours to submit proof of publication. If they fail to submit proof of publication, their payment is aborted (and they're obviously free to delete the message if they've actually published it).

The creator must refrain from deleting the message for 14 days. If an early deletion is detected, the Advertiser may submit proof of deletion which will abort the payment and may set a bad reputation for the creator.

If the creator must delete the message earlier than expected, they can voluntarily renounce their payment to avoid a reputational penalty.

The creator can renounce payment at any time.

For convenience, the Advertiser can undo its challenge and always pay the creator. This is only for convenience, since the Advertiser can always unilateraly send money to the creator reward address outside the contract scope.

Submitting proofs to the contract is costly (~4 usd), so rewards below that amount make conflict resolution impractical.

Any leftover funds from unpaid rewarsd are returned to the Advertiser's address.

Methods for issuing payouts can be called by any RSK account. 
