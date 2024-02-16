#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, from_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, CosmosMsg, WasmMsg, Empty};

use cw2::set_contract_version;
use cw20::Denom;
use cw721::{Cw721ReceiveMsg, Cw721ExecuteMsg};
use cw_utils::must_pay;

use crate::util;
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    InstantiateMsg, 
    QueryMsg, 
    StakedNftResponse, 
    StakedNftsResponse,
    ConfigResponse,
    CollectionResponse,
    NftReceiveMsg, 
    EarnInfosResponse,
    AirdropInfosResponse, 
    UserListResponse,
    CollectionListResponse,
    TotalLockedResponse,
};
use crate::state::{
    Config, 
    CONFIG,
    COLLECTION_MAP,
    NftInfo, 
    UserInfo, AirdropInfo, EarnInfo
};

const CONTRACT_NAME: &str = "Injstaking by AOI";
const CONTRACT_VERSION: &str = "1.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: msg.owner.clone(),
        creator: msg.owner.clone(),
        fee_address: msg.fee_address.clone(),
        tx_fee: msg.tx_fee.clone(),
        native_token: msg.native_token.clone(),
    };

    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::default())
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner { 
            owner 
        } => util::execute_update_owner(
            deps.storage, 
            info.sender, 
            owner
        ),
        ExecuteMsg::UpdateFeeAddress { 
            fee_address
        } => util::execute_update_fee_address(
            deps.storage, 
            info.sender, 
            fee_address
        ),
        ExecuteMsg::UpdateTxFee { 
            tx_fee
        } => util::execute_update_tx_fee(
            deps.storage, 
            info.sender, 
            tx_fee
        ),
        ExecuteMsg::RegisteCollection { 
            collection_address, 
            owner, 
            new_unstaking_fee, 
            new_unstaking_fee_percent ,
            cw20_address,
        } => util::execute_registe_collection(
            deps.storage, 
            info.sender, 
            collection_address, 
            owner, 
            cw20_address, 
            new_unstaking_fee, 
            new_unstaking_fee_percent
        ),
        ExecuteMsg::UpdateCollectionFee { 
            collection_address,
            new_unstaking_fee,
            new_unstaking_fee_percent,
        } => util::execute_update_collection_fee(
            deps.storage, 
            info.sender, 
            collection_address,
            new_unstaking_fee,
            new_unstaking_fee_percent,
        ),
        ExecuteMsg::UpdateCollectionState { 
            collection_address,
            is_show,
        } => util::execute_update_collection_state(
            deps.storage, 
            info.sender, 
            collection_address,
            is_show,
        ),
        ExecuteMsg::UpdateCollection {
            collection_address,
            new_cw20_address,
            new_owner,
            new_duration,
            new_fee_address,
            is_enabled,
        } => util::execute_update_collection(
            deps.storage,
            info.sender,
            collection_address,
            new_cw20_address,
            new_owner,
            new_duration,
            new_fee_address,
            is_enabled,
        ),
        ExecuteMsg::Charge { 
            collection_address, 
        } => execute_charge(
            deps, 
            info, 
            collection_address,
        ),
        ExecuteMsg::ChargeToken { 
            collection_address, 
            charge_amount 
        } => execute_charge_token(
            deps, 
            env, 
            info, 
            collection_address,
            charge_amount,
        ),
        ExecuteMsg::Withdraw {
            amount,
        } => execute_withdraw(
            deps, 
            env, 
            info, 
            amount,
        ),
        ExecuteMsg::WithdrawAirdrop { 
            collection_address, 
            cw20_address,
            amount 
        } => execute_withdraw_airdrop(
            deps, 
            env, 
            info, 
            collection_address,
            cw20_address,
            amount,
        ),
        ExecuteMsg::Airdrop {
            collection_address,
            cw20_address,
            airdrop_amount,
        } => execute_airdrop(
            deps, 
            env, 
            info,
            collection_address,
            cw20_address,
            airdrop_amount,
        ),
        ExecuteMsg::AirdropRestart {
            collection_address
        } => execute_airdrop_restart(
            deps, 
            env, 
            info, 
            collection_address,
        ),
        /* ExecuteMsg::ReceiveNft (
            msg
        ) => execute_receive_nft(
            deps, 
            env, 
            info, 
            msg
        ), */
        ExecuteMsg::Unstake {
            collection_address,
            token_id
        } => execute_unstake(
            deps, 
            env, 
            info, 
            collection_address,
            token_id
        ),
        ExecuteMsg::Staking {
            collection_address,
            token_id
        } => execute_stake(
            deps, 
            env, 
            info, 
            collection_address,
            token_id
        ),
        ExecuteMsg::TransferNft {
            collection_address,
            nft_id,
            recipient
        } => execute_transfer_nft(
            deps,
            env,
            info,
            collection_address,
            nft_id,
            recipient,
        ),
        ExecuteMsg::Claim {
            collection_address,
            cw20_address,
        } => execute_claim(
            deps, 
            env, 
            info, 
            collection_address,
            cw20_address,
        ),
        ExecuteMsg::Restake {
            collection_address,
            token_id
        } => execute_restake(
            deps, 
            env, 
            info, 
            collection_address,
            token_id
        ),
    }
}

