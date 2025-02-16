import { DeviceInfo } from './types'
import { tauriInvoke, isTauri } from './utils'

export const ping = async (value: string): Promise<string | null> =>
  await tauriInvoke<{
    value?: string
  }>('ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null))

export const getAvailability = async (): Promise<boolean> => {
  if (isTauri()) {
    return await tauriInvoke<boolean>('get_availability')
  } else {
    return (await navigator.bluetooth?.getAvailability()) ?? false
  }
}

export const requestDevice = async (
  options: RequestDeviceOptions,
) => {
  if (!(await getAvailability())) {
    return
  }

  if (isTauri()) {
    const info = await tauriInvoke<DeviceInfo>('request_device', { options })
    console.log(info)
  } else {
    const device = await navigator.bluetooth.requestDevice(options)
    console.log(device)
  }
}
