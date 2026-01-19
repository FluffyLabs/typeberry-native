declare module "*.wasm" {
  const init: () => Promise<WebAssembly.Module>;
  export default init;
}
