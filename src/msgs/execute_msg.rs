use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    SetValue { is_true: bool },
    // This execute message takes no parameters
    Toggle {},
}
