import * as koffi from 'koffi'
import { find_lib } from "@neige/utils/lib"

const lib = koffi.load(find_lib(`./target/${process.env.MODE === "dev" ? "debug" : "release"}`, "neige_http"))

type ImportFFIFunctions<T extends Record<string, (...args: Array<any>) => any>> = {
    [K in keyof T]: [koffi.TypeSpec, Array<koffi.TypeSpec>, (method: koffi.KoffiFunction, ...args: Parameters<T[K]>) => ReturnType<T[K]>]
} 
function import_from_ffi<T extends Record<string, (...args: Array<any>) => any>>(
    lib: koffi.IKoffiLib,
    functions: ImportFFIFunctions<T>
): T {
    return Object.fromEntries(
        Object
        .entries(functions)
        .map(([name, [return_type, args_types, fn]]) => {
            const method = lib.func(name as string, return_type, args_types)
            return [name, fn.bind(method)] as const
        })
    ) as T
}

// SERVER
const ServerCallback = koffi.pointer(koffi.proto("ServerCallback", "void", []))

export interface RustServer {}
const RustServer = koffi.struct({
    // javascript doesn't need to know the complete structure
    pool: "int",
    obstruct: "bool",
})

type FFIServer = {
    create_server(callback: () => void): RustServer
    get_pool_capacity(server: RustServer): number
    get_obstruction(server: RustServer): boolean
    set_pool_capacity(server: RustServer, pool_capacity: number): void
    set_obstruction(server: RustServer, obstruct: boolean): void
    launch_server(server: RustServer, port: number): void
    close_server(server: RustServer): void
}
export const rust_server = import_from_ffi<FFIServer>(lib, {
    create_server: [koffi.pointer(RustServer), [ServerCallback], (_create_server, callback) => {
        return _create_server(koffi.register(callback, ServerCallback))
    }],
    get_pool_capacity: ["uint32", [koffi.pointer(RustServer)], (_get_pool_capacity, server) => {
        return _get_pool_capacity(server)
    }],
    set_pool_capacity: ["void", [koffi.pointer(RustServer), "uint32"], (_set_pool_capacity, server, pool_capacity) => {
        return _set_pool_capacity(server, pool_capacity)
    }],
    get_obstruction: ["uint32", [koffi.pointer(RustServer)], (_get_obstruction, server) => {
        return _get_obstruction(server)
    }],
    set_obstruction: ["void", [koffi.pointer(RustServer), "uint32"], (_set_obstruction, server, obstruct) => {
        return _set_obstruction(server, obstruct)
    }],
    launch_server: ["void", [koffi.pointer(RustServer), "uint16"], (_launch_server, server, port) => {
        _launch_server(server, port)
    }],
    close_server: ["void", [koffi.pointer(RustServer)], (_close_server, server) => {
        _close_server(server)
    }],
})

