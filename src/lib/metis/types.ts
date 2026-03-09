export interface ErrorEnvelope {
  code: string
  message: string
  details?: string | null
}

export type CommandResponse<T> =
  | { status: 'ok'; data: T }
  | { status: 'err'; error: ErrorEnvelope }

export type ChannelSourceType = 'Manual' | 'External'
export type ChannelStatus = 'Active' | 'Archived' | 'Suspended'
export type TaskState = 'Queued' | 'Running' | 'Completed' | 'Failed' | 'Cancelled'
export type WorkerState = 'Pending' | 'Running' | 'Completed' | 'Failed' | 'Stopped'
export type HistoryRole = 'System' | 'User' | 'Assistant' | 'Tool'

export interface Channel {
  id: string
  title: string
  source_type: ChannelSourceType
  source_ref?: string | null
  status: ChannelStatus
  created_at: string
  updated_at: string
}

export interface Branch {
  id: string
  channel_id: string
  parent_branch_id?: string | null
  name: string
  is_active: boolean
  created_at: string
  updated_at: string
}

export interface Task {
  id: string
  channel_id: string
  branch_id?: string | null
  kind: string
  state: TaskState
  priority: number
  payload_json?: string | null
  created_at: string
  updated_at: string
  started_at?: string | null
  finished_at?: string | null
}

export interface Worker {
  id: string
  task_id: string
  worker_type: string
  state: WorkerState
  attempt: number
  last_heartbeat_at?: string | null
  started_at?: string | null
  finished_at?: string | null
  created_at: string
  updated_at: string
}

export interface HistoryEvent {
  id: string
  channel_id: string
  branch_id?: string | null
  task_id?: string | null
  worker_id?: string | null
  event_type: string
  role?: HistoryRole | null
  content_json: string
  correlation_id?: string | null
  created_at: string
}

export interface CreateChannelRequest {
  channel: Channel
}

export interface UpdateChannelStatusRequest {
  channel_id: string
  status: ChannelStatus
}

export interface ListBranchesByChannelRequest {
  channel_id: string
}

export interface EnqueueTaskRequest {
  task: Task
}

export interface UpdateTaskStateRequest {
  task_id: string
  state: TaskState
}

export interface ListTasksByChannelRequest {
  channel_id: string
}

export interface ListWorkersByTaskRequest {
  task_id: string
}

export interface CreateWorkerRequest {
  worker: Worker
}

export interface UpdateWorkerStateRequest {
  worker_id: string
  state: WorkerState
}

export interface WorkerHeartbeatRequest {
  worker_id: string
  heartbeat_at: string
}

export interface ListHistoryByChannelRequest {
  channel_id: string
}

export interface ListHistoryByBranchRequest {
  branch_id: string
}

export interface AppendHistoryRequest {
  event: HistoryEvent
}
