import { Server as RustServer } from "./ffi.js"
import { cpus } from "os"
import { on_exit } from "@neige/utils/exit"

export default class Server {
    private refferer: NodeJS.Timeout | undefined
    private inner_server: RustServer
    private reffered = true
    private started = false
    private closed = false

    constructor() {
        this.inner_server = new RustServer((req) => {
            console.log(req.headers())
            req.close()
        })
        this.inner_server.setPoolCapacity(cpus().length)

        this.close = this.close.bind(this)
        this.loop = this.loop.bind(this)

        // forces the server to close correctly if the process exits
        on_exit(this.close)
    }

    public getPoolCapacity(): number {
        return this.inner_server.getPoolCapacity()
    }

    public setPoolCapacity(pool: number): this {
        this.inner_server.setPoolCapacity(pool)
        return this
    }

    public ref(): this {
        if (!this.reffered) {
            this.reffered = true
            if (this.started) this.refferer = setInterval(this.loop, 2147483647)
        }
        return this
    }

    public unref(): this {
        clearInterval(this.refferer)
        if (this.reffered) this.reffered = false
        return this
    }

    public hasRef(): boolean {
        return this.reffered
    }

    public listen(port: number): this {
        this.started = true
        // forces the nodejs event loop to stay alive
        if (this.reffered) this.loop()

        this.inner_server.listen(port)
        return this
    }

    public close(): this {
        if (this.closed) return this

        this.unref()
        this.inner_server.close()
        this.started = false
        this.closed = true

        return this
    }

    private loop() {
        if (this.reffered) this.refferer = setInterval(this.loop, 2147483647)
    }
}
