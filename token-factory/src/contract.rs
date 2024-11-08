use std::vec;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, MintedTokens, QueryMsg};
use crate::state::{Config, CONFIG, MINTED_TOKENS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};

use cw2::set_contract_version;

use cw20_bonding::msg::{InstantiateMsg as BondingInstantiateMsg, ExecuteMsg as BondingExecuteMsg, CurveType};



/* Define contract name and version */
const CONTRACT_NAME: &str = "crates.io:token-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    /* Define the initial configuration for this contract that way you can
    limit the type of coin you want to accept each time a token-factory is
    created and also which kind of token would you like to mint based on
    the code id of the contract deployed */
    let state = Config {
        stable_denom: msg.stable_denom.to_string(),
        token_contract_code_id: msg.token_contract_code_id,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &state)?;
    MINTED_TOKENS.save(deps.storage, &Vec::new())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute(
            "token_contract_code_id",
            msg.token_contract_code_id.to_string(),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateBondingToken {
            name,
            symbol,
        } => execute_create_bonding_token(
            deps, env, info, name, symbol,
        ),
    }
}

pub fn execute_create_bonding_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    symbol: String,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;

// Define the curve type with Decimal slope and scale
let curve_type = CurveType::Linear {
    slope: Uint128::new(1),
    scale: 6,                               
};

// Construct `cw20-bonding` instantiation message
let bonding_instantiate_msg = BondingInstantiateMsg {
    name: name.clone(),
    symbol,
    decimals: 6,
    reserve_denom: config.stable_denom.clone(),
    reserve_decimals: 6, 
    curve_type,
};


    // Create a SubMsg to instantiate the bonding token
    let instantiate_bonding_token = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),  // Ensure factory has admin permissions
        code_id: config.token_contract_code_id,
        msg: to_binary(&bonding_instantiate_msg)?,
        funds: vec![],
        label: format!("Bonding Token {}", name.clone()),
    };
    

    // let sub_msg = SubMsg::reply_on_success(instantiate_bonding_token, INSTANTIATE_REPLY_ID);
    let sub_msg = SubMsg::new(instantiate_bonding_token);


    Ok(Response::new()
        .add_attribute("method", "execute_create_bonding_token")
        .add_submessage(sub_msg))
}


// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
//     match msg.id {
//         INSTANTIATE_REPLY_ID => {
//             handle_instantiate_reply(deps, msg)
//                 .map(|res| res.add_attribute("reply", "instantiate_success"))
//         }
//         _ => Err(StdError::generic_err("Unknown reply id")),
//     }
// }


// fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
//     let result = msg.result.into_result().map_err(|_| StdError::generic_err("Instantiate failed"))?;

//     // Extract the contract address from the data field
//     let contract_address = match result.data {
//         Some(data) => {
//             // The data is expected to be the contract address as a UTF-8 string
//             let address_str = String::from_utf8(data).map_err(|_| StdError::generic_err("Invalid UTF-8 data"))?;
//             // Validate and parse the address
//             deps.api.addr_validate(&address_str)?.to_string()
//         },
//         None => return Err(StdError::generic_err("No data in instantiate reply")),
//     };

//     // Store the new bonding token contract address
//     MINTED_TOKENS.update(deps.storage, |mut tokens| -> StdResult<Vec<String>> {
//         tokens.push(contract_address.clone());
//         Ok(tokens)
//     })?;

//     Ok(Response::new()
//         .add_attribute("method", "handle_instantiate_reply")
//         .add_attribute("contract_address", contract_address))
// }



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        /* Return the list of all tokens that were minted thru this contract */
        QueryMsg::GetMintedTokens {} => to_binary(&query_minted_tokens(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?)
    }
}

fn query_minted_tokens(deps: Deps) -> StdResult<MintedTokens> {
    Ok(MintedTokens {
        minted_tokens: MINTED_TOKENS.load(deps.storage)?,
    })
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

/* In case you want to upgrade this contract you can find information about
how to migrate the contract in the following link:
https://docs.terra.money/docs/develop/dapp/quick-start/contract-migration.html*/
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
