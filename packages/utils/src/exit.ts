const exit_events = [
    "SIGSTOP",
    "SIGQUIT",
    "SIGKILL",
    "SIGINT",
    "SIGABRT",
    "exit",
] as const

export function once_exit(callback: () => void) {
    exit_events.forEach(event => {
        process.once(event, callback)
    })
}
