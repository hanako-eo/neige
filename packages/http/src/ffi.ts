import koffi from "koffi"
import { dirname } from "path"
import { fileURLToPath } from "url"
import { find_lib } from "@neige/utils/lib"

const __dirname = dirname(fileURLToPath(import.meta.url))
const lib = koffi.load(
    find_lib(
        __dirname,
        `../target/${process.env.MODE === "dev" ? "debug" : "release"}`,
        "neige_http"
    )
)

type ImportFFIFunctions<
    T extends Record<string, (...args: Array<never>) => unknown>,
> = {
    [K in keyof T]: [
        koffi.TypeSpec,
        Array<koffi.TypeSpec>,
        (
            method: koffi.KoffiFunction,
            ...args: Parameters<T[K]>
        ) => ReturnType<T[K]>,
    ]
}
function import_from_ffi<
    T extends Record<string, (...args: Array<never>) => unknown>,
>(lib: koffi.IKoffiLib, functions: ImportFFIFunctions<T>): T {
    return Object.fromEntries(
        Object.entries(functions).map(
            ([name, [return_type, args_types, fn]]) => {
                const method = lib.func(name as string, return_type, args_types)
                return [name, fn.bind(undefined, method)] as const
            }
        )
    ) as T
}

// SERVER
export type RustRequest = typeof RustRequest
// javascript doesn't need to know the content of the structure
const RustRequest = koffi.pointer(koffi.opaque())
const ServerCallback = koffi.pointer(
    koffi.proto("ServerCallback", "void", [RustRequest])
    )
    
export type RustServer = typeof RustServer
// javascript doesn't need to know the content of the structure
const RustServer = koffi.pointer(koffi.opaque())

type FFIServer = {
    create_server(callback: (req: RustRequest) => void): RustServer
    get_pool_capacity(server: RustServer): number
    get_obstruction(server: RustServer): boolean
    set_pool_capacity(server: RustServer, pool_capacity: number): void
    set_obstruction(server: RustServer, obstruct: boolean): void
    launch_server(server: RustServer, port: number): void
    close_server(server: RustServer): void
}
export const rust_server = import_from_ffi<FFIServer>(lib, {
    create_server: [
        RustServer,
        [ServerCallback],
        (_create_server, callback) => {
            return _create_server(koffi.register(callback, ServerCallback))
        },
    ],
    get_pool_capacity: [
        "uint32",
        [RustServer],
        (_get_pool_capacity, server) => {
            return _get_pool_capacity(server)
        },
    ],
    set_pool_capacity: [
        "void",
        [RustServer, "uint32"],
        (_set_pool_capacity, server, pool_capacity) => {
            _set_pool_capacity(server, pool_capacity)
        },
    ],
    get_obstruction: [
        "uint32",
        [RustServer],
        (_get_obstruction, server) => {
            return _get_obstruction(server)
        },
    ],
    set_obstruction: [
        "void",
        [RustServer, "uint32"],
        (_set_obstruction, server, obstruct) => {
            _set_obstruction(server, obstruct)
        },
    ],
    launch_server: [
        "void",
        [RustServer, "uint16"],
        (_launch_server, server, port) => {
            _launch_server(koffi.address(server), port)
        },
    ],
    close_server: [
        "void",
        [RustServer],
        (_close_server, server) => {
            _close_server(server)
        },
    ],
})

type FFIRequest = {
    get_method(req: RustRequest): string
    get_url(req: RustRequest): string
    get_http_version(req: RustRequest): number
}
export const rust_request = import_from_ffi<FFIRequest>(lib, {
    get_method: [
        "char*",
        [RustRequest],
        (_get_method, req) => {
            return _get_method(req)
        },
    ],
    get_url: [
        "char*",
        [RustRequest],
        (_get_url, req) => {
            return _get_url(req)
        },
    ],
    get_http_version: [
        "uint8",
        [RustRequest],
        (_get_http_version, req) => {
            return _get_http_version(req)
        },
    ],
})
