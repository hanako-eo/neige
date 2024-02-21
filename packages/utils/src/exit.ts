const exit_events = [
    "SIGTERM",
    "SIGQUIT",
    "SIGINT",
    "SIGABRT",
    "uncaughtException",
    "exit",
] as const

export function on_exit(callback: () => void) {
    exit_events.forEach(event => {
        process.on(event, callback)
    })
}
