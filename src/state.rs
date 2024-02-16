use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub creator: Addr,
    pub fee_address: Addr,
    pub native_token: String,
    pub tx_fee: Uint128,
}

#[cw_serde]
pub struct AirdropInfo {
    pub cw20_address: Addr,
    pub total_airdrop: Uint128,
    pub airdropable: Uint128,
}

#[cw_serde]
pub struct Collection {
    pub address: Addr,
    pub owner: Addr,
    pub total_airdrop: Uint128,
    pub airdropable: Uint128,
    pub airdrop_infos: Vec<AirdropInfo>,
    pub duration: u64,
    pub started_at: u64,
    pub fee_address: Addr,
    pub cw20_address: Addr,
    pub is_started: bool,
    pub is_enabled: bool,
    pub is_show: bool,
    pub unstaking_fee: Uint128,
    pub unstaking_fee_percent: u64,
    pub users: Vec<UserInfo>
}

#[cw_serde]
pub struct  NftInfo {
    pub nft_id: String,
    pub lock_time: u64,
    pub airdrop: bool,
    pub collection_address: Addr,
}

#[cw_serde]
pub struct EarnInfo {
    pub cw20_address: Addr,
    pub total_earned: Uint128,
    pub claimable: Uint128,
}

#[cw_serde]
pub struct UserInfo {
    pub address: Addr,
    pub total_earnd: Uint128,
    pub claimable: Uint128,
    pub earn_infos: Vec<EarnInfo>,
    pub staked_nfts: Vec<NftInfo>,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const COLLECTION_MAP_PREFIX: &str = "collection_map";
pub const COLLECTION_MAP: Map<Addr, Collection> = Map::new(COLLECTION_MAP_PREFIX);