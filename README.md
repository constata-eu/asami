# ASAMI
Autonomous Social Ads Marketplace Initiative

Learn more about the project at https://asami.club

This is a mega-repo with all the project's code and documentation, pull requests and all sorts of collaborators are welcome.

You can contact us by creating an issue here.

###


- Must we keep the "HandleRequest" tables?
  - Yes, HandleRequest becomes internal use only. We can rename it to "Handles", we check here for collabs.
  - They don't have a price anymore.
  - They do have a score.

- Must we keep the "CollabRequest" tables to avoid registering collabs again (?)
  - No, we're not migrating campaigns so there's no risk of wrong collabs.

AsamiCore migration path:

- Only tokens are maintained.
- All previous tables are renamed to "old_" and have no more models. Campaigns table is renamed to "OldCampaigns"

- Full sync from on-chain data is no longer possible.
  - Handles, Accounts, Campaigns, Collabs tables can be renamed to "v1_**"

- "Request" tables are the new -only- datasource, with full tracking of the request state. 

- OnChainTxs are added to a queue and run sequentially.
  - Each OnChainTx-able model habtm of its params models.
  - OnChainTxId column always points to the most recent one.
  - One "request" could have participated in more than one TX, but only in one succesfull one.

- Campaigns are created in the DB, with all params and a unique id only known to the creator.
  - Campaigns are always scoped by member.
  - Brief hashes cannot be repeated by members.
  - Campaign ID in local DB is hash of member address + campaign brief.
  - Syncing looks up events, finds campaigns in "pending" state for the creator

  - API limits campaign creation, nobody can create "valid" campaigns for someone else.
  - If a campaign creation event is received without a valid brief, it's discarded.
  - Campaigns can be extended now.

  - This unique ID could have the brief ID to link both data points.
    - action: { "site": "X", "action": "repost", "contentId": "323232323" }
    - DB id is hash of creator + hash of action

    

  - The campaign is created on-chain referencing the off-chain id.
  - The off-chain data source sees the on-chain payment and continues the campaign flow.

- On-Chain-Txs now store the gas spent. 
- CampaignRequests 
