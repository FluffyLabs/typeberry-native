export * as bandersnatch from "@typeberry/bandersnatch";
export * as ed25519 from "@typeberry/ed25519";
export * as reedSolomon from "@typeberry/reed-solomon";

import bandersnatchInit from "@typeberry/bandersnatch";
import bandersnatchWasm from "../bandersnatch/pkg/bandersnatch_bg.wasm";
import ed25519Init from "@typeberry/ed25519";
import ed25519Wasm from "../ed25519/pkg/ed25519_wasm_bg.wasm";
import reedSolomonInit from "@typeberry/reed-solomon";
import reedSolomonWasm from "../reed-solomon/pkg/reed_solomon_wasm_bg.wasm";

await bandersnatchInit(bandersnatchWasm);
await ed25519Init(ed25519Wasm);
await reedSolomonInit(reedSolomonWasm);
