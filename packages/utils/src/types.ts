export interface Ref<T> {
    current: T | null
}

export function ref<T>(current?: T | null): Ref<T> {
    return { current: current ?? null }
}
