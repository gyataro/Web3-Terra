#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{FortuneResponse, MigrateMsg, InstantiateMsg, ExecuteMsg, QueryMsg, ScoreResponse};
use crate::state::{State, STORAGE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg
) -> StdResult<Response> {

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {

  // We're storing stuff in a variable called "state" of type "State"
  let state = State {
    fortune: msg.fortune,
    owner: info.sender.clone(),
    scores: vec![],
  };

  // We're setting the contract version using a helper function we imported
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  // We're storing state in a special variable called "STATE"
  STORAGE.save(deps.storage, &state)?;

  // Sending a response back to the caller
  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", info.sender)
    .add_attribute("fortune", msg.fortune.to_string())
    .add_attribute("scores", "".to_string()))
}

// Here's our execute message handler, we need `info` as a parameter too
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    // `score` is being passing in, we'll pass that forward
    ExecuteMsg::UpsertScore { score } => try_upsert_score(deps, info, score),
    ExecuteMsg::Send { addr, amount } => send(deps, info, addr, amount),
  }
}

// Here's our main upsert function - it adds a score if the address doesn't exist, or updates it if it does
fn try_upsert_score(
  deps: DepsMut,
  info: MessageInfo,
  score: u16,
) -> Result<Response, ContractError> {
  let mut state = STORAGE.load(deps.storage)?;
  let sender = info.sender.clone();
  let scores = &mut state.scores;
  let index = scores.iter().position(|(s, _)| s == &sender);
  match index {
    Some(i) => {
      scores[i].1 = score;
    },
    None => {
      scores.push((sender.clone(), score));
    }
  }
  STORAGE.save(deps.storage, &state)?;
  Ok(Response::new()
    .add_attribute("method", "upsert")
    .add_attribute("player", info.sender)
    .add_attribute("score", score.to_string()))
}

// Send UST from smart contract to an EOA
fn send(
  deps: DepsMut,
  info: MessageInfo,
  addr: String,
  amount: Uint128
) -> Result<Response, ContractError> {

  // Only contract owner can reward players
  let state = STORAGE.load(deps.storage)?;

  if state.owner != info.sender
  {
    return Err(ContractError::Unauthorized {}); 
  }

  let msg = CosmosMsg::Bank(BankMsg::Send {
    to_address: addr.clone(),
    amount: vec![
      Coin {
        denom: "uusd".to_string(),
        amount: amount,
    }],
  });

  Ok(Response::new()
    .add_attribute("method", "send")
    .add_attribute("player", addr.to_string())
    .add_attribute("amount", amount.to_string())
    .add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
      QueryMsg::GetFortune {} => to_binary(&query_fortune(deps)?),
      QueryMsg::GetScores {} => to_binary(&query_scores(deps)?),
  }
}

fn query_fortune(deps: Deps) -> StdResult<FortuneResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(FortuneResponse { fortune: state.fortune })
}

fn query_scores(deps: Deps) -> StdResult<ScoreResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(ScoreResponse { scores: state.scores })
}