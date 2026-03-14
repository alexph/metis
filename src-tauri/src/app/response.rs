use metis_contract::error::ErrorEnvelope;
use serde::{Deserialize, Serialize};

use crate::app::errors::CommandError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CommandResponse<T> {
    Ok { data: T },
    Err { error: ErrorEnvelope },
}

pub(crate) fn command_result<T>(result: Result<T, CommandError>) -> CommandResponse<T> {
    match result {
        Ok(data) => CommandResponse::Ok { data },
        Err(error) => CommandResponse::Err {
            error: error.to_envelope(),
        },
    }
}
