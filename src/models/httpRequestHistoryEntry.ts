import type { HttpRequest } from "./httpRequest";
import type { HttpResponse } from "./httpResponse";

// export class RawHttpRequestHistoryEntry {
//     constructor(
//     public id: string = "",
//     public timestamp: string = "",
//     public http_request: string = "",
//     public http_response: string | undefined = "",
//     ) {}
// }

export class ParsedHttpRequestHistoryEntry {
    constructor(
    public id: string = "",
    public timestamp: Date = new Date,
    public httpRequest: HttpRequest,
    public httpResponse: HttpResponse | undefined,
    ) {}

    // public static fromRaw(raw: RawHttpRequestHistoryEntry): ParsedHttpRequestHistoryEntry {
    //     return new ParsedHttpRequestHistoryEntry(
    //         raw.id,
    //         new Date(Date.parse(raw.timestamp)),
    //         JSON.parse(raw.http_request),
    //         JSON.parse(raw.http_response ?? "")
    //     )
    // }

}