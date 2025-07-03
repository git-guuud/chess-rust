/* tslint:disable */
/* eslint-disable */
export function set_state(fen: string): void;
export function get_state(): any;
export function get_state_fen(): string;
export function make_move(from: number, to: number): void;
export function get_valid_moves(from: number): Uint32Array;
export function in_check(): boolean;
export function change_promotion(tp: string): void;
export function _eval(): number;
export function engine_move(): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_state: (a: number, b: number) => [number, number];
  readonly get_state: () => any;
  readonly get_state_fen: () => [number, number];
  readonly make_move: (a: number, b: number) => void;
  readonly get_valid_moves: (a: number) => [number, number];
  readonly in_check: () => number;
  readonly change_promotion: (a: number, b: number) => void;
  readonly _eval: () => number;
  readonly engine_move: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
