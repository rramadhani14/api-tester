import { writable } from "svelte/store";
import type { ParsedHttpRequestHistoryEntry } from "../../models/httpRequestHistoryEntry";




export function createHttpRequestHistoryStore() {
    const { subscribe, update } = writable<ParsedHttpRequestHistoryEntry[]>([]);

    return {
        subscribe,
        unshiftEntry(entry: ParsedHttpRequestHistoryEntry) {
            update(prev => [entry, ...prev])
        },
        shiftEntry() {
            update(prev => prev.splice(1))
        }
    }
}

export const httpRequestHistoryStore = createHttpRequestHistoryStore();