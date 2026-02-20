<script lang="ts">
  import { stopPropagation } from "svelte/legacy";

    let { class: klass = "", mode = "HEADERS", store, disabled } = $props();
    function toggleMode(m: string) {
      mode = m;
    }
</script>
<div class={"flex flex-col " + klass}>
            <div class="flex flex-row">
              <button
              type="button"
                onclick={(e) => {toggleMode("HEADERS")}}
                class={"min-w-12 border-t-2 border-l-2 rounded-tl-sm px-2 border-b-2" +
                  (mode === "HEADERS"
                    ? "border-b-transparent"
                    : "")}
              >
                headers
              </button>
              <button
              type="button"
                onclick={(e) => {toggleMode("BODY")}}
                class={"min-w-12 border-t-2 border-l-2 border-r-2 rounded-tr-sm px-2 border-b-2" +
                  (mode === "BODY" ? "border-b-transparent" : "")}
              >
                body
              </button>
              <p class="flex-1 border-b-2"></p>
            </div>
            <div class="flex flex-1">
              {#if mode === "HEADERS"}
                <textarea
                  id="headers-input"
                  class="flex-1 w-full resize-none border-2 border-t-0 rounded-b-sm"
                  {disabled}
                  placeholder=""
                  bind:value={$store.headers}
                ></textarea>
              {:else}
                <textarea
                  id="body-input"
                  class="flex-1 w-full resize-none border-2 border-t-0 rounded-b-sm"
                  {disabled}
                  placeholder=""
                  bind:value={$store.body}
                ></textarea>
              {/if}
            </div>
          </div>