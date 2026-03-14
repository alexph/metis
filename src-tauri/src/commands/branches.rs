use metis_contract::branch::Branch;

use crate::{
    app::requests::ListBranchesByChannelRequest,
    app::response::{command_result, CommandResponse},
    app::use_cases::{AppUseCases, CommandUseCases},
};

#[tauri::command]
pub fn desktop_branches_list_by_channel(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    handle_branches_list_by_channel(app_use_cases.inner(), request)
}

pub(crate) fn handle_branches_list_by_channel(
    use_cases: &dyn CommandUseCases,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    command_result(use_cases.branches_list_by_channel(request))
}
