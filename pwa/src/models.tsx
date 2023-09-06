import _ from 'lodash';

export class Campaign {
  terms: ClassicTerms | NostrTerms;
  startDate: number;
  isClassic: boolean;
  advertiser: string;
  funding: BigInt;

  constructor(private raw: any, public id: number, public socialNetworks: string[]) {
    this.startDate = raw.startDate;
    this.isClassic = raw.classicTerms.offers.length > 0;
    this.advertiser = raw.advertiser;
    this.funding = raw.funding;
    this.terms = this.isClassic ? (new ClassicTerms(this, raw.classicTerms)) : (new NostrTerms(this, raw.nostrTerms));
  }

  getOffersForCreator(address: string): Array<ClassicOffer | NostrOffer> {
    return this.terms.offers.filter((o) => o.rewardAddress === address )
  }
}

class ClassicTerms {
  socialNetworkId: number;
  socialNetwork: string;
  rulesUrl: string;
  rulesHash: string;
  oracleAddress: string;
  oracleFee: BigInt;
  offers: ClassicOffer[];
  instructions: string;

  constructor(public campaign: Campaign, private raw: any) {
    this.socialNetworkId = raw.socialNetworkId;
    this.socialNetwork = campaign.socialNetworks[this.socialNetworkId];
    this.instructions = `Post a message according to the instructions in this link, and keep it posted for 14 days. ${raw.rulesUrl}`;
    this.offers = raw.offers.map((x, i) => new ClassicOffer(this, x, i) );
  }
}

class ClassicOffer {
  rewardAddress: string;
  rewardAmount: BigInt;
  
  constructor(public terms: ClassicTerms, private raw: any, public id: number) {
    this.rewardAddress = raw.rewardAddress;
    this.rewardAmount = raw.rewardAmount;
  }

  state(): string {
    return ClassicOfferState[this.raw.state]
  }
}

enum ClassicOfferState {
  // The offer is assumed to be published.
  Assumed,

  // The advertiser has challenged the message publication.
  Challenged, 

  // The collaborator (or someone else) has paid the oracle to confirm publication.
  ConfirmationRequested,

  // The oracle has confirmed the message has been published.
  Confirmed,

  // The oracle could not confirm the message publication. The offer will be refunded.
  Refuted,

  // The collaborator has renounced 
  Renounced,

  // The advertiser has reported the publication has been deleted before the campaign ended.
  // The oracle will either confirm the deletion or will move the offer back to the confirmed state.
  DeletionReported,

  // The oracle has confirmed the publication was deleted early. The reward will not be paid. The collaborator will be penalized.
  DeletionConfirmed
}

class NostrTerms {
  requestedContent: string;
  offers: NostrOffer[];
  socialNetwork: string;
  instructions: string;

  constructor(public campaign: Campaign, private raw: any, public id: number) {
    this.socialNetwork = "Nostr"; 
    this.instructions = `Make a post in nostr with this exact message (without the quotes) and keep it up for at least 14 days: "${raw.requestedContent}"`;
    this.offers = raw.offers.map((x, i) => new NostrOffer(this, x, i) );
  }
}

class NostrOffer {
  rewardAddress: string;
  rewardAmount: BigInt;
  
  constructor(public terms: NostrTerms, private raw: any, public id: number) {
    this.rewardAddress = raw.rewardAddress;
    this.rewardAmount = raw.rewardAmount;
  }

  state(): string {
    return NostrOfferState[this.raw.state]
  }
}

enum NostrOfferState {
  // The offer is assumed to be published.
  Assumed,

  // The advertiser has challenged the message publication.
  Challenged, 

  // The collaborator has submitted proof of the message.
  Confirmed,

  // The collaborator has renounced the reward, probably intending to delete the message.
  Renounced,

  // The advertiser has reported the publication has been deleted before the campaign ended.
  ReportedDeletion
}

