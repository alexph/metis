use metis_contract::branch::Branch;
use serde::{Deserialize, Serialize};

use crate::commands::{command_result, service::{CommandService, CommandServices}, CommandResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBranchesByChannelRequest {
    pub channel_id: String,
}

#[tauri::command]
pub fn desktop_branches_list_by_channel(
    services: tauri::State<'_, CommandServices>,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    handle_branches_list_by_channel(services.command_service(), request)
}

pub(crate) fn handle_branches_list_by_channel(
    service: &dyn CommandService,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    command_result(service.branches_list_by_channel(request))
}
