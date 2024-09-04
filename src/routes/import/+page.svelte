<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let key = "";
  let stuff: HTMLTextAreaElement;
</script>

<div class="p-5">
  <div class="flex">
    <textarea
      placeholder="Paste secret key"
      class="border"
      bind:value={key}
      bind:this={stuff}
    ></textarea>
    <button
      class="border p-2 ml-3"
      on:click={async () => {
        let result = await invoke("import", { key });
        console.log(result);
      }}>Import</button
    >
  </div>
  <div class="mt-5"><a href="/" class="border p-2">Back</a></div>
</div>