pub fn execute_charge (
    deps: DepsMut,
    info: MessageInfo,
    collection_address: Addr,
) -> Result<Response, ContractError> { 
    let cfg = CONFIG.load(deps.storage)?;
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;

    let receive_amount = match must_pay(&info, &cfg.native_token) {
        Ok(it) => it,
        Err(_err) => return Err(ContractError::InsufficientCw20 {  }),
    }.u128();

    collection.airdropable += Uint128::from(receive_amount);
    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;

    return Ok(Response::new()
        .add_attribute("collection_address", collection_address)
        .add_attribute("action", "execute_charge")
        .add_attribute("charge", receive_amount.to_string())
    );
}

pub fn execute_charge_token (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    charge_amount: Uint128
) -> Result<Response, ContractError> { 
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;

    let msg;
    if collection.cw20_address.to_string().len() == 0 {
        return Err(crate::ContractError::InvalidCw20Token {  });
    } else {
        if util::get_token_amount(deps.querier, Denom::Cw20(collection.cw20_address.clone()), info.sender.clone())? < charge_amount {
            return Err(crate::ContractError::InsufficientCw20 {  });
        }

        msg = util::transfer_from_token_message(
            Denom::Cw20(collection.cw20_address.clone()), 
            info.sender,
            env.clone().contract.address.clone(), 
            charge_amount
        )?;

        let index = collection.airdrop_infos.iter().position(|airdrop_info| airdrop_info.cw20_address == collection.cw20_address);
        match index {
            Some(index) => {
                collection.airdrop_infos[index].airdropable += charge_amount;
            },
            None => {
                let airdrop_info = AirdropInfo {
                    cw20_address: collection.cw20_address.clone(),
                    total_airdrop: Uint128::zero(),
                    airdropable: charge_amount,
                };
                collection.airdrop_infos.push(airdrop_info.clone())
            }
        }
        COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
    }

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("collection_address", collection_address)
        .add_attribute("action", "execute_charge")
        .add_attribute("charge", charge_amount.clone())
    )
}

pub fn execute_withdraw (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128
) -> Result<Response, ContractError> { 
    util::check_owner(deps.storage, info.sender.clone())?;

    let cfg = CONFIG.load(deps.storage)?;

    if util::get_token_amount(deps.querier, Denom::Native(cfg.native_token.clone()), env.clone().contract.address.clone())? < amount {
        return Err(crate::ContractError::InsufficientCw20 {  });
    }

    let msg = util::transfer_token_message(Denom::Native(cfg.native_token.clone()), amount.clone(), info.sender.clone())?;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "execute_withdraw")
        .add_attribute("withdraw", amount.clone())
    )
}

