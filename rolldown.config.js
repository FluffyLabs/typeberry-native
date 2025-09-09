import wasm from '@rollup/plugin-wasm';
import { dts } from 'rolldown-plugin-dts';
import { defineConfig } from 'rolldown';


export default defineConfig({
  input: 'native/index.ts',
  plugins: [wasm({
    maxFileSize: 100000000
  }), dts({
    resolve: true, // bundle all dependencies
  })],
  output: {
    dir: 'dist',
  },
})
