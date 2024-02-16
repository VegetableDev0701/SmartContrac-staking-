use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr};

use crate::state::AirdropInfo;
use crate::state::EarnInfo;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub fee_address: Addr,
    pub native_token: String,
    pub tx_fee: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateOwner {
        owner: Addr,
    },
    UpdateFeeAddress {
        fee_address: Addr,
    },
    UpdateTxFee {
        tx_fee: Uint128,
    },
    RegisteCollection {
        collection_address: Addr,
        owner: Addr,
        new_unstaking_fee: Uint128,
        new_unstaking_fee_percent: u64,
        cw20_address: Addr,
    },
    UpdateCollectionFee {
        collection_address: Addr,
        new_unstaking_fee: Uint128,
        new_unstaking_fee_percent: u64,
    },
    UpdateCollectionState {
        collection_address: Addr,
        is_show: bool,
    },
    UpdateCollection {
        collection_address: Addr,
        new_cw20_address: Addr,
        new_owner: Addr,
        new_duration: u64,
        new_fee_address: Addr,
        is_enabled: bool
    },
    Charge {
        collection_address: Addr,
    },
    ChargeToken {
        collection_address: Addr,
        charge_amount: Uint128
    },
    Withdraw {
        amount: Uint128
    },
    WithdrawAirdrop {
        collection_address: Addr,
        cw20_address: Addr,
        amount: Uint128
    },
    Airdrop { 
        collection_address: Addr,
        cw20_address: Addr,
        airdrop_amount: Uint128 
    },
    AirdropRestart {
        collection_address: Addr,
    },
    // ReceiveNft (Cw721ReceiveMsg),
    Restake { 
        collection_address: Addr,
        token_id: Vec<String>,
    },
    Unstake { 
        collection_address: Addr,
        token_id: Vec<String>,
    },
    Staking {
        collection_address: Addr,
        token_id: Vec<String>
    },
    Claim { 
        collection_address: Addr,
        cw20_address: Addr,
    },
    TransferNft {
        collection_address: Addr,
        nft_id: String,
        recipient: String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {
    },

    #[returns(CollectionResponse)]
    GetCollection {
        collection_address: Addr,
    },

    #[returns(UserListResponse)]
    GetUserList {
        collection_address: Addr,
    },

    #[returns(CollectionListResponse)]
    GetCollectionList {
    },

    #[returns(CollectionListResponse)]
    GetShownCollectionList {
    },

    #[returns(TotalLockedResponse)]
    GetTotalLocked {
        collection_address: Addr,
    },

    #[returns(StakedNftsResponse)]
    GetStakedNfts {
        address: Addr,
        collection_address: Addr,
    },

    #[returns(EarnInfosResponse)]
    GetEarnInfos {
        address: Addr,
        collection_address: Addr
    },

    #[returns(AirdropInfosResponse)]
    GetAirdropInfos {
        address: Addr,
    },

}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub fee_address: Addr,
    pub tx_fee: Uint128,
}

#[cw_serde]
pub struct CollectionResponse {
    pub address: Addr,
    pub owner: Addr,
    pub total_airdrop: Uint128,
    pub airdropable: Uint128,
    pub duration: u64,
    pub started_at: u64,
    pub fee_address: Addr,
    pub cw20_address: Addr,
    pub is_started: bool,
    pub is_show: bool,
    pub is_enabled: bool,
    pub unstaking_fee: Uint128,
    pub unstaking_fee_percent: u64,
    pub server_time: u64,
}

#[cw_serde]
pub struct CollectionListResponse {
    pub collections: Vec<Addr>,
}

#[cw_serde]
pub struct UserListResponse {
    pub addresses: Vec<Addr>,
}

#[cw_serde]
pub struct TotalLockedResponse {
    pub count: Uint128,
}

#[cw_serde]
pub struct StakedNftResponse {
    pub nft_id: String,
    pub airdrop: bool,
    pub lock_time: u64
}

#[cw_serde]
pub struct StakedNftsResponse {
    pub nft_maps: Vec<StakedNftResponse>,
}

#[cw_serde]
pub struct EarnInfosResponse {
    pub total_earned: Uint128,
    pub claimable: Uint128,
    pub earn_infos: Vec<EarnInfo>
}

#[cw_serde]
pub struct AirdropInfosResponse {
    pub total_airdrop: Uint128,
    pub airdropable: Uint128,
    pub airdrop_infos: Vec<AirdropInfo>
}

#[cw_serde]
pub enum NftReceiveMsg {
    Stake {
        sender: String,
        token_id: String,
        collection_address: Addr,
    }
}