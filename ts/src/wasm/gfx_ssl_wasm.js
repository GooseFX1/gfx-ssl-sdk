
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
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
export function swap(ssl_in, ssl_out, pair, liability_in, liability_out, swapped_liability_in, swapped_liability_out, oracles, amount_in) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(ssl_in, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(ssl_out, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArray8ToWasm0(pair, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        uint64CvtShim[0] = liability_in;
        const low3 = u32CvtShim[0];
        const high3 = u32CvtShim[1];
        uint64CvtShim[0] = liability_out;
        const low4 = u32CvtShim[0];
        const high4 = u32CvtShim[1];
        uint64CvtShim[0] = swapped_liability_in;
        const low5 = u32CvtShim[0];
        const high5 = u32CvtShim[1];
        uint64CvtShim[0] = swapped_liability_out;
        const low6 = u32CvtShim[0];
        const high6 = u32CvtShim[1];
        _assertClass(oracles, OracleRegistry);
        var ptr7 = oracles.ptr;
        oracles.ptr = 0;
        uint64CvtShim[0] = amount_in;
        const low8 = u32CvtShim[0];
        const high8 = u32CvtShim[1];
        wasm.swap(retptr, ptr0, len0, ptr1, len1, ptr2, len2, low3, high3, low4, high4, low5, high5, low6, high6, ptr7, low8, high8);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
            throw takeObject(r1);
        }
        return SwapResult.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4);
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}
/**
* Initialize Javascript logging and panic handler
*/
export function solana_program_init() {
    wasm.solana_program_init();
}

/**
*/
export class Hash {

