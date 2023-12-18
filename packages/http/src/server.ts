import { type RustServer, create_server, close_server, launch_server } from "./_rust_server.js"
import { once_exit } from "@neige/utils/exit"

export default class Server {
    private inner_server: RustServer
    private force_event_loop = false

    constructor() {
        this.inner_server = create_server(() => { })
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
        once_exit(this.close)
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
