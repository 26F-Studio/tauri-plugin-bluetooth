import { invoke } from '@tauri-apps/api/core'

declare global {
  interface Window {
    isTauri?: boolean | undefined
  }
}

export const ping = async (value: string): Promise<string | null> =>
  await invoke<{
    value?: string
  }>('plugin:bluetooth|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null))

export const getAvailability = async (): Promise<boolean> => {
  if (window.isTauri) {
    return await invoke<boolean>('plugin:bluetooth|getAvailability')
  } else {
    return (await navigator.bluetooth?.getAvailability()) ?? false
  }
}
