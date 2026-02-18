import { writable } from "svelte/store";

export function createHttpResponseStore() {
    const { subscribe, update } = writable({
        body: "",
        headers: ""
    });

    return {
        subscribe,
        updateBody(body: string) {
            update(prev => ({...prev, body}))
        },
        updateHeaders(headers: string) {
            update(prev => ({...prev, headers}))
        }
    }
}

export const httpResponseStore = createHttpResponseStore();