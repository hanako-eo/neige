import Server, { ServerCallback } from "./server.js"
export { Server }

export function createServer(callback: ServerCallback) {
    return new Server(callback)
}