pub fn execute_withdraw_airdrop (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    cw20_address: Addr,
    amount: Uint128
) -> Result<Response, ContractError> { 
    util::check_collection_owner(deps.storage, collection_address.clone(), info.sender.clone())?;

    let cfg = CONFIG.load(deps.storage)?;
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;

    if cw20_address.to_string().len() == 0 {
        if amount > collection.airdropable {
            return Err(crate::ContractError::InsufficientCw20 {  });
        }

        if util::get_token_amount(deps.querier, Denom::Native(cfg.native_token.clone()), env.clone().contract.address.clone())? < amount {
            return Err(crate::ContractError::InsufficientCw20 {  });
        }

        let msg = util::transfer_token_message(Denom::Native(cfg.native_token.clone()), amount.clone(), info.sender.clone())?;

        collection.airdropable -= amount;
        COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
    
        Ok(Response::new()
            .add_message(msg)
            .add_attribute("collection_address", collection_address)
            .add_attribute("action", "execute_withdraw")
            .add_attribute("withdraw", amount.clone())
        )
    } else {
        let index = collection.airdrop_infos.iter().position(|airdrop_info| airdrop_info.cw20_address == cw20_address);
        match index {
            Some(index) => {
                if collection.airdrop_infos[index].airdropable < amount {
                    return Err(crate::ContractError::InsufficientCw20 {  });
                }

                if util::get_token_amount(deps.querier, Denom::Cw20(cw20_address.clone()), env.clone().contract.address.clone())? < amount {
                    return Err(crate::ContractError::InsufficientCw20 {  });
                }
        
                let msg = util::transfer_token_message(Denom::Cw20(cw20_address.clone()), amount.clone(), info.sender.clone())?;

                collection.airdrop_infos[index].airdropable -= amount;

                COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
            
                Ok(Response::new()
                    .add_message(msg)
                    .add_attribute("collection_address", collection_address)
                    .add_attribute("action", "execute_withdraw")
                    .add_attribute("withdraw", amount.clone())
                )
            },
            None => {
                return Err(crate::ContractError::InsufficientCw20 {  });
            }
        }
    }
}

