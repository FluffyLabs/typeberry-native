import * as wasm from "./bandersnatch_bg.wasm";
import { __wbg_set_wasm } from "./bandersnatch_bg.js";
__wbg_set_wasm(wasm);
export * from "./bandersnatch_bg.js";
