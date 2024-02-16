use std::str::FromStr;

use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::transaction::errors::ParseError;

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, EnumIter, Eq, Hash, PartialEq, Encode, Decode,
)]
#[cfg_attr(feature = "scale-info", derive(scale_info::TypeInfo))]
pub enum TransactionType {
    Declare,
    DeployAccount,
    InvokeFunction,
    L1Handler,
}

impl FromStr for TransactionType {
    type Err = ParseError;

    fn from_str(tx_type: &str) -> Result<Self, Self::Err> {
        match tx_type {
            "Declare" | "DECLARE" => Ok(TransactionType::Declare),
            "DeployAccount" | "DEPLOY_ACCOUNT" => Ok(TransactionType::DeployAccount),
            "InvokeFunction" | "INVOKE_FUNCTION" => Ok(TransactionType::InvokeFunction),
            "L1Handler" | "L1_HANDLER" => Ok(TransactionType::L1Handler),
            unknown_tx_type => Err(ParseError::UnknownTransactionType(unknown_tx_type.to_string())),
        }
    }
}
