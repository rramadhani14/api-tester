<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { httpRequestStore } from "../lib/stores/httpRequestStore";
  import { httpResponseStore } from "$lib/stores/httpResponseStore";

  let selectedRequestTab = $state("HEADERS");
  let selectedResponseTab = $state("HEADERS");
  function parseHeaders(rawHeaders: string): Map<string, string> {
    if (!rawHeaders || rawHeaders.trim().length === 0) return new Map();
    const lines = rawHeaders.split("\n");
    return new Map(
      lines.map((it) => it.split(":", 2)).map((it) => [it[0], it[1]] as const),
    );
  }
  async function sendHttpRequest(event: Event) {
    // $inspect(httpRequestStore);

    let result: { body: string, headers: any[]} = await invoke("execute_http_request", {
      request: {
        method: $httpRequestStore.httpMethod,
        url: $httpRequestStore.url,
        headers: parseHeaders($httpRequestStore.headers),
        body: $httpRequestStore.body,
      },
    });
    httpResponseStore.updateBody(result.body);
    let parsedHeaders = Object.entries(result.headers).map(it => `${it[0]}: ${it[1]}`).join("\n");
    httpResponseStore.updateHeaders(parsedHeaders);
  }
</script>

<main class="w-full flex flex-col p-2">
  <h1>API Tester</h1>
  <div class="flex flex-row w-full">
    <div class="w-full">
      <form onsubmit={sendHttpRequest} class="flex flex-col">
        <div class="flex flex-col gap-2">
          <div class="flex flex-row w-full">
            <select
              bind:value={$httpRequestStore.httpMethod}
              class="border-2 rounded-l-sm px-2"
            >
              <option selected>GET</option>
              <option>POST</option>
              <option>PUT</option>
              <option>PATCH</option>
            </select>
            <input
              id="url-input"
              class="flex-1 border-y-2"
              placeholder="Enter a http url..."
              bind:value={$httpRequestStore.url}
            />
            <button class="border-2 rounded-r-sm px-4" type="submit"
              >SEND</button
            >
          </div>
          <div>
            <div class="flex flex-row">
              <button
                onclick={() => (selectedRequestTab = "HEADERS")}
                class={"min-w-12 border-t-2 border-l-2 rounded-tl-sm px-2 border-b-2" +
                  (selectedRequestTab === "HEADERS"
                    ? "border-b-transparent"
                    : "")}
              >
                headers
              </button>
              <button
                onclick={() => (selectedRequestTab = "BODY")}
                class={"min-w-12 border-t-2 border-l-2 border-r-2 rounded-tr-sm px-2 border-b-2" +
                  (selectedRequestTab === "BODY" ? "border-b-transparent" : "")}
              >
                body
              </button>
              <p class="flex-1 border-b-2"></p>
            </div>
            <div>
              {#if selectedRequestTab === "HEADERS"}
                <textarea
                  id="headers-input"
                  class="w-full resize-none border-2 border-t-0 rounded-b-sm"
                  rows="20"
                  placeholder="Enter a http headers..."
                  bind:value={$httpRequestStore.headers}
                ></textarea>
              {:else}
                <textarea
                  id="body-input"
                  class="w-full resize-none border-2 border-t-0 rounded-b-sm"
                  rows="20"
                  placeholder="Enter a http body..."
                  bind:value={$httpRequestStore.body}
                ></textarea>
              {/if}
            </div>
          </div>
          <div>
            <div class="flex flex-row">
              <button
                onclick={() => (selectedResponseTab = "HEADERS")}
                class={"min-w-12 border-t-2 border-l-2 rounded-tl-sm px-2 border-b-2" +
                  (selectedResponseTab === "HEADERS"
                    ? "border-b-transparent"
                    : "")}
              >
                headers
              </button>
              <button
                onclick={() => (selectedResponseTab = "BODY")}
                class={"min-w-12 border-t-2 border-l-2 border-r-2 rounded-tr-sm px-2 border-b-2" +
                  (selectedResponseTab === "BODY"
                    ? "border-b-transparent"
                    : "")}
              >
                body
              </button>
              <p class="flex-1 border-b-2"></p>
            </div>

            <div>
              {#if selectedResponseTab === "HEADERS"}
            <textarea
              class="w-full resize-none border-2 border-t-0 rounded-b-sm"
              rows="20"
              disabled
              bind:value={$httpResponseStore.headers}
            ></textarea>
              {:else}
            <textarea
              class="w-full resize-none border-2 border-t-0 rounded-b-sm"
              rows="20"
              disabled
              bind:value={$httpResponseStore.body}
            ></textarea>
              {/if}
            </div>
          </div>
        </div>
      </form>
    </div>
    <div></div>
  </div>
</main>

<style>
  /* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
} */
</style>
