use std::convert::{From, TryFrom};
use cosmwasm_std::{
    to_binary,  Response, StdResult, Uint128, Coin, BankMsg,
    WasmMsg, WasmQuery, QueryRequest, Addr, Storage, CosmosMsg,  QuerierWrapper, BalanceResponse as NativeBalanceResponse, BankQuery, Order, BlockInfo
};
use cw20::{Cw20ExecuteMsg, Denom, BalanceResponse as CW20BalanceResponse, Cw20QueryMsg};
use crate::error::ContractError;
use crate::state::{
    CONFIG,
    COLLECTION_MAP,
    Collection,
};

pub fn check_owner(
    storage: &mut dyn Storage,
    address: Addr
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;
    
    if address != cfg.owner && address != cfg.creator {
        return Err(ContractError::Unauthorized {  })
    }
    Ok(Response::new()
        .add_attribute("action", "check_owner")
    )
}

pub fn check_enabled(
    storage: &mut dyn Storage,
    collection_address: Addr
) -> Result<Response, ContractError> {
    let collection = COLLECTION_MAP.load(storage, collection_address)?;
    if !collection.is_enabled {
        return Err(ContractError::Disabled {  })
    }
    Ok(Response::new().add_attribute("action", "check_enabled"))
}

pub fn check_airdrop_start(
    storage: &mut dyn Storage,
    collection_address: Addr
) -> Result<Response, ContractError> {
    let collection = COLLECTION_MAP.load(storage, collection_address)?;
    if !collection.is_started {
        return Err(ContractError::NotStarted {  })
    }
    Ok(Response::new().add_attribute("action", "check_airdrop_start"))
}

pub fn check_collection_owner(
    storage: &mut dyn Storage,
    collection_address: Addr,
    address: Addr
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;
    let collection = COLLECTION_MAP.load(storage, collection_address)?;
    if address != collection.owner && address != cfg.owner {
        return Err(ContractError::Unauthorized {  })
    }
    Ok(Response::new().add_attribute("action", "check_collection_owner"))
}

pub fn execute_update_owner(
    storage: &mut dyn Storage,
    address: Addr,
    owner: Addr,
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;
    
    CONFIG.update(storage, |mut exists| -> StdResult<_> {
        exists.owner = owner.clone();
        Ok(exists)
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_owner")
        .add_attribute("owner", owner.clone())
    )
}

pub fn execute_update_fee_address(
    storage: &mut dyn Storage,
    address: Addr,
    fee_address: Addr,
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;
    
    CONFIG.update(storage, |mut exists| -> StdResult<_> {
        exists.fee_address = fee_address.clone();
        Ok(exists)
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_fee_address")
        .add_attribute("fee_address", fee_address.clone())
    )
}

pub fn execute_update_tx_fee(
    storage: &mut dyn Storage,
    address: Addr,
    tx_fee: Uint128,
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;
    
    CONFIG.update(storage, |mut exists| -> StdResult<_> {
        exists.tx_fee = tx_fee.clone();
        Ok(exists)
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_tx_fee")
        .add_attribute("tx_fee", tx_fee.clone())
    )
}

pub fn execute_registe_collection(
    storage: &mut dyn Storage,
    address: Addr,
    collection_address: Addr,
    owner: Addr,
    cw20_address: Addr,
    new_unstaking_fee: Uint128,
    new_unstaking_fee_percent: u64,
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;

    let collection = Collection {
        address: collection_address.clone(),
        owner: owner.clone(),
        total_airdrop: Uint128::zero(),
        airdropable: Uint128::zero(),
        airdrop_infos: vec![],
        started_at: 0u64,
        duration: 0,
        fee_address: owner.clone(),
        cw20_address: cw20_address.clone(),
        is_started: false,
        is_show: true,
        is_enabled: true,
        unstaking_fee: new_unstaking_fee,
        unstaking_fee_percent: new_unstaking_fee_percent,
        users: vec![]
    };
    COLLECTION_MAP.save(storage, collection_address.clone(), &collection)?;
    Ok(Response::new().add_attribute("action", "registe_collection"))
}

pub fn execute_update_collection_fee(
    storage: &mut dyn Storage,
    address: Addr,
    collection_address: Addr,
    new_unstaking_fee: Uint128,
    new_unstaking_fee_percent: u64,
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;

    let exist = COLLECTION_MAP.load(storage, collection_address.clone());
    match exist {
        Ok(mut collection) => {
            collection.unstaking_fee = new_unstaking_fee;
            collection.unstaking_fee_percent = new_unstaking_fee_percent;
            COLLECTION_MAP.save(storage, collection_address, &collection)?;

            Ok(Response::new().add_attribute("action", "update_collection_fee"))
        },
        Err(_) => {
            return Err(crate::ContractError::InvalidCollection {  });
        }
    }
}

pub fn execute_update_collection_state(
    storage: &mut dyn Storage,
    address: Addr,
    collection_address: Addr,
    is_show: bool
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;

    let exist = COLLECTION_MAP.load(storage, collection_address.clone());
    match exist {
        Ok(mut collection) => {
            collection.is_show = is_show;
            COLLECTION_MAP.save(storage, collection_address, &collection)?;

            Ok(Response::new().add_attribute("action", "update_collection_state"))
        },
        Err(_) => {
            return Err(crate::ContractError::InvalidCollection {  });
        }
    }
}

pub fn execute_update_collection(
    storage: &mut dyn Storage,
    address: Addr,
    collection_address: Addr,
    new_cw20_address: Addr,
    new_owner: Addr,
    new_duration: u64,
    new_fee_address: Addr,
    is_enabled: bool,
) -> Result<Response, ContractError> {
    check_collection_owner(storage, collection_address.clone(), address)?;

    let exist = COLLECTION_MAP.load(storage, collection_address.clone());
    match exist {
        Ok(mut collection) => {
            collection.owner = new_owner.clone();
            collection.cw20_address = new_cw20_address.clone();
            collection.duration = new_duration;
            collection.fee_address = new_fee_address.clone();
            collection.is_enabled = is_enabled;
            COLLECTION_MAP.save(storage, collection_address.clone(), &collection)?;

            Ok(Response::new().add_attribute("action", "update_collection"))
        },
        Err(_) => {
            return Err(crate::ContractError::InvalidCollection {  });
        }
    }
}

pub fn transfer_token_message(
    denom: Denom,
    amount: Uint128,
    receiver: Addr
) -> Result<CosmosMsg, ContractError> {

    match denom.clone() {
        Denom::Native(native_str) => {
            return Ok(BankMsg::Send {
                to_address: receiver.clone().into(),
                amount: vec![Coin{
                    denom: native_str.clone(),
                    amount
                }]
            }.into());
        },
        Denom::Cw20(native_token) => {
            return Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: native_token.clone().into(),
                funds: vec![],
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: receiver.clone().into(),
                    amount
                })?,
            }));
        }
    }
}