pub fn execute_airdrop(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    cw20_address: Addr,
    airdrop_amount: Uint128,
) -> Result<Response, ContractError> { 
    util::check_enabled(deps.storage, collection_address.clone())?;
    util::check_collection_owner(deps.storage, collection_address.clone(), info.sender.clone())?;

    if airdrop_amount <= Uint128::zero() {
        return Err(crate::ContractError::InvalidAirdrop {  });
    }
    
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;

    if cw20_address.to_string().len() == 0 {
        if airdrop_amount > collection.airdropable {
            return Err(crate::ContractError::InsufficientCw20 {  });
        }
    } else {
        let index = collection.airdrop_infos.iter().position(|airdrop_info| airdrop_info.cw20_address == cw20_address);
        match index {
            Some(index) => {
                if collection.airdrop_infos[index].airdropable < airdrop_amount {
                    return Err(crate::ContractError::InsufficientCw20 {  });
                }
            },
            None => {
                return Err(crate::ContractError::InsufficientCw20 {  });
            }
        }
    }

    let nft_count = util::get_in_locktime_nft_count(deps.storage, env.block.clone(), collection_address.clone())?;

    if nft_count.is_zero() {
        return Err(crate::ContractError::NoUnexpiredNft {  });
    }

    if airdrop_amount < nft_count {
        return Err(crate::ContractError::OverNftCount { 
            nft_count 
        });
    }

    let airdrop = Uint128::from(airdrop_amount/nft_count);

    let users = collection.users.clone();
    let mut new_users = vec![];
    for _userinfo in users.iter() {
        let mut userinfo = _userinfo.clone();
        let mut nftcount = Uint128::zero();
        for (index, _nftinfo) in userinfo.staked_nfts.clone().iter().enumerate() {
            let mut nftinfo = _nftinfo.clone();
            if nftinfo.lock_time > env.block.time.seconds() && nftinfo.collection_address == collection_address {
                nftinfo.airdrop = true;
                nftcount += Uint128::from(1u128);
            }
            userinfo.staked_nfts[index] = nftinfo;
        };

        if cw20_address.to_string().len() == 0 {
            userinfo.claimable += airdrop * nftcount;
        } else {
            let index = userinfo.earn_infos.iter().position(|earn_info| earn_info.cw20_address == cw20_address);
            match index {
                Some(index) => {
                    userinfo.earn_infos[index].claimable += airdrop * nftcount;
                },
                None => {
                    let earn_info = EarnInfo {
                        cw20_address: cw20_address.clone(),
                        total_earned: Uint128::zero(),
                        claimable: airdrop * nftcount,
                    };
                    userinfo.earn_infos.push(earn_info);
                }
            }
        }
        new_users.push(userinfo);
    }
    collection.users = new_users;

    if cw20_address.to_string().len() == 0 {
        collection.total_airdrop += airdrop_amount.clone();
        collection.airdropable -= airdrop_amount.clone();
    } else {
        let index = collection.airdrop_infos.iter().position(|airdrop_info| airdrop_info.cw20_address == cw20_address);
        match index {
            Some(index) => {
                collection.airdrop_infos[index].total_airdrop += airdrop_amount.clone();
                collection.airdrop_infos[index].airdropable -= airdrop_amount.clone();
            },
            None => {
                return Err(crate::ContractError::InsufficientCw20 {  });
            }
        }
    }
    collection.is_started = false;
    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
    
    Ok(Response::new()
        .add_attribute("action", "execute_airdrop")
        .add_attribute("collection_address", collection_address.clone())
        .add_attribute("airdrop", airdrop_amount.clone())
    )
}

