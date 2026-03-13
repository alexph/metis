pub mod branches;
pub mod channels;
pub mod errors;
pub mod events;
pub mod history;
pub mod service;
pub mod tasks;
pub mod workers;

use metis_contract::error::ErrorEnvelope;
use serde::{Deserialize, Serialize};

use crate::commands::{errors::CommandError, events::{emit_tauri_event, event_name, CommandEvent}};

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

pub(crate) fn emit_best_effort(app: Option<&tauri::AppHandle>, event: CommandEvent) {
    let Some(app) = app else {
        return;
    };

    let emitted_event_name = event_name(&event);
    if let Err(error) = emit_tauri_event(app, event) {
        tracing::warn!(
            event_name = emitted_event_name,
            code = %error.code,
            message = %error.message,
            "command event emission failed"
        );
    }
}
