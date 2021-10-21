# Appchain Function Spec

- Create, Alter Authority, Update, Execute On Behalf of DAO : Pallet Deip\_DAO
    - create(name, authority, metadata)
    - alter\_authority(alter\_authority)
    - update\_dao(new\_metadata)
    - on\_behalf(dao\_id, call)

- Create, Decide on Proposal: Pallet Deip\_Proposal
    - propose(batch, external\_id) 
    - decide(proposal\_id, decision)

- Create, Destroy, Issue, Burn, Transfer, Freeze, Thaw, Transfer Ownership, Set Team, Set Metadata of Assets: Pallet Deip\_Assets
    - create\_asset(asset\_id, admin, min\_balance, project\_id)
    - destroy(id)
    - issue\_asset(id, beneficiary, amount)
    - burn(id, who, amount)
    - transfer(id, target, amount)
    - freeze(id, who)
    - thaw(id, who)
    - freeze\_asset(id)
    - thaw\_asset(id)
    - transfer\_ownership(id, owner)
    - set\_team(id, issuer, admin, freezer)
    - set\_metadata(id, name, symbol, decimals)

- Create, Update Project; Create Project Content; Create Project NDA, Create, Fulfill, Reject NDA Content Access; Create, Upvote Review, Add Domain, Create, Accept, Reject Contract Agreement; Create Investment Opportunity, Invest: Pallet Deip
    - create\_project(is\_private, external\_id, team\_id, description, domains)
    - update\_project(project\_id, description, is\_private)
    - create\_project\_content(external\_id, project\_external\_id, team\_id, content\_type, description, content, authors, references)
    - create\_project\_nda(external\_id, end\_date, contract\_hash, maybe\_start\_date, parties, projects)
    - create\_nda\_content\_access\_request(external\_id, nda\_external\_id, encrypted\_payload\_hash, encrypted\_payload\_iv)
    - fulfill\_nda\_content\_access\_request(external\_id, encrypted\_payload\_encryption\_key, proof)
    - reject\_nda\_content\_access\_request(external\_id)
    - create\_review(external\_id, author, content, domains, assessment\_model, weight, project\_content\_external\_id)
    - upvote\_review(review\_id, domain\_id)
    - add\_domain(domain)
    - create\_contract\_agreement(id, creator, parties, hash, activation\_time, expiration\_time, terms)
    - accept\_contract\_agreement(id, party)
    - reject\_contract\_agreement(id, party)
    - create\_investment\_opportunity(external\_id, creator, shares, funding\_model)
    - invest(investment\_id, asset\_id)

