/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} ssl_in
* @param {Uint8Array} ssl_out
* @param {Uint8Array} pair
* @param {OracleRegistry} oracles
* @param {BigInt} amount_in
* @param {BigInt} min_out
* @returns {SwapResult}
*/
export function swap(ssl_in: Uint8Array, ssl_out: Uint8Array, pair: Uint8Array, oracles: OracleRegistry, amount_in: BigInt, min_out: BigInt): SwapResult;
/**
*/
export class OracleRegistry {
  free(): void;
/**
*/
  constructor();
/**
* @param {Uint8Array} key
* @param {Uint8Array} data
*/
  add_oracle(key: Uint8Array, data: Uint8Array): void;
}
/**
*/
export class SwapResult {
  free(): void;
/**
*/
  out: BigInt;
/**
*/
  price_impact: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_oracleregistry_free: (a: number) => void;
  readonly oracleregistry_new: () => number;
  readonly oracleregistry_add_oracle: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_swapresult_free: (a: number) => void;
  readonly __wbg_get_swapresult_out: (a: number, b: number) => void;
  readonly __wbg_set_swapresult_out: (a: number, b: number, c: number) => void;
  readonly __wbg_get_swapresult_price_impact: (a: number) => number;
  readonly __wbg_set_swapresult_price_impact: (a: number, b: number) => void;
  readonly swap: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path: InitInput | Promise<InitInput>): Promise<InitOutput>;
