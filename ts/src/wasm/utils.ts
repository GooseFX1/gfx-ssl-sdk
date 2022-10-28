export function splitu64(i: bigint): [number, number] {
    return [Number(i & ((1n << 32n) - 1n)), Number(i >> 32n)];
}

export function mergeu64(lo: number, hi: number): bigint {
    return (BigInt(hi) << 32n) | (BigInt(lo) & ((1n << 32n) - 1n));
}