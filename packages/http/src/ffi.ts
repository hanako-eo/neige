import { dirname } from "node:path"
import { fileURLToPath } from "node:url"
import { createRequire } from "node:module"
import { find_lib, get_platform } from "@neige/utils/lib"
import { IncomingHttpHeaders } from "node:http"
import { AddressInfo } from "node:net"

const require = createRequire(import.meta.url)
const __dirname = dirname(fileURLToPath(import.meta.url))

const lib = require(
    find_lib(__dirname, "../", "neige-http") ?? `@neige/http-${get_platform()}`
)

export interface Socket {
    readonly remoteAddr: AddressInfo
    readonly localAddr: AddressInfo
}

export interface Request {
    readonly method: string
    readonly url: string
    readonly version: string

    headers(): IncomingHttpHeaders
    socket(): Socket
    close(): void
}

export interface ServerConstructor {
    new (callback: (req: Request) => void): Server
}

export interface Server {
    getPoolCapacity(): number
    setPoolCapacity(poolCapacity: number): void
    listen(port: number): void
    close(): void
}
export const Server: ServerConstructor = lib.Server
