import { type RustServer, rust_server } from "./ffi.js"
import { cpus } from "os"
import { on_exit } from "@neige/utils/exit"

export default class Server {
    private inner_server: RustServer
    private reffered = true
    private started = false
    private closed = false

    constructor() {
        this.inner_server = rust_server.create_server(async (test) => {
            console.log(test)
        })
        this.setPoolCapacity(cpus().length)

        this.close = this.close.bind(this)
        this.loop = this.loop.bind(this)

        // forces the server to close correctly if the process exits
        on_exit(this.close)
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
            if (this.started) setImmediate(this.loop)
        }
        return this
    }

    public unref(): Server {
        if (this.reffered) this.reffered = false
        return this
    }

    public hasRef(): boolean {
        return this.reffered
    }

    public listen(port: number) {
        this.started = true
        // forces the nodejs event loop to stay alive
        if (this.reffered) setImmediate(this.loop)

        rust_server.launch_server(this.inner_server, port)
    }

    public close() {
        if (this.closed) return

        rust_server.close_server(this.inner_server)
        this.started = false
        this.closed = true
        this.unref()
    }

    private loop() {
        if (this.reffered) setImmediate(this.loop)
    }
}
