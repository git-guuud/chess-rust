/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const set_state: (a: number, b: number) => [number, number];
export const get_state: () => any;
export const get_state_fen: () => [number, number];
export const make_move: (a: number, b: number) => void;
export const get_valid_moves: (a: number) => [number, number];
export const in_check: () => number;
export const change_promotion: (a: number, b: number) => void;
export const _eval: () => number;
export const engine_move: () => number;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_export_2: WebAssembly.Table;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
