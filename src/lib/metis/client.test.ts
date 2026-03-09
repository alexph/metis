import { beforeEach, describe, expect, it, vi } from 'vitest'

import { metisClient } from './client'

const invokeMock = vi.fn()

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}))

describe('metisClient', () => {
  beforeEach(() => {
    invokeMock.mockReset()
  })

  it('returns typed data when command response is ok', async () => {
    invokeMock.mockResolvedValue({
      status: 'ok',
      data: [{ id: 'channel-1', title: 'One' }],
    })

    const channels = await metisClient.channelsList()

    expect(channels).toEqual([{ id: 'channel-1', title: 'One' }])
    expect(invokeMock).toHaveBeenCalledWith('desktop_channels_list')
  })

  it('throws MetisInvokeError for envelope error responses', async () => {
    invokeMock.mockResolvedValue({
      status: 'err',
      error: {
        code: 'service_validation_error',
        message: 'invalid input',
        details: 'title required',
      },
    })

    await expect(metisClient.channelsList()).rejects.toMatchObject({
      name: 'MetisInvokeError',
      code: 'service_validation_error',
      message: 'invalid input',
      details: 'title required',
    })
  })

  it('wraps transport failures into desktop_invoke_error', async () => {
    invokeMock.mockRejectedValue(new Error('ipc unavailable'))

    await expect(metisClient.channelsList()).rejects.toMatchObject({
      name: 'MetisInvokeError',
      code: 'desktop_invoke_error',
      message: 'failed to invoke desktop command',
      details: 'ipc unavailable',
    })
  })

  it('sends request payload to invoke for mutation commands', async () => {
    const request = {
      channel: {
        id: 'channel-2',
        title: 'New',
        source_type: 'Manual' as const,
        source_ref: null,
        status: 'Active' as const,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      },
    }

    invokeMock.mockResolvedValue({ status: 'ok', data: request.channel })

    await metisClient.channelsCreate(request)

    expect(invokeMock).toHaveBeenCalledWith('desktop_channels_create', {
      request,
    })
  })
})
