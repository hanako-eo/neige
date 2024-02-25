import { ref } from "./types"

const exit_events = ["SIGTERM", "SIGQUIT", "SIGINT", "SIGABRT", "exit"] as const

export function on_exit(callback: Function) {
    const exit_ref = ref<Function>(callback)
    exit_events.forEach((event) => {
        process.once(event, () => {
            if (exit_ref.current !== null) {
                exit_ref.current()
                exit_ref.current = null
                process.exit(process.exitCode ?? 1)
            }
        })
    })
}
