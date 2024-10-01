/**
 * @param {bigint} bi
 */
export function bigint_to_bytes(bi) {
    const len = Math.max(1, Math.ceil(bi.toString(2).length / 8));
    const res = new Uint8Array(len);
    let i = 0;
    while (bi > 0) {
        res[i] = Number(bi % BigInt(256))
        bi = bi / BigInt(256)
        i += 1
    }
    return res.reverse();
}