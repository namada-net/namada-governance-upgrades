# Namada Mainnet Governance Proposals

Namada mainnet launch is divided into [5 phases](https://namada.net/mainnet-launch). Going though each phase require a governance proposal to active some features. This repository contains the WASM to associate with each governance proposal.

> 🔧 The parameters values are still being decided. 🔧

## Block party -> Staking party 

Staking rewards are enabled for delegators and validators staking NAM. Public Goods Funding is enabled to support public goods in the Namada ecosystem and beyond. Phase 3 will begin when governance votes to enable transfers and shielding of IBC assets.

The reference code can be found in [block_party folder](./block_party/).

## Staking party -> Shielding party

Transparent and shielded transfers of governance-enabled IBC assets are live. Users can begin shielding assets in the unified shielded set. NAM transfers remain locked until phase 5. Phase 4 will begin when governance votes to enable shielding rewards.

The reference code can be found in [staking_party folder](./staking_party/).

## Shielding party -> Shielding Reward party

Shielding rewards for governance-enabled IBC assets are live. Users begin collecting rewards for shielding assets, which protects their data and helps strengthen Namada’s unified shielded set. Phase 5 will begin when governance votes to enable NAM transfers.

The reference code can be found in [shielding_party folder](./shielding_party/).

## Shielding Reward party -> NAM Party

When the Namada community is confident that the network is stable, NAM transfers are enabled. All key protocol functionality is now live. From here on, new features and support for new assets can continue to be added by the community via on-chain governance.

The reference code can be found in [shielding_reward_party folder](./shielding_reward_party/).