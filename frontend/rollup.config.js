import rust from "@wasm-tool/rollup-plugin-rust";
import commonjs from "@rollup/plugin-commonjs";
import nodeResolve from "@rollup/plugin-node-resolve";
import json from "@rollup/plugin-json";
import nodePolyfills from 'rollup-plugin-node-polyfills';

export default {
    input: {
        "frontend": "Cargo.toml",
    },
    output: {
        name: "bundle",
        dir: "html",
        format: 'es'
    },
    plugins: [
        rust({
            serverPath: "/secure/"
        }),
        json(),
        nodePolyfills(),
        nodeResolve({
            browser: true,
            jsnext: true,
            main: true
        }),
        commonjs({
            include: [ "node_modules/**" ]
        }),
    ],
};
