/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} ssl_in
* @param {Uint8Array} ssl_out
* @param {Uint8Array} pair
* @param {BigInt} liability_in
* @param {BigInt} liability_out
* @param {BigInt} swapped_liability_in
* @param {BigInt} swapped_liability_out
* @param {OracleRegistry} oracles
* @param {BigInt} amount_in
* @returns {SwapResult}
*/
export function swap(ssl_in: Uint8Array, ssl_out: Uint8Array, pair: Uint8Array, liability_in: BigInt, liability_out: BigInt, swapped_liability_in: BigInt, swapped_liability_out: BigInt, oracles: OracleRegistry, amount_in: BigInt): SwapResult;
/**
* Initialize Javascript logging and panic handler
*/
export function solana_program_init(): void;
/**
*/
export class Hash {
  free(): void;
/**
* Create a new Hash object
*
* * `value` - optional hash as a base58 encoded string, `Uint8Array`, `[number]`
* @param {any} value
*/
  constructor(value: any);
/**
* Return the base58 string representation of the hash
* @returns {string}
*/
  toString(): string;
/**
* Checks if two `Hash`s are equal
* @param {Hash} other
* @returns {boolean}
*/
  equals(other: Hash): boolean;
/**
* Return the `Uint8Array` representation of the hash
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
}
/**
* A directive for a single invocation of a Solana program.
*
* An instruction specifies which program it is calling, which accounts it may
* read or modify, and additional data that serves as input to the program. One
* or more instructions are included in transactions submitted by Solana
* clients. Instructions are also used to describe [cross-program
* invocations][cpi].
*
* [cpi]: https://docs.solana.com/developing/programming-model/calling-between-programs
*
* During execution, a program will receive a list of account data as one of
* its arguments, in the same order as specified during `Instruction`
* construction.
*
* While Solana is agnostic to the format of the instruction data, it has
* built-in support for serialization via [`borsh`] and [`bincode`].
*
* [`borsh`]: https://docs.rs/borsh/latest/borsh/
* [`bincode`]: https://docs.rs/bincode/latest/bincode/
*
* # Specifying account metadata
*
* When constructing an [`Instruction`], a list of all accounts that may be
* read or written during the execution of that instruction must be supplied as
* [`AccountMeta`] values.
*
* Any account whose data may be mutated by the program during execution must
* be specified as writable. During execution, writing to an account that was
* not specified as writable will cause the transaction to fail. Writing to an
* account that is not owned by the program will cause the transaction to fail.
*
* Any account whose lamport balance may be mutated by the program during
* execution must be specified as writable. During execution, mutating the
* lamports of an account that was not specified as writable will cause the
* transaction to fail. While _subtracting_ lamports from an account not owned
* by the program will cause the transaction to fail, _adding_ lamports to any
* account is allowed, as long is it is mutable.
*
* Accounts that are not read or written by the program may still be specified
* in an `Instruction`'s account list. These will affect scheduling of program
* execution by the runtime, but will otherwise be ignored.
*
* When building a transaction, the Solana runtime coalesces all accounts used
* by all instructions in that transaction, along with accounts and permissions
* required by the runtime, into a single account list. Some accounts and
* account permissions required by the runtime to process a transaction are
* _not_ required to be included in an `Instruction`s account list. These
* include:
*
* - The program ID &mdash; it is a separate field of `Instruction`
* - The transaction's fee-paying account &mdash; it is added during [`Message`]
*   construction. A program may still require the fee payer as part of the
*   account list if it directly references it.
*
* [`Message`]: crate::message::Message
*
* Programs may require signatures from some accounts, in which case they
* should be specified as signers during `Instruction` construction. The
* program must still validate during execution that the account is a signer.
*/
export class Instruction {
  free(): void;
}
/**
*/
export class Instructions {
  free(): void;
/**
*/
  constructor();
/**
* @param {Instruction} instruction
*/
  push(instruction: Instruction): void;
}
/**
* A vanilla Ed25519 key pair
*/
export class Keypair {
  free(): void;
/**
* Create a new `Keypair `
*/
  constructor();
/**
* Convert a `Keypair` to a `Uint8Array`
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* Recover a `Keypair` from a `Uint8Array`
* @param {Uint8Array} bytes
* @returns {Keypair}
*/
  static fromBytes(bytes: Uint8Array): Keypair;
/**
* Return the `Pubkey` for this `Keypair`
* @returns {Pubkey}
*/
  pubkey(): Pubkey;
}
/**
* A Solana transaction message (legacy).
*
* See the [`message`] module documentation for further description.
*
* [`message`]: crate::message
*
* Some constructors accept an optional `payer`, the account responsible for
* paying the cost of executing a transaction. In most cases, callers should
* specify the payer explicitly in these constructors. In some cases though,
* the caller is not _required_ to specify the payer, but is still allowed to:
* in the `Message` structure, the first account is always the fee-payer, so if
* the caller has knowledge that the first account of the constructed
* transaction's `Message` is both a signer and the expected fee-payer, then
* redundantly specifying the fee-payer is not strictly required.
*/
export class Message {
  free(): void;
/**
* The id of a recent ledger entry.
*/
  recent_blockhash: Hash;
}
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
export class Pubkey {
  free(): void;
/**
* Create a new Pubkey object
*
* * `value` - optional public key as a base58 encoded string, `Uint8Array`, `[number]`
* @param {any} value
*/
  constructor(value: any);
/**
* Return the base58 string representation of the public key
* @returns {string}
*/
  toString(): string;
/**
* Check if a `Pubkey` is on the ed25519 curve.
* @returns {boolean}
*/
  isOnCurve(): boolean;
/**
* Checks if two `Pubkey`s are equal
* @param {Pubkey} other
* @returns {boolean}
*/
  equals(other: Pubkey): boolean;
/**
* Return the `Uint8Array` representation of the public key
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* Derive a Pubkey from another Pubkey, string seed, and a program id
* @param {Pubkey} base
* @param {string} seed
* @param {Pubkey} owner
* @returns {Pubkey}
*/
  static createWithSeed(base: Pubkey, seed: string, owner: Pubkey): Pubkey;
/**
* Derive a program address from seeds and a program id
* @param {any[]} seeds
* @param {Pubkey} program_id
* @returns {Pubkey}
*/
  static createProgramAddress(seeds: any[], program_id: Pubkey): Pubkey;
/**
* Find a valid program address
*
* Returns:
* * `[PubKey, number]` - the program address and bump seed
* @param {any[]} seeds
* @param {Pubkey} program_id
* @returns {any}
*/
  static findProgramAddress(seeds: any[], program_id: Pubkey): any;
}
/**
*/
export class SwapResult {
  free(): void;
/**
*/
  amount_in: BigInt;
/**
*/
  amount_out: BigInt;
/**
*/
  fee_paid: BigInt;
/**
*/
  insta_price: number;
/**
*/
  oracle_price: number;
/**
*/
  price_impact: number;
/**
*/
  swap_price: number;
}
export class SystemInstruction {
  free(): void;
/**
* @param {Pubkey} from_pubkey
* @param {Pubkey} to_pubkey
* @param {BigInt} lamports
* @param {BigInt} space
* @param {Pubkey} owner
* @returns {Instruction}
*/
  static createAccount(from_pubkey: Pubkey, to_pubkey: Pubkey, lamports: BigInt, space: BigInt, owner: Pubkey): Instruction;
/**
* @param {Pubkey} from_pubkey
* @param {Pubkey} to_pubkey
* @param {Pubkey} base
* @param {string} seed
* @param {BigInt} lamports
* @param {BigInt} space
* @param {Pubkey} owner
* @returns {Instruction}
*/
  static createAccountWithSeed(from_pubkey: Pubkey, to_pubkey: Pubkey, base: Pubkey, seed: string, lamports: BigInt, space: BigInt, owner: Pubkey): Instruction;
/**
* @param {Pubkey} pubkey
* @param {Pubkey} owner
* @returns {Instruction}
*/
  static assign(pubkey: Pubkey, owner: Pubkey): Instruction;
/**
* @param {Pubkey} pubkey
* @param {Pubkey} base
* @param {string} seed
* @param {Pubkey} owner
* @returns {Instruction}
*/
  static assignWithSeed(pubkey: Pubkey, base: Pubkey, seed: string, owner: Pubkey): Instruction;
/**
* @param {Pubkey} from_pubkey
* @param {Pubkey} to_pubkey
* @param {BigInt} lamports
* @returns {Instruction}
*/
  static transfer(from_pubkey: Pubkey, to_pubkey: Pubkey, lamports: BigInt): Instruction;
/**
* @param {Pubkey} from_pubkey
* @param {Pubkey} from_base
* @param {string} from_seed
* @param {Pubkey} from_owner
* @param {Pubkey} to_pubkey
* @param {BigInt} lamports
* @returns {Instruction}
*/
  static transferWithSeed(from_pubkey: Pubkey, from_base: Pubkey, from_seed: string, from_owner: Pubkey, to_pubkey: Pubkey, lamports: BigInt): Instruction;
/**
* @param {Pubkey} pubkey
* @param {BigInt} space
* @returns {Instruction}
*/
  static allocate(pubkey: Pubkey, space: BigInt): Instruction;
/**
* @param {Pubkey} address
* @param {Pubkey} base
* @param {string} seed
* @param {BigInt} space
* @param {Pubkey} owner
* @returns {Instruction}
*/
  static allocateWithSeed(address: Pubkey, base: Pubkey, seed: string, space: BigInt, owner: Pubkey): Instruction;
/**
* @param {Pubkey} from_pubkey
* @param {Pubkey} nonce_pubkey
* @param {Pubkey} authority
* @param {BigInt} lamports
* @returns {Array<any>}
*/
  static createNonceAccount(from_pubkey: Pubkey, nonce_pubkey: Pubkey, authority: Pubkey, lamports: BigInt): Array<any>;
/**
* @param {Pubkey} nonce_pubkey
* @param {Pubkey} authorized_pubkey
* @returns {Instruction}
*/
  static advanceNonceAccount(nonce_pubkey: Pubkey, authorized_pubkey: Pubkey): Instruction;
/**
* @param {Pubkey} nonce_pubkey
* @param {Pubkey} authorized_pubkey
* @param {Pubkey} to_pubkey
* @param {BigInt} lamports
* @returns {Instruction}
*/
  static withdrawNonceAccount(nonce_pubkey: Pubkey, authorized_pubkey: Pubkey, to_pubkey: Pubkey, lamports: BigInt): Instruction;
/**
* @param {Pubkey} nonce_pubkey
* @param {Pubkey} authorized_pubkey
* @param {Pubkey} new_authority
* @returns {Instruction}
*/
  static authorizeNonceAccount(nonce_pubkey: Pubkey, authorized_pubkey: Pubkey, new_authority: Pubkey): Instruction;
}
/**
* An atomically-commited sequence of instructions.
*
* While [`Instruction`]s are the basic unit of computation in Solana,
* they are submitted by clients in [`Transaction`]s containing one or
* more instructions, and signed by one or more [`Signer`]s.
*
* [`Signer`]: crate::signer::Signer
*
* See the [module documentation] for more details about transactions.
*
* [module documentation]: self
*
* Some constructors accept an optional `payer`, the account responsible for
* paying the cost of executing a transaction. In most cases, callers should
* specify the payer explicitly in these constructors. In some cases though,
* the caller is not _required_ to specify the payer, but is still allowed to:
* in the [`Message`] structure, the first account is always the fee-payer, so
* if the caller has knowledge that the first account of the constructed
* transaction's `Message` is both a signer and the expected fee-payer, then
* redundantly specifying the fee-payer is not strictly required.
*/
export class Transaction {
  free(): void;
/**
* Create a new `Transaction`
* @param {Instructions} instructions
* @param {Pubkey | undefined} payer
*/
  constructor(instructions: Instructions, payer?: Pubkey);
/**
* Return a message containing all data that should be signed.
* @returns {Message}
*/
  message(): Message;
/**
* Return the serialized message data to sign.
* @returns {Uint8Array}
*/
  messageData(): Uint8Array;
/**
* Verify the transaction
*/
  verify(): void;
/**
* @param {Keypair} keypair
* @param {Hash} recent_blockhash
*/
  partialSign(keypair: Keypair, recent_blockhash: Hash): void;
/**
* @returns {boolean}
*/
  isSigned(): boolean;
/**
* @returns {Uint8Array}
*/
  toBytes(): Uint8Array;
/**
* @param {Uint8Array} bytes
* @returns {Transaction}
*/
  static fromBytes(bytes: Uint8Array): Transaction;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_oracleregistry_free: (a: number) => void;
  readonly oracleregistry_new: () => number;
  readonly oracleregistry_add_oracle: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_swapresult_free: (a: number) => void;
  readonly __wbg_get_swapresult_amount_in: (a: number, b: number) => void;
  readonly __wbg_set_swapresult_amount_in: (a: number, b: number, c: number) => void;
  readonly __wbg_get_swapresult_fee_paid: (a: number, b: number) => void;
  readonly __wbg_set_swapresult_fee_paid: (a: number, b: number, c: number) => void;
  readonly __wbg_get_swapresult_amount_out: (a: number, b: number) => void;
  readonly __wbg_set_swapresult_amount_out: (a: number, b: number, c: number) => void;
  readonly __wbg_get_swapresult_price_impact: (a: number) => number;
  readonly __wbg_set_swapresult_price_impact: (a: number, b: number) => void;
  readonly __wbg_get_swapresult_swap_price: (a: number) => number;
  readonly __wbg_set_swapresult_swap_price: (a: number, b: number) => void;
  readonly __wbg_get_swapresult_insta_price: (a: number) => number;
  readonly __wbg_set_swapresult_insta_price: (a: number, b: number) => void;
  readonly __wbg_get_swapresult_oracle_price: (a: number) => number;
  readonly __wbg_set_swapresult_oracle_price: (a: number, b: number) => void;
  readonly swap: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number) => void;
  readonly __wbg_transaction_free: (a: number) => void;
  readonly transaction_constructor: (a: number, b: number) => number;
  readonly transaction_message: (a: number) => number;
  readonly transaction_messageData: (a: number, b: number) => void;
  readonly transaction_verify: (a: number, b: number) => void;
  readonly transaction_partialSign: (a: number, b: number, c: number) => void;
  readonly transaction_isSigned: (a: number) => number;
  readonly transaction_toBytes: (a: number, b: number) => void;
  readonly transaction_fromBytes: (a: number, b: number, c: number) => void;
  readonly __wbg_keypair_free: (a: number) => void;
  readonly keypair_constructor: () => number;
  readonly keypair_toBytes: (a: number, b: number) => void;
  readonly keypair_fromBytes: (a: number, b: number, c: number) => void;
  readonly keypair_pubkey: (a: number) => number;
  readonly __wbg_instruction_free: (a: number) => void;
  readonly systeminstruction_createAccount: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly systeminstruction_createAccountWithSeed: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly systeminstruction_assign: (a: number, b: number) => number;
  readonly systeminstruction_assignWithSeed: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly systeminstruction_transfer: (a: number, b: number, c: number, d: number) => number;
  readonly systeminstruction_transferWithSeed: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly systeminstruction_allocate: (a: number, b: number, c: number) => number;
  readonly systeminstruction_allocateWithSeed: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly systeminstruction_createNonceAccount: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly systeminstruction_advanceNonceAccount: (a: number, b: number) => number;
  readonly systeminstruction_withdrawNonceAccount: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly systeminstruction_authorizeNonceAccount: (a: number, b: number, c: number) => number;
  readonly __wbg_message_free: (a: number) => void;
  readonly __wbg_get_message_recent_blockhash: (a: number) => number;
  readonly __wbg_set_message_recent_blockhash: (a: number, b: number) => void;
  readonly __wbg_pubkey_free: (a: number) => void;
  readonly pubkey_constructor: (a: number, b: number) => void;
  readonly pubkey_toString: (a: number, b: number) => void;
  readonly pubkey_isOnCurve: (a: number) => number;
  readonly pubkey_equals: (a: number, b: number) => number;
  readonly pubkey_toBytes: (a: number, b: number) => void;
  readonly pubkey_createWithSeed: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly pubkey_createProgramAddress: (a: number, b: number, c: number, d: number) => void;
  readonly pubkey_findProgramAddress: (a: number, b: number, c: number, d: number) => void;
  readonly solana_program_init: () => void;
  readonly __wbg_instructions_free: (a: number) => void;
  readonly instructions_constructor: () => number;
  readonly instructions_push: (a: number, b: number) => void;
  readonly hash_constructor: (a: number, b: number) => void;
  readonly hash_toString: (a: number, b: number) => void;
  readonly hash_equals: (a: number, b: number) => number;
  readonly hash_toBytes: (a: number, b: number) => void;
  readonly __wbg_hash_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
