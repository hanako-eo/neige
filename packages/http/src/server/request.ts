import EventEmitter from "node:events"
import { Request as RustRequest, Socket as RustSocket } from "../ffi.js"
import { IncomingHttpHeaders } from "node:http"

export default class IncomingMessage extends EventEmitter {
    public readonly headers: IncomingHttpHeaders
    public readonly method: string
    public readonly url: string
    public readonly httpVersion: string
    public readonly httpVersionMajor: number
    public readonly httpVersionMinor: number

    public readonly readableEnded = false

    private socket: RustSocket

    constructor(private inner: RustRequest) {
        super({ captureRejections: true })

        this.headers = inner.headers()
        this.method = inner.method
        this.url = inner.url
        this.httpVersion = inner.version
        ;[this.httpVersionMajor, this.httpVersionMinor] = this.httpVersion
            .split(".")
            .map((s) => parseInt(s, 10))

        this.socket = inner.socket()
    }

    address() {
        return this.socket.remoteAddr
    }

    read(chunkSize: number = 1024): Buffer {
        const buffer: Buffer = this.inner.read(chunkSize)
        setImmediate(() => {
            this.emit("read", buffer)
            if (buffer.length < chunkSize) this.emit("end")
        })
        return buffer
    }

    destroy(error?: Error | null) {
        this.inner.close()
        if (error)
            setImmediate(() => {
                this.emit("error", error)
            })
    }
}
