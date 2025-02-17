/* tslint:disable */
/* eslint-disable */
export function main_js(): void;
export enum Key {
  Start = 0,
  Select = 1,
  B = 2,
  A = 3,
  Down = 4,
  Up = 5,
  Left = 6,
  Right = 7,
}
export class JimbotWeb {
  free(): void;
  constructor(cartridge_bytes: Uint8Array);
  run(lcd_data: Uint8Array): void;
  joypad_release(key: Key): void;
  joypad_press(key: Key): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_jimbotweb_free: (a: number, b: number) => void;
  readonly main_js: () => void;
  readonly jimbotweb_new: (a: number, b: number) => number;
  readonly jimbotweb_run: (a: number, b: number, c: number, d: any) => void;
  readonly jimbotweb_joypad_release: (a: number, b: number) => void;
  readonly jimbotweb_joypad_press: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__headd2c84f1a66374: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