pub fn execute_airdrop_restart(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr
) -> Result<Response, ContractError> { 
    util::check_enabled(deps.storage, collection_address.clone())?;
    util::check_collection_owner(deps.storage, collection_address.clone(), info.sender.clone())?;

    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
    collection.started_at = env.block.time.seconds();
    collection.is_started = true;
    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;

    Ok(Response::new()
        .add_attribute("collection_address", collection_address.clone())
        .add_attribute("action", "execute_airdrop_restart")
    )
}
pub fn execute_stake (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    token_id: Vec<String>
) -> Result<Response, ContractError> {
    util::check_enabled(deps.storage, collection_address.clone())?;
    util::check_airdrop_start(deps.storage, collection_address.clone())?;

    let cfg = CONFIG.load(deps.storage)?;
    let fee_amount = match must_pay(&info, &cfg.native_token) {
        Ok(it) => it,
        Err(_err) => return Err(ContractError::InsufficientCw20 {  }),
    }.u128();

    if fee_amount < u128::from(cfg.tx_fee) {
        return Err(ContractError::InsufficientCw20 {  })
    }

    let mut msgs:Vec<CosmosMsg> = vec![];
    let mut nftinfos = vec![];
    let user_addr = info.sender.clone();

    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
   
    let duration = collection.duration;

    for nft_id in token_id.iter() {
        msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collection_address.clone().to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                token_id: nft_id.clone(),
                recipient: env.contract.address.clone().into()
            })?,
            funds: vec![],
        }));
        
        let _nftinfo = NftInfo {
            nft_id: nft_id.clone(),
            lock_time: duration + env.block.time.seconds(),
            airdrop: false,
            collection_address: collection.address.clone()
        };
        nftinfos.push(_nftinfo);
    }

    let _userinfo = UserInfo {
        address: user_addr.clone(),
        staked_nfts: nftinfos.clone(),
        total_earnd: Uint128::zero(),
        claimable: Uint128::zero(),
        earn_infos: vec![],
    };
    let mut new_users = collection.users.clone();
    let mut find = false;
    for (index, user_info) in new_users.iter().enumerate() {
        if user_info.address == user_addr.clone() {
            let mut _userinfo2 = user_info.clone();
            for nftinfo in nftinfos {
                _userinfo2.staked_nfts.push(nftinfo);
            }
            new_users[index] = _userinfo2;
            find = true;
            break;
        }
    }
    if !find {
        new_users.push(_userinfo);
    }
    collection.users = new_users;
    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "nft staking")
    )
    
}
pub fn execute_receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw721ReceiveMsg
) -> Result<Response, ContractError> {
    util::check_enabled(deps.storage, info.sender.clone())?;

    let msg: NftReceiveMsg = from_binary(&wrapper.msg)?;

    match msg {
        NftReceiveMsg::Stake {
            sender,
            token_id,
            collection_address
        } => {
            util::check_airdrop_start(deps.storage, collection_address.clone())?;
        
            if info.sender.clone() != collection_address.clone() {
                return Err(crate::ContractError::InvalidCw721Token {  });
            }
        
            let stake_nft_id = wrapper.token_id.clone();
            let user_addr = deps.api.addr_validate(wrapper.sender.as_str())?;

            if (sender != user_addr) || (token_id != stake_nft_id) {
                return Err(ContractError::InvalidCw721Msg {  });
            }

            let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
            let duration = collection.duration;
            let _nftinfo = NftInfo {
                nft_id: stake_nft_id.clone(),
                lock_time: duration+env.block.time.seconds(),
                airdrop: false,
                collection_address: collection.address.clone()
            };
            
            let _userinfo = UserInfo {
                address: user_addr.clone(),
                staked_nfts: vec![_nftinfo.clone()],
                total_earnd: Uint128::zero(),
                claimable: Uint128::zero(),
                earn_infos: vec![],
            };
            let mut new_users = collection.users.clone();
            let mut find = false;
            for (index, user_info) in new_users.iter().enumerate() {
                if user_info.address == user_addr.clone() {
                    let mut _userinfo2 = user_info.clone();
                    _userinfo2.staked_nfts.push(_nftinfo.clone());
                    new_users[index] = _userinfo2;
                    find = true;
                    break;
                }
            }
            if !find {
                new_users.push(_userinfo);
            }
            collection.users = new_users;
            COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;

            Ok(Response::new()
                .add_attribute("action", "execute_stake")
                .add_attribute("collection_address", collection.address.clone())
                .add_attribute("nft_id", stake_nft_id.clone())
            )
        }
    }
}