pub fn transfer_from_token_message(
    denom: Denom,
    owner: Addr,
    receiver: Addr,
    amount: Uint128,
) -> Result<CosmosMsg, ContractError> {

    match denom.clone() {
        Denom::Native(_native_str) => {
            return Err(ContractError::InsufficientCw20 {  });
        },
        Denom::Cw20(native_token) => {
            return Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: native_token.clone().into(),
                funds: vec![],
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: owner.clone().into(),
                    recipient: receiver.clone().into(),
                    amount
                })?,
            }));
        }
    }
}

pub fn get_token_amount(
    querier: QuerierWrapper,
    denom: Denom,
    contract_addr: Addr
) -> Result<Uint128, ContractError> {

    match denom.clone() {
        Denom::Native(native_str) => {
            let native_response: NativeBalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
                address: contract_addr.clone().into(),
                denom: native_str
            }))?;
            return Ok(native_response.amount.amount);
        },
        Denom::Cw20(native_token) => {
            let balance_response: CW20BalanceResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: native_token.clone().into(),
                msg: to_binary(&Cw20QueryMsg::Balance {address: contract_addr.clone().into()})?,
            }))?;
            return Ok(balance_response.balance);
        }
    }
}

pub fn get_in_locktime_nft_count(
    storage: &dyn Storage,
    block: BlockInfo,
    collection_address: Addr,
) -> Result<Uint128, ContractError> {
    let mut count = 0;
    let collection = COLLECTION_MAP.load(storage, collection_address.clone())?;
    let users = collection.users;
    for userinfo in users.iter() {
        count += userinfo.staked_nfts
            .iter()
            .filter(|nftinfo| 
                (nftinfo.lock_time > block.time.seconds() && nftinfo.collection_address == collection_address)
            ).count();
    }

    return Ok(Uint128::from(u128::try_from(count).unwrap()));
        
}

pub fn get_all_addresses(
    storage: &dyn Storage,
    collection_address: Addr,
) -> Result<Vec<Addr>, ContractError> {
    let mut addresses = Vec::new();
    let collection = COLLECTION_MAP.load(storage, collection_address)?;
    for userinfo in collection.users.iter() {
        addresses.push(userinfo.address.clone());
    }
    return Ok(addresses)
}

pub fn get_all_collections(
    storage: &dyn Storage,
) -> Vec<Addr> {
    let mut collections = Vec::new();
    let result: StdResult<Vec<(Addr, Collection)>> = COLLECTION_MAP.range(storage, None, None, Order::Ascending).collect();
    
    match result {
        Ok(all_collections) => {
            for (address, _collection) in all_collections.iter() {
                collections.push(address.clone());
            }
    
            return collections;
        },
        Err(_error) => {
            return  Vec::new();
        }
    }
}

pub fn get_all_shown_collections(
    storage: &dyn Storage,
) -> Vec<Addr> {
    let mut collections = Vec::new();
    let result: StdResult<Vec<(Addr, Collection)>> = COLLECTION_MAP.range(storage, None, None, Order::Ascending).collect();
    
    match result {
        Ok(all_collections) => {
            for (address, collection) in all_collections.iter() {
                if collection.is_show {
                    collections.push(address.clone());
                }
            }
    
            return collections;
        },
        Err(_error) => {
            return  Vec::new();
        }
    }
}