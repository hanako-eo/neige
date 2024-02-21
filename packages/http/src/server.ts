import {
    type RustServer,
    rust_server
} from "./ffi.js"
import { cpus } from "os"
import { once_exit } from "@neige/utils/exit"

export default class Server {
    private inner_server: RustServer
    private reffered = true
    private closed = false

    constructor() {
        this.inner_server = rust_server.create_server(() => { })
        this.setPoolCapacity(cpus().length)

        // forces the server to close correctly if the process exits
        once_exit(this.close)
    }

    public getPoolCapacity(): number {
        return rust_server.get_pool_capacity(this.inner_server)
    }

    public setPoolCapacity(pool: number) {
        rust_server.set_pool_capacity(this.inner_server, pool)
    }

    public getObstruction(): boolean {
        return rust_server.get_obstruction(this.inner_server)
    }

    public setObstruction(obstruct: boolean) {
        rust_server.set_obstruction(this.inner_server, obstruct)
    }

    public ref(): Server {
        if (!this.reffered) {
            this.reffered = true
            process.nextTick(this.loop)
        }
        return this
    }

    public unref(): Server {
        if (this.reffered)
            this.reffered = false
        return this
    }

    public hasRef(): boolean {
        return this.reffered
    }

    public listen(port: number) {
        // forces the nodejs event loop to stay alive
        if (this.reffered)
            process.nextTick(this.loop)

        rust_server.launch_server(this.inner_server, port)
    }

    public close() {
        if (this.closed)
            return

        rust_server.close_server(this.inner_server)
        this.closed = true
        this.unref()
    }

    private loop() {
        if (this.reffered)
            process.nextTick(this.loop)
    }
}
