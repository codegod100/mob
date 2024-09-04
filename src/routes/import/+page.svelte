<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let key = "";
  let stuff: HTMLTextAreaElement;
</script>

<div>
  <div>
    <textarea
      placeholder="Paste secret key"
      class="border"
      bind:value={key}
      bind:this={stuff}
    ></textarea>
  </div>
  <div>
    <button
      on:click={async () => {
        let result = await invoke("import", { key });
        console.log(result);
      }}>Import Secret Key</button
    >
  </div>
</div>
<div><a href="/">Back</a></div>
