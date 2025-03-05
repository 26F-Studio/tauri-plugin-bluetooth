import { isTauri } from '@tauri-apps/api/core'

import { tauriInvoke } from './utils'

type WebBluetoothDevice = globalThis.BluetoothDevice

type WebBluetoothRemoteGATTServer = globalThis.BluetoothRemoteGATTServer

type WebBluetoothRemoteGATTService = globalThis.BluetoothRemoteGATTService

export interface DeviceInfo {
  id: string
  name?: string | undefined
  services: string[]
}

export interface RequestDeviceTauriOptions {
  timeout?: number
}

export class BluetoothRemoteGATTServer implements WebBluetoothRemoteGATTServer {
  private readonly _device: BluetoothDeviceTauri
  private _connected = false

  constructor(device: BluetoothDeviceTauri) {
    this._device = device
  }

  get connected(): boolean {
    return this._connected
  }

  get device(): BluetoothDevice {
    return this._device as unknown as BluetoothDevice
  }

  async connect(): Promise<BluetoothRemoteGATTServer> {
    if (await tauriInvoke<boolean>('gatt_connect', { deviceId: this._device.id })) {
      this._connected = true
    }
    return this
  }

  disconnect(): void {
    if (this._connected) {
      tauriInvoke('gatt_disconnect', { deviceId: this._device.id }).then(() => {
        this._connected = false
      })
    }
  }

  async getPrimaryService(service: BluetoothServiceUUID): Promise<BluetoothRemoteGATTService> {
    const uuid = await tauriInvoke<string>('get_service', {
      service,
    })
    return new BluetoothRemoteGATTServiceTauri(this.device, uuid)
  }

  async getPrimaryServices(service?: BluetoothServiceUUID): Promise<BluetoothRemoteGATTService[]> {
    throw new Error('Method not implemented.')
  }
}

class BluetoothRemoteGATTService implements WebBluetoothRemoteGATTService {
  constructor(
    public readonly device: BluetoothDevice,
    public readonly uuid: string,
  ) {}

  isPrimary: boolean = false

  getCharacteristic(
    characteristic: BluetoothCharacteristicUUID,
  ): Promise<BluetoothRemoteGATTCharacteristic> {
    throw new Error('Method not implemented.')
  }

  getCharacteristics(
    characteristic?: BluetoothCharacteristicUUID,
  ): Promise<BluetoothRemoteGATTCharacteristic[]> {
    throw new Error('Method not implemented.')
  }

  getIncludedService(service: BluetoothServiceUUID): Promise<BluetoothRemoteGATTService> {
    throw new Error('Method not implemented.')
  }

  getIncludedServices(service?: BluetoothServiceUUID): Promise<BluetoothRemoteGATTService[]> {
    throw new Error('Method not implemented.')
  }

  addEventListener(type: unknown, listener: unknown, useCapture?: unknown): void {
    throw new Error('Method not implemented.')
  }

  dispatchEvent(event: Event): boolean {
    throw new Error('Method not implemented.')
  }

  removeEventListener(
    type: string,
    callback: EventListenerOrEventListenerObject | null,
    options?: EventListenerOptions | boolean,
  ): void {
    throw new Error('Method not implemented.')
  }

  oncharacteristicvaluechanged: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  onserviceadded: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  onservicechanged: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  onserviceremoved: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
}

interface BluetoothDeviceTauri {
  id: string
  name?: string | undefined
}

export class BluetoothDevice {
  private readonly _device: WebBluetoothDevice | BluetoothDeviceTauri
  private _watchingAdvertisements = false

  constructor(id: string, name?: string | undefined)
  constructor(device: WebBluetoothDevice)
  constructor(idOrDevice: string | WebBluetoothDevice, name?: string) {
    if (isTauri()) {
      if (typeof idOrDevice !== 'string') {
        throw new TypeError(
          'BluetoothDeviceWrapper should pass "id" and "name" as arguments in Tauri environment',
        )
      }
      this._device = { id: idOrDevice, name }
      this.gatt = new BluetoothRemoteGATTServer(this._device)
    } else {
      if (typeof idOrDevice === 'string') {
        throw new TypeError(
          'BluetoothDeviceWrapper should pass "device" as argument in browser environment',
        )
      }
      this._device = idOrDevice
      if (idOrDevice.gatt) {
        this.gatt = new BluetoothRemoteGATTServer(idOrDevice)
      }
    }
  }

  readonly gatt?: BluetoothRemoteGATTServer | undefined

  get id(): string {
    return this._device.id
  }

  get name(): string | undefined {
    return this._device.name
  }

  async forget(): Promise<void> {
    if (isTauri()) {
      await tauriInvoke('device_forget', { deviceId: this.id })
    } else {
      await (<WebBluetoothDevice>this._device).forget()
    }
  }

  async watchAdvertisements(options?: WatchAdvertisementsOptions): Promise<void> {
    if (this._watchingAdvertisements) {
      return
    }

    if (isTauri()) {
      await tauriInvoke('watch_ads', {
        deviceId: this.id,
        signal: options?.signal,
      })
    } else {
      await (<WebBluetoothDevice>this._device).watchAdvertisements(options)
    }
  }

  get watchingAdvertisements(): boolean {
    return this._watchingAdvertisements
  }

  addEventListener(type: unknown, listener: unknown, useCapture?: unknown): void {
    throw new Error('Method not implemented.')
  }

  dispatchEvent(event: Event): boolean {
    throw new Error('Method not implemented.')
  }

  removeEventListener(
    type: string,
    callback: EventListenerOrEventListenerObject | null,
    options?: EventListenerOptions | boolean,
  ): void {
    throw new Error('Method not implemented.')
  }

  readonly onadvertisementreceived: (this: this, ev: BluetoothAdvertisingEvent) => any = () => {
    throw new Error('Method not implemented.')
  }
  ongattserverdisconnected: (this: this, ev: Event) => any = () => {}
  readonly oncharacteristicvaluechanged: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  readonly onserviceadded: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  readonly onservicechanged: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
  readonly onserviceremoved: (this: this, ev: Event) => any = () => {
    throw new Error('Method not implemented.')
  }
}