pub fn execute_restake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    token_id: Vec<String>,
) -> Result<Response, ContractError> {
    util::check_enabled(deps.storage, collection_address.clone())?;
    util::check_airdrop_start(deps.storage, collection_address.clone())?;

    let cfg = CONFIG.load(deps.storage)?;
    let fee_amount = match must_pay(&info, &cfg.native_token) {
        Ok(it) => it,
        Err(_err) => return Err(ContractError::InsufficientCw20 {  }),
    }.u128();

    if fee_amount < u128::from(cfg.tx_fee) {
        return Err(ContractError::InsufficientCw20 {  })
    }

    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
    let user_index = collection.users.iter().position(|user_info| user_info.address == info.sender).unwrap_or(usize::MAX);
    
    if user_index == usize::MAX {
        return Err(ContractError::NoStakedNft {  });
    }

    let mut userinfo = collection.users[user_index].clone();

    if userinfo.staked_nfts.is_empty() {
        return Err(ContractError::NoStakedNft {  });
    }
    
    for nft_id in token_id.iter() {
        let index = userinfo.staked_nfts.iter().position(|nft| nft.nft_id == nft_id.clone()).unwrap_or(usize::MAX);
        if index == usize::MAX {
            return Err(ContractError::NoStakedNft {  });
        }
        let mut nftinfo: NftInfo = userinfo.staked_nfts[index].clone();
        if nftinfo.collection_address != collection.address {
            return Err(ContractError::InvalidCw721Msg {  });
        }

        if nftinfo.lock_time > env.block.time.seconds() {
            continue;
        }

        nftinfo.lock_time = env.block.time.seconds() + collection.duration;
        nftinfo.airdrop = false;
        userinfo.staked_nfts[index] = nftinfo;
       
    }

    collection.users[user_index] = userinfo;

    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;

    Ok(Response::new()
        .add_attribute("action", "restake")
        .add_attribute("collection_address", collection.address.clone())
    )
    
}
pub fn execute_transfer_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection_address: Addr,
    nft_id: String,
    recipient: String,
) -> Result<Response, ContractError> {
    util::check_owner(deps.storage, info.sender.clone())?;
    let mut msgs:Vec<CosmosMsg> = vec![];
    msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection_address.clone().to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            token_id: nft_id,
            recipient: recipient.clone().into()
        })?,
        funds: vec![],
    }));
    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "transfer nft")
    )
}
pub fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    token_id: Vec<String>
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
    let user_index = collection.users.iter().position(|user_info| user_info.address == info.sender).unwrap_or(usize::MAX);
    
    if user_index == usize::MAX {
        return Err(ContractError::NoStakedNft {  });
    }

    let mut userinfo = collection.users[user_index].clone();

    if userinfo.staked_nfts.is_empty() {
        return Err(ContractError::NoStakedNft {  });
    }
    
    let mut msgs:Vec<CosmosMsg> = vec![];
    let mut total_fee = Uint128::zero();
    for nft_id in token_id.iter() {
        let index = userinfo.staked_nfts.iter().position(|nft| nft.nft_id == nft_id.clone()).unwrap_or(usize::MAX);
        if index == usize::MAX {
            return Err(ContractError::NoStakedNft {  });
        }
        let nftinfo: NftInfo = userinfo.staked_nfts[index].clone();
        if nftinfo.collection_address != collection.address {
            return Err(ContractError::InvalidCw721Token {  });
        }

        if (nftinfo.lock_time > env.block.time.seconds()) && (collection.unstaking_fee > Uint128::zero()) {
            total_fee += collection.unstaking_fee;
        }

        msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collection.address.clone().to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                token_id: nft_id.clone(),
                recipient: info.sender.clone().into()
            })?,
            funds: vec![],
        }));
        
        userinfo.staked_nfts.remove(index);
    }
    
	if total_fee > Uint128::zero() {
		let receive_fee = match must_pay(&info, &cfg.native_token) {
			Ok(it) => it,
			Err(_err) => return Err(ContractError::Locktime {  }),
		}.u128();

		if receive_fee >= u128::from(total_fee) {
			let owner_fee = u128::from(total_fee) * u128::from(collection.unstaking_fee_percent) / 100u128;
            if owner_fee > 0u128 {
                let owner_fee_msg = util::transfer_token_message(Denom::Native(cfg.native_token.clone()), Uint128::from(owner_fee), cfg.fee_address.clone())?;
                msgs.push(owner_fee_msg);
            }
			let fee_msg = util::transfer_token_message(Denom::Native(cfg.native_token.clone()), total_fee - Uint128::from(owner_fee), collection.fee_address.clone())?;
			msgs.push(fee_msg);
		} else {
			return Err(ContractError::Locktime {  });
		}
	}

    collection.users[user_index] = userinfo;

    COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "unstake")
    )

}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_address: Addr,
    cw20_address: Addr,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let mut collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
    let user_index = collection.users.iter().position(|user_info| user_info.address == info.sender).unwrap_or(usize::MAX);
    
    if user_index == usize::MAX {
        return Err(ContractError::NoStakedNft {  });
    }

    let mut userinfo = collection.users[user_index].clone();

    if userinfo.staked_nfts.is_empty() {
        return Err(ContractError::NoStakedNft {  });
    }
    
    let reward_msg;
    let amount;
    if cw20_address.to_string().len() == 0 {
        if userinfo.claimable == Uint128::zero() {
            return Err(ContractError::NoReward {  });
        }
        amount = userinfo.claimable.clone();
        if util::get_token_amount(deps.querier, Denom::Native(cfg.native_token.clone()), env.clone().contract.address.clone())? < amount {
            return Err(crate::ContractError::InsufficientCw20 {  });
        }
        reward_msg = util::transfer_token_message(Denom::Native(cfg.native_token.clone()), amount, info.sender.clone())?;
        userinfo.total_earnd += amount;
        userinfo.claimable = Uint128::zero();

        collection.users[user_index] = userinfo;

        COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
            
        return Ok(Response::new()
            .add_message(reward_msg)
            .add_attribute("action", "claim")
            .add_attribute("address", info.sender.clone().to_string())
            .add_attribute("claimed_amount", amount)
        );
    } else {
        let index = userinfo.earn_infos.iter().position(|earn_info| earn_info.cw20_address == cw20_address);
        match index {
            Some(index) => {
                amount = userinfo.earn_infos[index].claimable.clone();
                if util::get_token_amount(deps.querier, Denom::Cw20(collection.cw20_address.clone()), env.clone().contract.address.clone())? < amount {
                    return Err(crate::ContractError::InsufficientCw20 {  });
                }
                reward_msg = util::transfer_token_message(Denom::Cw20(collection.cw20_address.clone()), amount, info.sender.clone())?;
                userinfo.earn_infos[index].total_earned += amount;
                userinfo.earn_infos[index].claimable = Uint128::zero();

                collection.users[user_index] = userinfo;

                COLLECTION_MAP.save(deps.storage, collection_address.clone(), &collection)?;
                    
                return Ok(Response::new()
                    .add_message(reward_msg)
                    .add_attribute("action", "claim")
                    .add_attribute("address", info.sender.clone().to_string())
                    .add_attribute("claimed_amount", amount.clone())
                );
            },
            None => {
                return Err(ContractError::NoReward {  });
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetCollection {collection_address} => to_binary(&query_collection(deps, env, collection_address)?),
        QueryMsg::GetUserList {collection_address} => to_binary(&query_user_list(deps, env, collection_address)?),
        QueryMsg::GetCollectionList {} => to_binary(&query_collection_list(deps)?),
        QueryMsg::GetShownCollectionList {} => to_binary(&query_shown_collection_list(deps)?),
        QueryMsg::GetTotalLocked {collection_address} => to_binary(&query_total_locked(deps, env, collection_address)?),
        QueryMsg::GetStakedNfts { address, collection_address } => to_binary(&query_staked_nfts(deps, address, collection_address)?),
        QueryMsg::GetAirdropInfos { address } => to_binary(&query_airdrop_infos(deps, address)?),
        QueryMsg::GetEarnInfos { address, collection_address } => to_binary(&query_earn_infos(deps, address, collection_address)?),
    }
}

pub fn query_config(
    deps: Deps, 
) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        fee_address: config.fee_address,
        tx_fee: config.tx_fee
    })
}

