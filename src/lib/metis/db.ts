import { createCollection } from '@tanstack/db'

import type { Branch, Channel, HistoryEvent, Task, Worker } from './types'

export const channelsCollection = createCollection<Channel, string>({
  id: 'channels',
  getKey: (channel) => channel.id,
  sync: {
    sync: ({ markReady }) => {
      markReady()
    },
  },
})

export const branchesCollection = createCollection<Branch, string>({
  id: 'branches',
  getKey: (branch) => branch.id,
  sync: {
    sync: ({ markReady }) => {
      markReady()
    },
  },
})

export const tasksCollection = createCollection<Task, string>({
  id: 'tasks',
  getKey: (task) => task.id,
  sync: {
    sync: ({ markReady }) => {
      markReady()
    },
  },
})

export const workersCollection = createCollection<Worker, string>({
  id: 'workers',
  getKey: (worker) => worker.id,
  sync: {
    sync: ({ markReady }) => {
      markReady()
    },
  },
})

export const historyCollection = createCollection<HistoryEvent, string>({
  id: 'history',
  getKey: (event) => event.id,
  sync: {
    sync: ({ markReady }) => {
      markReady()
    },
  },
})

export function syncChannels(records: Channel[]): void {
  syncCollectionRecords(channelsCollection, records, (record) => record.id)
}

export function syncBranches(records: Branch[]): void {
  syncCollectionRecords(branchesCollection, records, (record) => record.id)
}

export function syncTasks(records: Task[]): void {
  syncCollectionRecords(tasksCollection, records, (record) => record.id)
}

export function syncWorkers(records: Worker[]): void {
  syncCollectionRecords(workersCollection, records, (record) => record.id)
}

export function syncHistory(records: HistoryEvent[]): void {
  syncCollectionRecords(historyCollection, records, (record) => record.id)
}

export function upsertChannel(record: Channel): void {
  upsertRecord(channelsCollection, record, record.id)
}

export function upsertTask(record: Task): void {
  upsertRecord(tasksCollection, record, record.id)
}

export function upsertWorker(record: Worker): void {
  upsertRecord(workersCollection, record, record.id)
}

export function upsertHistory(record: HistoryEvent): void {
  upsertRecord(historyCollection, record, record.id)
}

function upsertRecord<T extends object>(
  collection: {
    has: (key: string) => boolean
    insert: (data: T) => unknown
    update: (key: string, callback: (draft: T) => void) => unknown
  },
  record: T,
  key: string,
): void {
  if (collection.has(key)) {
    collection.update(key, (draft) => {
      Object.assign(draft, record)
    })
    return
  }

  collection.insert(record)
}

function syncCollectionRecords<T extends object>(
  collection: {
    state: Map<string, T>
    has: (key: string) => boolean
    insert: (data: T[]) => unknown
    update: (key: string, callback: (draft: T) => void) => unknown
    delete: (keys: string[]) => unknown
  },
  records: T[],
  getKey: (record: T) => string,
): void {
  const existingKeys = new Set(collection.state.keys())

  for (const record of records) {
    const key = getKey(record)
    if (collection.has(key)) {
      collection.update(key, (draft) => {
        Object.assign(draft, record)
      })
    } else {
      collection.insert([record])
    }
    existingKeys.delete(key)
  }

  if (existingKeys.size > 0) {
    collection.delete(Array.from(existingKeys))
  }
}
