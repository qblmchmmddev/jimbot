/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_js(): void;
/**
*/
export enum Key {
  Start,
  Select,
  B,
  A,
  Down,
  Up,
  Left,
  Right,
}
/**
*/
export class JimbotWeb {
  free(): void;
/**
* @param {Uint8Array} cartridge_bytes
*/
  constructor(cartridge_bytes: Uint8Array);
/**
* @param {Uint8Array} lcd_data
*/
  run(lcd_data: Uint8Array): void;
/**
* @param {number} key
*/
  joypad_release(key: number): void;
/**
* @param {number} key
*/
  joypad_press(key: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_jimbotweb_free: (a: number) => void;
  readonly main_js: () => void;
  readonly jimbotweb_new: (a: number, b: number) => number;
  readonly jimbotweb_run: (a: number, b: number, c: number) => void;
  readonly jimbotweb_joypad_release: (a: number, b: number) => void;
  readonly jimbotweb_joypad_press: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he8e1718a2d73f573: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
