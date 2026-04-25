import { readFileSync, writeFileSync, unlinkSync } from 'node:fs';
import { brotliCompressSync, constants } from 'node:zlib';

const input = './build/index.html';
const output = './build/index.html.br';

const compressed = brotliCompressSync(readFileSync(input), {
    params: { [constants.BROTLI_PARAM_QUALITY]: constants.BROTLI_MAX_QUALITY },
});
writeFileSync(output, compressed);
unlinkSync(input);
