import wasm from '@rollup/plugin-wasm';
import { dts } from 'rolldown-plugin-dts';
import { defineConfig } from 'rolldown';
import { resolve } from 'path';

// Custom plugin to resolve local @typeberry packages
function resolveLocalPackages() {
  return {
    name: 'resolve-local-packages',
    resolveId(id) {
      if (id === '@typeberry/bandersnatch') {
        return resolve('./bandersnatch/src/index.ts');
      }
      if (id === '@typeberry/ed25519') {
        return resolve('./ed25519/pkg/ed25519_wasm.js');
      }
      if (id === '@typeberry/reed-solomon') {
        return resolve('./reed-solomon/pkg/reed_solomon_wasm.js');
      }
      return null;
    }
  };
}

export default defineConfig({
  input: 'native/index.ts',
  external: [/^@typeberry\/.*-native-.*$/],
  plugins: [
    resolveLocalPackages(),
    wasm({
      maxFileSize: 100000000
    }), 
    dts({
      resolve: true, // bundle all dependencies
    })
  ],
  output: {
    dir: 'dist',
  },
})
