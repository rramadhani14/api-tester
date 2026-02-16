import { writable } from "svelte/store";

export function createHttpRequestStore() {
    const { subscribe, update } = writable({
        httpMethod: "",
        url: "",
        body: "",
        headers: ""
    });

    return {
        subscribe,
        updateHttpMethod(httpMethod: string) {
            update(prev => ({...prev, httpMethod}))
        },
        updateUrl(url: string) {
            update(prev => ({...prev, url}))
        },
        updateBody(body: string) {
            update(prev => ({...prev, body}))
        },
        updateHeaders(headers: string) {
            update(prev => ({...prev, headers}))
        }
    }
}

export const httpRequestStore = createHttpRequestStore();