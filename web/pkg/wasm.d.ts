/* tslint:disable */
/* eslint-disable */
export class EmuWasm {
  free(): void;
  constructor();
  reset(): void;
  tick(): void;
  tick_timers(): void;
  keypress(evt: KeyboardEvent, pressed: boolean): void;
  virtual_keypress(key_str: string, pressed: boolean): void;
  load_game(data: Uint8Array): void;
  draw_screen(scale: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emuwasm_free: (a: number, b: number) => void;
  readonly emuwasm_new: () => [number, number, number];
  readonly emuwasm_reset: (a: number) => void;
  readonly emuwasm_tick: (a: number) => void;
  readonly emuwasm_tick_timers: (a: number) => void;
  readonly emuwasm_keypress: (a: number, b: any, c: number) => void;
  readonly emuwasm_virtual_keypress: (a: number, b: number, c: number, d: number) => void;
  readonly emuwasm_load_game: (a: number, b: any) => void;
  readonly emuwasm_draw_screen: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
