export const green = '#046A38';

/**
 * @param {File} f
 */
export async function readFile(f) {
  let arraybuf = await f.arrayBuffer();
  const blob = new Blob([new Uint8Array(arraybuf)], { type: f.type });
  let s = await blob.text();
  return s;
}

/**
 * @param {bigint} bi
 */
export function bigint_to_bytes(bi) {
  const len = Math.max(1, Math.ceil(bi.toString(2).length / 8));
  const res = new Uint8Array(len);
  let i = 0;
  while (bi > 0) {
    res[i] = Number(bi % BigInt(256));
    bi = bi / BigInt(256);
    i += 1;
  }
  return res.reverse();
}

/**
 * @param {number} millis
 */
export function millisToMinutesAndSeconds(millis) {
  var minutes = Math.floor(millis / 60000);
  var seconds = Math.floor((millis % 60000) / 1000);
  if (minutes == 0) {
    return seconds.toFixed(0) + ' seconds';
  } else {
    return minutes + ' minutes ' + seconds.toFixed(0) + ' seconds';
  }
}
