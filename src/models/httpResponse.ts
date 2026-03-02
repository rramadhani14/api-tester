export class HttpResponse {
  constructor(
    public status_code: number = 999,
    public body: string = "",
    public headers: Map<String, String> = new Map(),
  ) {}
}
