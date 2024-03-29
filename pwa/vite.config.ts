import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill';
import { NodeModulesPolyfillPlugin } from '@esbuild-plugins/node-modules-polyfill';
import nodePolyfills from 'rollup-plugin-polyfill-node';
import cjs from '@rollup/plugin-commonjs';
import svgr from "vite-plugin-svgr";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
      svgr(),
      react(),
    ],
    define: {
      "window.global": {},
      'process.env': process.env,
      'process.version': JSON.stringify(process.version),
    },
    server: {
      https: false,
    },
    build: {
      // minify: false,
      //target: "es2015",
      outDir: 'dist',
      commonjsOptions: { 
        transformMixedEsModules: true,
        include: [/react-twitter-embed/]
      },
      rollupOptions: {
        plugins: [
          // Enable rollup polyfills plugin
          // used during production bundling
          nodePolyfills({
            include: ['node_modules/** / *.js', '../../node_modules/** / *.js'],
          }),
          cjs(),
          svgr(),
        ],
      },
    }
});

