import path from "node:path"

export function find_lib(...paths: Array<string>): string {
    let file_extension: string
    switch (process.platform) {
        case "win32":
            file_extension = ".dll"
            break
        case "darwin":
            file_extension = ".dylib"
            break
        default:
            file_extension = ".so"
    }

    // I assume that paths is not empty
    const name = paths.pop()!

    return path.join(...paths, `lib${name}${file_extension}`)
}