pub fn query_collection(
    deps: Deps, 
    env: Env,
    collection_address: Addr,
) -> StdResult<CollectionResponse> {
    let collection = COLLECTION_MAP.load(deps.storage, collection_address)?;
    Ok(CollectionResponse {
        address: collection.address,
        owner: collection.owner,
        total_airdrop: collection.total_airdrop,
        airdropable: collection.airdropable,
        duration: collection.duration,
        started_at: collection.started_at,
        fee_address: collection.fee_address,
        cw20_address: collection.cw20_address,
        is_started: collection.is_started,
        is_show: collection.is_show,
        is_enabled: collection.is_enabled,
        unstaking_fee: collection.unstaking_fee,
        unstaking_fee_percent: collection.unstaking_fee_percent,
        server_time: env.block.time.seconds()
    })
}

pub fn query_user_list(
    deps: Deps, 
    _env: Env, 
    collection_address: Addr
) -> StdResult<UserListResponse> {
    let result = util::get_all_addresses(deps.storage, collection_address.clone());

    match result {
        Ok(addresses) => {
            let mut result = Vec::new();
            for address in addresses {
                let nfts = query_staked_nfts(deps, address.clone(), collection_address.clone())?;
                if nfts.nft_maps.len() > 0 {
                    result.push(address.clone())
                }
            }
            Ok(UserListResponse {
                addresses: result
            })
        },
        Err(_error) => {
            Ok(UserListResponse {
                addresses: Vec::new()
            })
        }
    }
}

