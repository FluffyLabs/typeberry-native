import wasm from '@rollup/plugin-wasm';
import { defineConfig } from 'rolldown';


export default defineConfig({
  input: 'native/index.ts',
  plugins: [wasm()],
  output: {
    dir: 'dist',
  },
})
