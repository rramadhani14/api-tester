export enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}
export class HttpRequest {
    constructor(
        public method: HttpMethod = HttpMethod.GET,
        public url: string = "",
        public body: string = "",
        public headers: Map<String, String> = new Map()
    ) {}
}