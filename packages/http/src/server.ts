import {
    type RustServer,
    create_server,
    close_server,
    launch_server,
    set_pool_capacity,
    set_obstruction,
    get_pool_capacity,
    get_obstruction
} from "./_rust_server.js"
import { once_exit } from "@neige/utils/exit"

export default class Server {
    private inner_server: RustServer
    private force_event_loop = false

    constructor() {
        this.inner_server = create_server(() => { })
        once_exit(this.close)
    }

    public get_pool_capacity(): number {
        return get_pool_capacity(this.inner_server)
    }

    public set_pool_capacity(pool: number) {
        set_pool_capacity(this.inner_server, pool)
    }

    public get_obstruction(): boolean {
        return get_obstruction(this.inner_server)
    }

    public set_obstruction(obstruct: boolean) {
        set_obstruction(this.inner_server, obstruct)
    }

    public launch_on(port: number) {
        // forces the nodejs event loop to stay alive, I'm not really happy
        // with this solution but as I can't really find any other elegant
        // solution that doesn't that doesn't clutter up the event loop unlike
        // setTimeout or setInterval
        if (!this.inner_server.obstruct) {
            process.stdin.resume()
            this.force_event_loop = true
        }

        // forces the server to close correctly if the process exits
        launch_server(this.inner_server, port)
    }

    public close() {
        close_server(this.inner_server)

        // close correctly the event loop 
        if (this.force_event_loop) {
            process.stdin.pause();
            this.force_event_loop = false
        }
    }
}
