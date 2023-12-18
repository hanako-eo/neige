import path from "node:path"

export function find_lib(folder: string, name: string): string {
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

    return path.join(folder, `lib${name}${file_extension}`)
}
