import { invoke } from '@tauri-apps/api/core'

import { INVOKE_PREFIX } from './constants'

export const tauriInvoke = async <T>(command: string, args?: any): Promise<T> => {
  return await invoke<T>(INVOKE_PREFIX + command, args)
}
