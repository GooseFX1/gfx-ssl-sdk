
import BN from 'bn.js';
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import * as lo from 'buffer-layout';

class PublicKeyLayout extends lo.Blob {
    constructor(property) {
        super(32, property);
    }

    decode(b, offset) {
        return new PublicKey(super.decode(b, offset));
    }

    encode(src, b, offset) {
        return super.encode(src.toBuffer(), b, offset);
    }
}

export function publicKeyLayout(property) {
    return new PublicKeyLayout(property);
}


class BNLayout extends lo.Blob {
    constructor(public b, public property) {
        super(b, property);
    }

    decode(b, offset) {
        return new BN(super.decode(b, offset), 10, 'le');
    }

    encode(src, b, offset) {
        return super.encode(src.toArrayLike(Buffer, 'le', (this as any).span), b, offset);
    }
}


export function u64(property) {
    return new BNLayout(8, property);
}



export function u128(property) {
    return new BNLayout(16, property);
}
