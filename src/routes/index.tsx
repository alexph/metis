import { createFileRoute } from '@tanstack/react-router'
import { useLiveQuery } from '@tanstack/react-db'
import { useState } from 'react'

import {
  MetisInvokeError,
  channelsCollection,
  useChannelsQuery,
  useCreateChannelMutation,
} from '@/lib/metis'

export const Route = createFileRoute('/')({ component: App })

function App() {
  const [title, setTitle] = useState('')

  const channelsQuery = useChannelsQuery()
  const liveChannels = useLiveQuery(() => channelsCollection)
  const createChannelMutation = useCreateChannelMutation()

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

  const channelError = channelsQuery.error
  const createError = createChannelMutation.error

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
        <ul className="space-y-2">
          {(liveChannels.data ?? channelsQuery.data ?? []).map((channel) => (
            <li key={channel.id} className="rounded border border-slate-200 px-3 py-2">
              <div className="font-medium">{channel.title}</div>
              <div className="text-sm text-slate-500">{channel.id}</div>
            </li>
          ))}
        </ul>
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
