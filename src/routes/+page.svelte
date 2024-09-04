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

  <div><a href="/sign">Create a signature</a></div>
  <div><a href="/verify">Verify a signature</a></div>
  <div><a href="/import">Import Secret Key</a></div>
  <div>
    <div>
      <button
        on:click={async () => {
          secret = await invoke("export");
        }}>Export Secret Key</button
      >
    </div>
    <div>
      {#if secret}<textarea bind:this={stuff} class="border">{secret}</textarea
        ><button
          on:click={() => {
            stuff.select();
            document.execCommand("copy");
          }}>Copy to clipboard</button
        >{/if}
    </div>
  </div>
</div>
