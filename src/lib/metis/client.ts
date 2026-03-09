import { invoke } from '@tauri-apps/api/core'

import type {
  AppendHistoryRequest,
  Branch,
  Channel,
  CommandResponse,
  CreateChannelRequest,
  CreateWorkerRequest,
  EnqueueTaskRequest,
  ErrorEnvelope,
  HistoryEvent,
  ListBranchesByChannelRequest,
  ListHistoryByBranchRequest,
  ListHistoryByChannelRequest,
  ListTasksByChannelRequest,
  ListWorkersByTaskRequest,
  Task,
  UpdateChannelStatusRequest,
  UpdateTaskStateRequest,
  UpdateWorkerStateRequest,
  Worker,
  WorkerHeartbeatRequest,
} from './types'

export class MetisInvokeError extends Error {
  readonly code: string
  readonly details?: string

  constructor(envelope: ErrorEnvelope) {
    super(envelope.message)
    this.name = 'MetisInvokeError'
    this.code = envelope.code
    this.details = envelope.details ?? undefined
  }
}

const COMMANDS = {
  channelsList: 'desktop_channels_list',
  channelsCreate: 'desktop_channels_create',
  channelsUpdateStatus: 'desktop_channels_update_status',
  branchesListByChannel: 'desktop_branches_list_by_channel',
  tasksEnqueue: 'desktop_tasks_enqueue',
  tasksUpdateState: 'desktop_tasks_update_state',
  tasksListByChannel: 'desktop_tasks_list_by_channel',
  workersListByTask: 'desktop_workers_list_by_task',
  workersCreate: 'desktop_workers_create',
  workersUpdateState: 'desktop_workers_update_state',
  workersHeartbeat: 'desktop_workers_heartbeat',
  historyListByChannel: 'desktop_history_list_by_channel',
  historyListByBranch: 'desktop_history_list_by_branch',
  historyAppend: 'desktop_history_append',
} as const

async function invokeCommand<TData, TRequest = never>(
  command: string,
  request?: TRequest,
): Promise<TData> {
  try {
    const response = request
      ? await invoke<CommandResponse<TData>>(command, { request })
      : await invoke<CommandResponse<TData>>(command)

    if (response.status === 'ok') {
      return response.data
    }

    throw new MetisInvokeError(response.error)
  } catch (error) {
    if (error instanceof MetisInvokeError) {
      throw error
    }

    throw new MetisInvokeError({
      code: 'desktop_invoke_error',
      message: 'failed to invoke desktop command',
      details: error instanceof Error ? error.message : String(error),
    })
  }
}

export const metisClient = {
  channelsList: () => invokeCommand<Channel[]>(COMMANDS.channelsList),
  channelsCreate: (request: CreateChannelRequest) =>
    invokeCommand<Channel, CreateChannelRequest>(COMMANDS.channelsCreate, request),
  channelsUpdateStatus: (request: UpdateChannelStatusRequest) =>
    invokeCommand<Channel, UpdateChannelStatusRequest>(
      COMMANDS.channelsUpdateStatus,
      request,
    ),
  branchesListByChannel: (request: ListBranchesByChannelRequest) =>
    invokeCommand<Branch[], ListBranchesByChannelRequest>(
      COMMANDS.branchesListByChannel,
      request,
    ),
  tasksEnqueue: (request: EnqueueTaskRequest) =>
    invokeCommand<Task, EnqueueTaskRequest>(COMMANDS.tasksEnqueue, request),
  tasksUpdateState: (request: UpdateTaskStateRequest) =>
    invokeCommand<Task, UpdateTaskStateRequest>(COMMANDS.tasksUpdateState, request),
  tasksListByChannel: (request: ListTasksByChannelRequest) =>
    invokeCommand<Task[], ListTasksByChannelRequest>(COMMANDS.tasksListByChannel, request),
  workersListByTask: (request: ListWorkersByTaskRequest) =>
    invokeCommand<Worker[], ListWorkersByTaskRequest>(
      COMMANDS.workersListByTask,
      request,
    ),
  workersCreate: (request: CreateWorkerRequest) =>
    invokeCommand<Worker, CreateWorkerRequest>(COMMANDS.workersCreate, request),
  workersUpdateState: (request: UpdateWorkerStateRequest) =>
    invokeCommand<Worker, UpdateWorkerStateRequest>(
      COMMANDS.workersUpdateState,
      request,
    ),
  workersHeartbeat: (request: WorkerHeartbeatRequest) =>
    invokeCommand<Worker, WorkerHeartbeatRequest>(COMMANDS.workersHeartbeat, request),
  historyListByChannel: (request: ListHistoryByChannelRequest) =>
    invokeCommand<HistoryEvent[], ListHistoryByChannelRequest>(
      COMMANDS.historyListByChannel,
      request,
    ),
  historyListByBranch: (request: ListHistoryByBranchRequest) =>
    invokeCommand<HistoryEvent[], ListHistoryByBranchRequest>(
      COMMANDS.historyListByBranch,
      request,
    ),
  historyAppend: (request: AppendHistoryRequest) =>
    invokeCommand<HistoryEvent, AppendHistoryRequest>(COMMANDS.historyAppend, request),
}
