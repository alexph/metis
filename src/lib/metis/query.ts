import {
  QueryClient,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { useEffect } from 'react'

import { metisClient } from './client'
import {
  syncBranches,
  syncChannels,
  syncHistory,
  syncTasks,
  syncWorkers,
  upsertChannel,
  upsertHistory,
  upsertTask,
  upsertWorker,
} from './db'
import type {
  AppendHistoryRequest,
  Channel,
  CreateChannelRequest,
  CreateWorkerRequest,
  EnqueueTaskRequest,
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

export const metisQueryClient = new QueryClient()

export const metisQueryKeys = {
  channels: () => ['metis', 'channels'] as const,
  branchesByChannel: (channelId: string) =>
    ['metis', 'branches', 'channel', channelId] as const,
  tasksByChannel: (channelId: string) => ['metis', 'tasks', 'channel', channelId] as const,
  workersByTask: (taskId: string) => ['metis', 'workers', 'task', taskId] as const,
  historyByChannel: (channelId: string) =>
    ['metis', 'history', 'channel', channelId] as const,
  historyByBranch: (branchId: string) =>
    ['metis', 'history', 'branch', branchId] as const,
}

export function useChannelsQuery() {
  const query = useQuery({
    queryKey: metisQueryKeys.channels(),
    queryFn: () => metisClient.channelsList(),
  })

  useEffect(() => {
    if (query.data) {
      syncChannels(query.data)
    }
  }, [query.data])

  return query
}

export function useBranchesByChannelQuery(request: ListBranchesByChannelRequest) {
  const query = useQuery({
    queryKey: metisQueryKeys.branchesByChannel(request.channel_id),
    queryFn: () => metisClient.branchesListByChannel(request),
    enabled: request.channel_id.trim().length > 0,
  })

  useEffect(() => {
    if (query.data) {
      syncBranches(query.data)
    }
  }, [query.data])

  return query
}

export function useTasksByChannelQuery(request: ListTasksByChannelRequest) {
  const query = useQuery({
    queryKey: metisQueryKeys.tasksByChannel(request.channel_id),
    queryFn: () => metisClient.tasksListByChannel(request),
    enabled: request.channel_id.trim().length > 0,
  })

  useEffect(() => {
    if (query.data) {
      syncTasks(query.data)
    }
  }, [query.data])

  return query
}

export function useWorkersByTaskQuery(request: ListWorkersByTaskRequest) {
  const query = useQuery({
    queryKey: metisQueryKeys.workersByTask(request.task_id),
    queryFn: () => metisClient.workersListByTask(request),
    enabled: request.task_id.trim().length > 0,
  })

  useEffect(() => {
    if (query.data) {
      syncWorkers(query.data)
    }
  }, [query.data])

  return query
}

export function useHistoryByChannelQuery(request: ListHistoryByChannelRequest) {
  const query = useQuery({
    queryKey: metisQueryKeys.historyByChannel(request.channel_id),
    queryFn: () => metisClient.historyListByChannel(request),
    enabled: request.channel_id.trim().length > 0,
  })

  useEffect(() => {
    if (query.data) {
      syncHistory(query.data)
    }
  }, [query.data])

  return query
}

export function useHistoryByBranchQuery(request: ListHistoryByBranchRequest) {
  const query = useQuery({
    queryKey: metisQueryKeys.historyByBranch(request.branch_id),
    queryFn: () => metisClient.historyListByBranch(request),
    enabled: request.branch_id.trim().length > 0,
  })

  useEffect(() => {
    if (query.data) {
      syncHistory(query.data)
    }
  }, [query.data])

  return query
}

export function useCreateChannelMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: CreateChannelRequest) => metisClient.channelsCreate(request),
    onSuccess: (channel: Channel) => {
      upsertChannel(channel)
      void queryClient.invalidateQueries({ queryKey: metisQueryKeys.channels() })
    },
  })
}

export function useUpdateChannelStatusMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: UpdateChannelStatusRequest) =>
      metisClient.channelsUpdateStatus(request),
    onSuccess: (channel: Channel) => {
      upsertChannel(channel)
      void queryClient.invalidateQueries({ queryKey: metisQueryKeys.channels() })
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.branchesByChannel(channel.id),
      })
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.tasksByChannel(channel.id),
      })
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.historyByChannel(channel.id),
      })
    },
  })
}

export function useEnqueueTaskMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: EnqueueTaskRequest) => metisClient.tasksEnqueue(request),
    onSuccess: (task: Task) => {
      upsertTask(task)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.tasksByChannel(task.channel_id),
      })
    },
  })
}

export function useUpdateTaskStateMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: UpdateTaskStateRequest) => metisClient.tasksUpdateState(request),
    onSuccess: (task: Task) => {
      upsertTask(task)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.tasksByChannel(task.channel_id),
      })
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.workersByTask(task.id),
      })
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.historyByChannel(task.channel_id),
      })
    },
  })
}

export function useCreateWorkerMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: CreateWorkerRequest) => metisClient.workersCreate(request),
    onSuccess: (worker: Worker) => {
      upsertWorker(worker)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.workersByTask(worker.task_id),
      })
    },
  })
}

export function useUpdateWorkerStateMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: UpdateWorkerStateRequest) =>
      metisClient.workersUpdateState(request),
    onSuccess: (worker: Worker) => {
      upsertWorker(worker)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.workersByTask(worker.task_id),
      })
    },
  })
}

export function useWorkerHeartbeatMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: WorkerHeartbeatRequest) =>
      metisClient.workersHeartbeat(request),
    onSuccess: (worker: Worker) => {
      upsertWorker(worker)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.workersByTask(worker.task_id),
      })
    },
  })
}

export function useAppendHistoryMutation() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (request: AppendHistoryRequest) => metisClient.historyAppend(request),
    onSuccess: (event: HistoryEvent) => {
      upsertHistory(event)
      void queryClient.invalidateQueries({
        queryKey: metisQueryKeys.historyByChannel(event.channel_id),
      })
      if (event.branch_id) {
        void queryClient.invalidateQueries({
          queryKey: metisQueryKeys.historyByBranch(event.branch_id),
        })
      }
    },
  })
}
