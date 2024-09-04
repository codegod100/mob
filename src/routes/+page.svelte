<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  let secret = "";
  let stuff: HTMLTextAreaElement;
  let publicKey = "";
  onMount(async () => {
    [publicKey] = await invoke<[string, string]>("sign", {
      message: "",
    });
  });
</script>

<div class="p-5 text-center">
  <h1 class="mb-3">Sign and Verify Utility</h1>
  <div class="mb-5">
    <div>Your public key is <span class="border">{publicKey}</span></div>
    <div>If that's not right, import your secret key</div>
  </div>

  <div class="mb-5">
    <a href="/sign" class="border p-2 mb-10">Create a signature</a>
  </div>
  <div class="mb-5">
    <a href="/verify" class="border p-2">Verify a signature</a>
  </div>
  <div class="mb-3">
    <a href="/import" class="border p-2">Import Secret Key</a>
  </div>
  <div>
    <div>
      <button
        class="border p-2 mb-3"
        on:click={async () => {
          secret = await invoke("export");
        }}>Export Secret Key</button
      >
      <div class="flex justify-center">
        {#if secret}
          <textarea bind:this={stuff} class="border">{secret}</textarea>

          <button
            class="border p-2 ml-3"
            on:click={() => {
              stuff.select();
              document.execCommand("copy");
            }}>Copy to clipboard</button
          >
        {/if}
      </div>
    </div>
  </div>
</div>
