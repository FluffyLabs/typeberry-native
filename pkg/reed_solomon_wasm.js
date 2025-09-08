
import * as wasm from "./reed_solomon_wasm_bg.wasm";
import { __wbg_set_wasm } from "./reed_solomon_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./reed_solomon_wasm_bg.js";
