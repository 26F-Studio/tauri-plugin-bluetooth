import { invoke } from '@tauri-apps/api/core'

import { INVOKE_PREFIX } from './constants'

declare global {
  interface Window {
    isTauri?: boolean | undefined
  }
}

export const tauriInvoke = async <T>(command: string, args?: any): Promise<T> => {
  return await invoke<T>(INVOKE_PREFIX + command, args)
}

export const isTauri = (): boolean => window?.isTauri ?? false
