import { existsSync } from "node:fs"
import path from "node:path"

export function get_platform(): string | null {
    switch (process.platform) {
        case "win32":
            return "win32-x64-msvc"
        case "darwin":
            return "darwin-x64"
        case "linux":
            return "linux-x64-gnu"
        default:
            return null
    }
}

export function find_lib(...paths: Array<string>): string | null {
    // I assume that paths is not empty
    const lib_name = paths.pop()!
    const target_name = get_platform()

    const lib_path = path.join(...paths, `${lib_name}.${target_name}.node`)

    if (existsSync(lib_path)) return lib_path

    return null
}