    static __wrap(ptr) {
        const obj = Object.create(Hash.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hash_free(ptr);
    }
    /**
    * Create a new Hash object
    *
    * * `value` - optional hash as a base58 encoded string, `Uint8Array`, `[number]`
    * @param {any} value
    */
    constructor(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hash_constructor(retptr, addHeapObject(value));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Hash.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Return the base58 string representation of the hash
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hash_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Checks if two `Hash`s are equal
    * @param {Hash} other
    * @returns {boolean}
    */
    equals(other) {
        _assertClass(other, Hash);
        const ret = wasm.hash_equals(this.ptr, other.ptr);
        return ret !== 0;
    }
    /**
    * Return the `Uint8Array` representation of the hash
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.hash_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
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

    static __wrap(ptr) {
        const obj = Object.create(Instruction.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_instruction_free(ptr);
    }
}
/**
*/
export class Instructions {

    static __wrap(ptr) {
        const obj = Object.create(Instructions.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_instructions_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.instructions_constructor();
        return Instructions.__wrap(ret);
    }
    /**
    * @param {Instruction} instruction
    */
    push(instruction) {
        _assertClass(instruction, Instruction);
        var ptr0 = instruction.ptr;
        instruction.ptr = 0;
        wasm.instructions_push(this.ptr, ptr0);
    }
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

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_message_free(ptr);
    }
    /**
    * The id of a recent ledger entry.
    */
    get recent_blockhash() {
        const ret = wasm.__wbg_get_message_recent_blockhash(this.ptr);
        return Hash.__wrap(ret);
    }
    /**
    * The id of a recent ledger entry.
    * @param {Hash} arg0
    */
    set recent_blockhash(arg0) {
        _assertClass(arg0, Hash);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_message_recent_blockhash(this.ptr, ptr0);
    }
}
/**
*/
export class OracleRegistry {

    static __wrap(ptr) {
        const obj = Object.create(OracleRegistry.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_oracleregistry_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.oracleregistry_new();
        return OracleRegistry.__wrap(ret);
    }
    /**
    * @param {Uint8Array} key
    * @param {Uint8Array} data
    */
    add_oracle(key, data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(key, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.oracleregistry_add_oracle(retptr, this.ptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class Pubkey {

    static __wrap(ptr) {
        const obj = Object.create(Pubkey.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pubkey_free(ptr);
    }
    /**
    * Create a new Pubkey object
    *
    * * `value` - optional public key as a base58 encoded string, `Uint8Array`, `[number]`
    * @param {any} value
    */
    constructor(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.pubkey_constructor(retptr, addHeapObject(value));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Pubkey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Return the base58 string representation of the public key
    * @returns {string}
    */
    toString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.pubkey_toString(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Check if a `Pubkey` is on the ed25519 curve.
    * @returns {boolean}
    */
    isOnCurve() {
        const ret = wasm.pubkey_isOnCurve(this.ptr);
        return ret !== 0;
    }
    /**
    * Checks if two `Pubkey`s are equal
    * @param {Pubkey} other
    * @returns {boolean}
    */
    equals(other) {
        _assertClass(other, Pubkey);
        const ret = wasm.pubkey_equals(this.ptr, other.ptr);
        return ret !== 0;
    }
    /**
    * Return the `Uint8Array` representation of the public key
    * @returns {Uint8Array}
    */
    toBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.pubkey_toBytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Derive a Pubkey from another Pubkey, string seed, and a program id
    * @param {Pubkey} base
    * @param {string} seed
    * @param {Pubkey} owner
    * @returns {Pubkey}
    */
    static createWithSeed(base, seed, owner) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(base, Pubkey);
            const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(owner, Pubkey);
            wasm.pubkey_createWithSeed(retptr, base.ptr, ptr0, len0, owner.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Pubkey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Derive a program address from seeds and a program id
    * @param {any[]} seeds
    * @param {Pubkey} program_id
    * @returns {Pubkey}
    */
    static createProgramAddress(seeds, program_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayJsValueToWasm0(seeds, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(program_id, Pubkey);
            wasm.pubkey_createProgramAddress(retptr, ptr0, len0, program_id.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return Pubkey.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Find a valid program address
    *
    * Returns:
    * * `[PubKey, number]` - the program address and bump seed
    * @param {any[]} seeds
    * @param {Pubkey} program_id
    * @returns {any}
    */
    static findProgramAddress(seeds, program_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayJsValueToWasm0(seeds, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(program_id, Pubkey);
            wasm.pubkey_findProgramAddress(retptr, ptr0, len0, program_id.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
export class SwapResult {

    static __wrap(ptr) {
        const obj = Object.create(SwapResult.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_swapresult_free(ptr);
    }
    /**
    */
    get out() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_swapresult_out(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            u32CvtShim[0] = r0;
            u32CvtShim[1] = r1;
            const n0 = uint64CvtShim[0];
            return n0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {BigInt} arg0
    */
    set out(arg0) {
        uint64CvtShim[0] = arg0;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        wasm.__wbg_set_swapresult_out(this.ptr, low0, high0);
    }
    /**
    */
    get price_impact() {
        const ret = wasm.__wbg_get_swapresult_price_impact(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set price_impact(arg0) {
        wasm.__wbg_set_swapresult_price_impact(this.ptr, arg0);
    }
}

export class SystemInstruction {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_systeminstruction_free(ptr);
    }
    /**
    * @param {Pubkey} from_pubkey
    * @param {Pubkey} to_pubkey
    * @param {BigInt} lamports
    * @param {BigInt} space
    * @param {Pubkey} owner
    * @returns {Instruction}
    */
    static createAccount(from_pubkey, to_pubkey, lamports, space, owner) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        uint64CvtShim[0] = lamports;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        uint64CvtShim[0] = space;
        const low1 = u32CvtShim[0];
        const high1 = u32CvtShim[1];
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_createAccount(from_pubkey.ptr, to_pubkey.ptr, low0, high0, low1, high1, owner.ptr);
        return Instruction.__wrap(ret);
    }
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
    static createAccountWithSeed(from_pubkey, to_pubkey, base, seed, lamports, space, owner) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        uint64CvtShim[0] = lamports;
        const low1 = u32CvtShim[0];
        const high1 = u32CvtShim[1];
        uint64CvtShim[0] = space;
        const low2 = u32CvtShim[0];
        const high2 = u32CvtShim[1];
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_createAccountWithSeed(from_pubkey.ptr, to_pubkey.ptr, base.ptr, ptr0, len0, low1, high1, low2, high2, owner.ptr);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} pubkey
    * @param {Pubkey} owner
    * @returns {Instruction}
    */
    static assign(pubkey, owner) {
        _assertClass(pubkey, Pubkey);
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_assign(pubkey.ptr, owner.ptr);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} pubkey
    * @param {Pubkey} base
    * @param {string} seed
    * @param {Pubkey} owner
    * @returns {Instruction}
    */
    static assignWithSeed(pubkey, base, seed, owner) {
        _assertClass(pubkey, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_assignWithSeed(pubkey.ptr, base.ptr, ptr0, len0, owner.ptr);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} from_pubkey
    * @param {Pubkey} to_pubkey
    * @param {BigInt} lamports
    * @returns {Instruction}
    */
    static transfer(from_pubkey, to_pubkey, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        uint64CvtShim[0] = lamports;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.systeminstruction_transfer(from_pubkey.ptr, to_pubkey.ptr, low0, high0);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} from_pubkey
    * @param {Pubkey} from_base
    * @param {string} from_seed
    * @param {Pubkey} from_owner
    * @param {Pubkey} to_pubkey
    * @param {BigInt} lamports
    * @returns {Instruction}
    */
    static transferWithSeed(from_pubkey, from_base, from_seed, from_owner, to_pubkey, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(from_base, Pubkey);
        const ptr0 = passStringToWasm0(from_seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(from_owner, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        uint64CvtShim[0] = lamports;
        const low1 = u32CvtShim[0];
        const high1 = u32CvtShim[1];
        const ret = wasm.systeminstruction_transferWithSeed(from_pubkey.ptr, from_base.ptr, ptr0, len0, from_owner.ptr, to_pubkey.ptr, low1, high1);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} pubkey
    * @param {BigInt} space
    * @returns {Instruction}
    */
    static allocate(pubkey, space) {
        _assertClass(pubkey, Pubkey);
        uint64CvtShim[0] = space;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.systeminstruction_allocate(pubkey.ptr, low0, high0);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} address
    * @param {Pubkey} base
    * @param {string} seed
    * @param {BigInt} space
    * @param {Pubkey} owner
    * @returns {Instruction}
    */
    static allocateWithSeed(address, base, seed, space, owner) {
        _assertClass(address, Pubkey);
        _assertClass(base, Pubkey);
        const ptr0 = passStringToWasm0(seed, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        uint64CvtShim[0] = space;
        const low1 = u32CvtShim[0];
        const high1 = u32CvtShim[1];
        _assertClass(owner, Pubkey);
        const ret = wasm.systeminstruction_allocateWithSeed(address.ptr, base.ptr, ptr0, len0, low1, high1, owner.ptr);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} from_pubkey
    * @param {Pubkey} nonce_pubkey
    * @param {Pubkey} authority
    * @param {BigInt} lamports
    * @returns {Array<any>}
    */
    static createNonceAccount(from_pubkey, nonce_pubkey, authority, lamports) {
        _assertClass(from_pubkey, Pubkey);
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authority, Pubkey);
        uint64CvtShim[0] = lamports;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.systeminstruction_createNonceAccount(from_pubkey.ptr, nonce_pubkey.ptr, authority.ptr, low0, high0);
        return takeObject(ret);
    }
    /**
    * @param {Pubkey} nonce_pubkey
    * @param {Pubkey} authorized_pubkey
    * @returns {Instruction}
    */
    static advanceNonceAccount(nonce_pubkey, authorized_pubkey) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        const ret = wasm.systeminstruction_advanceNonceAccount(nonce_pubkey.ptr, authorized_pubkey.ptr);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} nonce_pubkey
    * @param {Pubkey} authorized_pubkey
    * @param {Pubkey} to_pubkey
    * @param {BigInt} lamports
    * @returns {Instruction}
    */
    static withdrawNonceAccount(nonce_pubkey, authorized_pubkey, to_pubkey, lamports) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        _assertClass(to_pubkey, Pubkey);
        uint64CvtShim[0] = lamports;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.systeminstruction_withdrawNonceAccount(nonce_pubkey.ptr, authorized_pubkey.ptr, to_pubkey.ptr, low0, high0);
        return Instruction.__wrap(ret);
    }
    /**
    * @param {Pubkey} nonce_pubkey
    * @param {Pubkey} authorized_pubkey
    * @param {Pubkey} new_authority
    * @returns {Instruction}
    */
    static authorizeNonceAccount(nonce_pubkey, authorized_pubkey, new_authority) {
        _assertClass(nonce_pubkey, Pubkey);
        _assertClass(authorized_pubkey, Pubkey);
        _assertClass(new_authority, Pubkey);
        const ret = wasm.systeminstruction_authorizeNonceAccount(nonce_pubkey.ptr, authorized_pubkey.ptr, new_authority.ptr);
        return Instruction.__wrap(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {

    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_16f24b0728c5e67b = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_next_c7a2a6b012059a5e = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_dd1a890d37e38d73 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_982b1c7ac0cbc69d = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_2def2d1fb38b02cd = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_4b9cedbeda0c0e30 = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_8bbb82393651dd9c = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_newwithlength_9c398a17849b31ce = function(arg0) {
        const ret = new Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_a42efa3c7f01c8b1 = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_isArray_8480ed76e5369634 = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_push_a72df856079e6930 = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_values_71935f80778b5113 = function(arg0) {
        const ret = getObject(arg0).values();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_buffer_5e74a88a1424a2e0 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_e3b800e570795b3c = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_5b8081e9d002f0df = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_30803400a8f15c59 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_8a8537f46e056474 = function(arg0) {
        const ret = getObject(arg0) instanceof Uint8Array;
        return ret;
    };
    imports.wbg.__wbg_debug_6e114a5b27d7915d = function(arg0) {
        console.debug(getObject(arg0));
    };
    imports.wbg.__wbg_error_ca520cb687b085a1 = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__wbg_info_32ab782ec7072fac = function(arg0) {
        console.info(getObject(arg0));
    };
    imports.wbg.__wbg_log_fbd13631356d44e4 = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_warn_97f10a6b0dbb8c5c = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_instruction_new = function(arg0) {
        const ret = Instruction.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_pubkey_new = function(arg0) {
        const ret = Pubkey.__wrap(arg0);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

