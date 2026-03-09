import { createFileRoute } from '@tanstack/react-router'
import { useLiveQuery } from '@tanstack/react-db'
import { useEffect, useMemo, useState } from 'react'

import {
  MetisInvokeError,
  channelsCollection,
  historyCollection,
  tasksCollection,
  useAppendHistoryMutation,
  useChannelsQuery,
  useCreateChannelMutation,
  useEnqueueTaskMutation,
  useHistoryByChannelQuery,
  useTasksByChannelQuery,
} from '@/lib/metis'

export const Route = createFileRoute('/')({ component: App })

function App() {
  const [title, setTitle] = useState('')
  const [selectedChannelId, setSelectedChannelId] = useState('')
  const [taskKind, setTaskKind] = useState('analysis')
  const [historyInput, setHistoryInput] = useState('')

  const channelsQuery = useChannelsQuery()
  const liveChannels = useLiveQuery(channelsCollection)
  const createChannelMutation = useCreateChannelMutation()
  const enqueueTaskMutation = useEnqueueTaskMutation()
  const appendHistoryMutation = useAppendHistoryMutation()

  useEffect(() => {
    if (!selectedChannelId && liveChannels.data && liveChannels.data.length > 0) {
      setSelectedChannelId(liveChannels.data[0].id)
    }
  }, [liveChannels.data, selectedChannelId])

  const selectedChannel = useMemo(
    () =>
      (liveChannels.data ?? []).find((channel) => channel.id === selectedChannelId) ?? null,
    [liveChannels.data, selectedChannelId],
  )

  const tasksQuery = useTasksByChannelQuery({ channel_id: selectedChannelId })
  const historyQuery = useHistoryByChannelQuery({ channel_id: selectedChannelId })

  const liveTasks = useLiveQuery(tasksCollection)
  const liveHistory = useLiveQuery(historyCollection)

  const selectedChannelTasks = useMemo(
    () =>
      (liveTasks.data ?? tasksQuery.data ?? []).filter(
        (task) => task.channel_id === selectedChannelId,
      ),
    [liveTasks.data, tasksQuery.data, selectedChannelId],
  )

  const selectedChannelHistory = useMemo(
    () =>
      (liveHistory.data ?? historyQuery.data ?? [])
        .filter((event) => event.channel_id === selectedChannelId)
        .slice()
        .sort((a, b) => a.created_at.localeCompare(b.created_at)),
    [liveHistory.data, historyQuery.data, selectedChannelId],
  )

  function onCreateChannel() {
    const normalizedTitle = title.trim()
    if (!normalizedTitle) {
      return
    }

    const now = new Date().toISOString()
    createChannelMutation.mutate({
      channel: {
        id: crypto.randomUUID(),
        title: normalizedTitle,
        source_type: 'Manual',
        source_ref: null,
        status: 'Active',
        created_at: now,
        updated_at: now,
      },
    })
    setTitle('')
  }

  function onEnqueueTask() {
    if (!selectedChannelId || !taskKind.trim()) {
      return
    }

    const now = new Date().toISOString()
    enqueueTaskMutation.mutate({
      task: {
        id: crypto.randomUUID(),
        channel_id: selectedChannelId,
        branch_id: null,
        kind: taskKind.trim(),
        state: 'Queued',
        priority: 0,
        payload_json: null,
        created_at: now,
        updated_at: now,
        started_at: null,
        finished_at: null,
      },
    })
  }

  function onAppendHistory() {
    if (!selectedChannelId || !historyInput.trim()) {
      return
    }

    appendHistoryMutation.mutate({
      event: {
        id: crypto.randomUUID(),
        channel_id: selectedChannelId,
        branch_id: null,
        task_id: null,
        worker_id: null,
        event_type: 'message',
        role: 'User',
        content_json: JSON.stringify({ text: historyInput.trim() }),
        correlation_id: crypto.randomUUID(),
        created_at: new Date().toISOString(),
      },
    })

    setHistoryInput('')
  }

  const channelError = channelsQuery.error
  const createError = createChannelMutation.error
  const taskError = tasksQuery.error ?? enqueueTaskMutation.error
  const historyError = historyQuery.error ?? appendHistoryMutation.error

  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-6 px-6 py-10">
      <h1 className="text-2xl font-semibold">Metis Channels</h1>

      <section className="flex gap-3">
        <input
          value={title}
          onChange={(event) => setTitle(event.target.value)}
          placeholder="New channel title"
          className="flex-1 rounded border border-slate-300 px-3 py-2"
        />
        <button
          type="button"
          onClick={onCreateChannel}
          disabled={createChannelMutation.isPending}
          className="rounded bg-slate-900 px-4 py-2 text-white disabled:opacity-60"
        >
          {createChannelMutation.isPending ? 'Creating...' : 'Create'}
        </button>
      </section>

      {renderError(channelError)}
      {renderError(createError)}

      {channelsQuery.isLoading && !liveChannels.data ? (
        <p>Loading channels...</p>
      ) : (
        <>
          <ul className="space-y-2">
            {(liveChannels.data ?? channelsQuery.data ?? []).map((channel) => (
              <li
                key={channel.id}
                className={`cursor-pointer rounded border px-3 py-2 ${
                  selectedChannelId === channel.id
                    ? 'border-slate-900 bg-slate-50'
                    : 'border-slate-200'
                }`}
                onClick={() => setSelectedChannelId(channel.id)}
              >
                <div className="font-medium">{channel.title}</div>
                <div className="text-sm text-slate-500">{channel.id}</div>
              </li>
            ))}
          </ul>

          <section className="mt-6 rounded border border-slate-200 p-4">
            <h2 className="text-lg font-semibold">Tasks</h2>
            <p className="mt-1 text-sm text-slate-500">
              {selectedChannel
                ? `Selected channel: ${selectedChannel.title}`
                : 'Select a channel to view tasks'}
            </p>

            <div className="mt-3 flex gap-3">
              <input
                value={taskKind}
                onChange={(event) => setTaskKind(event.target.value)}
                placeholder="Task kind"
                className="flex-1 rounded border border-slate-300 px-3 py-2"
              />
              <button
                type="button"
                onClick={onEnqueueTask}
                disabled={enqueueTaskMutation.isPending || !selectedChannel}
                className="rounded bg-slate-900 px-4 py-2 text-white disabled:opacity-60"
              >
                {enqueueTaskMutation.isPending ? 'Queueing...' : 'Enqueue Task'}
              </button>
            </div>

            {renderError(taskError)}
            {tasksQuery.isLoading && selectedChannelTasks.length === 0 ? (
              <p className="mt-3 text-sm">Loading tasks...</p>
            ) : (
              <ul className="mt-3 space-y-2">
                {selectedChannelTasks.map((task) => (
                  <li key={task.id} className="rounded border border-slate-200 px-3 py-2">
                    <div className="font-medium">{task.kind}</div>
                    <div className="text-sm text-slate-500">{task.state}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>

          <section className="mt-6 rounded border border-slate-200 p-4">
            <h2 className="text-lg font-semibold">History</h2>
            <div className="mt-3 flex gap-3">
              <input
                value={historyInput}
                onChange={(event) => setHistoryInput(event.target.value)}
                placeholder="Write a history message"
                className="flex-1 rounded border border-slate-300 px-3 py-2"
              />
              <button
                type="button"
                onClick={onAppendHistory}
                disabled={appendHistoryMutation.isPending || !selectedChannel}
                className="rounded bg-slate-900 px-4 py-2 text-white disabled:opacity-60"
              >
                {appendHistoryMutation.isPending ? 'Appending...' : 'Append'}
              </button>
            </div>

            {renderError(historyError)}
            {historyQuery.isLoading && selectedChannelHistory.length === 0 ? (
              <p className="mt-3 text-sm">Loading history...</p>
            ) : (
              <ul className="mt-3 space-y-2">
                {selectedChannelHistory.map((event) => (
                  <li key={event.id} className="rounded border border-slate-200 px-3 py-2">
                    <div className="text-sm font-medium">{event.event_type}</div>
                    <div className="text-xs text-slate-500">{event.content_json}</div>
                  </li>
                ))}
              </ul>
            )}
          </section>
        </>
      )}
    </main>
  )
}

function renderError(error: unknown) {
  if (!error) {
    return null
  }

  if (error instanceof MetisInvokeError) {
    return (
      <p className="rounded border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
        {error.code}: {error.message}
      </p>
    )
  }

  return (
    <p className="rounded border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
      {String(error)}
    </p>
  )
}
