import * as wasm from "./bandersnatch_wasm_bg.wasm";
import { __wbg_set_wasm } from "./bandersnatch_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./bandersnatch_wasm_bg.js";
