import { defineConfig } from "rollup"
import typescript from "@rollup/plugin-typescript"
import replace from "@rollup/plugin-replace"

export default defineConfig({
    input: "./src/index.ts",
    output: {
        dir: "build",
        format: "module"
    },
    plugins: [
        replace({
            preventAssignment: true,
            'process.env.MODE': JSON.stringify(process.env.MODE)
        }),
        typescript(),
    ]
})