import { dirname } from "node:path"
import { fileURLToPath } from "node:url"
import { createRequire } from "node:module"
import { find_lib, get_platform } from "@neige/utils/lib"

const require = createRequire(import.meta.url);
const __dirname = dirname(fileURLToPath(import.meta.url))

const lib = require(find_lib(__dirname, "../", "neige-http") ?? `@neige/http-${get_platform()}`)

export interface ServerConstructor {
    new(callback: () => void): Server
}

export interface Server {
    getPoolCapacity(): number
    setPoolCapacity(poolCapacity: number): void
    getObstruction(): boolean
    setObstruction(obstruction: boolean): void
    listen(port: number): void
    close(): void
}
export const Server: ServerConstructor = lib.Server
