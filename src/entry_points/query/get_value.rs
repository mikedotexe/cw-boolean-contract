use crate::state::CONFIG;
use cosmwasm_std::{Deps, Env, StdError, StdResult};

pub fn query(deps: Deps, _env: Env) -> StdResult<bool> {
    // Set our state variable according to the input
    let config = CONFIG.load(deps.storage);

    match config {
        Ok(c) => Ok(c.is_true),
        Err(_) => Err(StdError::generic_err(
            "Could not load config which has the boolean",
        )),
    }
}
