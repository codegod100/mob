<script>
  import { invoke } from "@tauri-apps/api/core";
  let key = "";
</script>

<div>
  <div>
    <textarea
      id="stuff"
      placeholder="Paste secret key"
      class="border"
      bind:value={key}
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