pub fn query_collection_list(deps: Deps) -> StdResult<CollectionListResponse> {
    let collections = util::get_all_collections(deps.storage);

    Ok(CollectionListResponse {
        collections
    })
}

pub fn query_shown_collection_list(deps: Deps) -> StdResult<CollectionListResponse> {
    let collections = util::get_all_shown_collections(deps.storage);

    Ok(CollectionListResponse {
        collections
    })
}

pub fn query_total_locked(
    deps: Deps, 
    env: Env,
    collection_address: Addr,
) -> StdResult<TotalLockedResponse> {
    let nft_count = util::get_in_locktime_nft_count(deps.storage, env.block.clone(), collection_address.clone());
    match nft_count {
        Ok(nft_count) => {
            Ok(TotalLockedResponse {
                count: nft_count.clone()
            })
        },
        Err(_error) => {
            Ok(TotalLockedResponse {
                count: Uint128::zero()
            })
        }
    }
}

pub fn query_staked_nfts(
    deps: Deps, 
    address: Addr,
    collection_address: Addr,
) -> StdResult<StakedNftsResponse> {
    let collection = COLLECTION_MAP.load(deps.storage, collection_address.clone())?;
    let user_index = collection.users.iter().position(|user_info| user_info.address == address).unwrap_or(usize::MAX);
    
    if user_index == usize::MAX {
        return Ok(StakedNftsResponse { nft_maps: vec![]})
    }

    let userinfo = collection.users[user_index].clone();

    let mut nft_maps : Vec<StakedNftResponse> = Vec::new();
    for nft in userinfo.staked_nfts {
        if nft.collection_address != collection_address {
            continue;
        }
        nft_maps.push(StakedNftResponse { 
            nft_id: nft.nft_id, 
            airdrop: nft.airdrop, 
            lock_time: nft.lock_time,
        })
    }
    Ok(StakedNftsResponse { nft_maps })
}

pub fn query_earn_infos(
    deps: Deps, 
    address: Addr,
    collection_address: Addr
) -> StdResult<EarnInfosResponse> {
    let collection = COLLECTION_MAP.load(deps.storage, collection_address)?;
    let user_index = collection.users.iter().position(|user_info| user_info.address == address).unwrap_or(usize::MAX);
    
    if user_index == usize::MAX {
        return  Ok(EarnInfosResponse {
            total_earned: Uint128::zero(),
            claimable: Uint128::zero(),
            earn_infos: vec![],
        })
    }

    let userinfo = collection.users[user_index].clone();

    Ok(EarnInfosResponse {
        total_earned: userinfo.total_earnd.clone(),
        claimable: userinfo.claimable.clone(),
        earn_infos: userinfo.earn_infos.clone(),
    })
}

pub fn query_airdrop_infos(
    deps: Deps, 
    address: Addr
) -> StdResult<AirdropInfosResponse> {
    let collection = COLLECTION_MAP.load(deps.storage, address);

    match collection {
        Ok(collection) => {
            Ok(AirdropInfosResponse {
                total_airdrop: collection.total_airdrop.clone(),
                airdropable: collection.airdropable.clone(),
                airdrop_infos: collection.airdrop_infos.clone(),
            })
        },
        Err(_error) => {
            Ok(AirdropInfosResponse {
                total_airdrop: Uint128::zero(),
                airdropable: Uint128::zero(),
                airdrop_infos: vec![],
            })
        }
    }
}