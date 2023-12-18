import * as koffi from 'koffi'
import { find_lib } from "@neige/utils/lib"

const lib = koffi.load(find_lib(`./target/${process.env.MODE === "dev" ? "debug" : "release"}`, "neige_http"))

const ServerCallback = koffi.pointer(koffi.proto("ServerCallback", "void", []))

const RustServer = koffi.struct({
    // javascript doesn't need to know the complete structure
    pool: "int",
    obstruct: "bool",
})
export interface RustServer {
    pool: number
    obstruct: boolean
}

const _create_server = lib.func("create_server", koffi.pointer(RustServer), [ServerCallback])
export function create_server(callback: () => void): RustServer {
    return _create_server(koffi.register(callback, ServerCallback))
}

const _set_pool_capacity = lib.func("set_pool_capacity", koffi.pointer(RustServer), [ServerCallback, "uint32"])
export function set_pool_capacity(server: RustServer, pool_capacity: number): RustServer {
    return _set_pool_capacity(server, pool_capacity)
}

const _set_obstruction = lib.func("set_obstruction", koffi.pointer(RustServer), [ServerCallback, "uint32"])
export function set_obstruction(server: RustServer, obstruct: boolean): RustServer {
    return _set_obstruction(server, obstruct)
}

const _launch_server = lib.func("launch_server", "void", [koffi.pointer(RustServer), "uint16"])
export function launch_server(server: RustServer, port: number) {
    _launch_server(server, port)
}

const _close_server = lib.func("close_server", "void", [koffi.pointer(RustServer)])
export function close_server(server: RustServer) {
    _close_server(server)
}
