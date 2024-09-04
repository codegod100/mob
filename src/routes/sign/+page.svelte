<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let message = "";
  let publicKey = "";
  let signature = "";
  let stuff: HTMLTextAreaElement;
  let signed = false;
  $: output = JSON.stringify({ publicKey, signature, message });
  async function sign() {
    signed = true;

    [publicKey, signature] = await invoke("sign", { message });
  }
</script>

<div>
  <form on:submit|preventDefault={sign}>
    <input
      class="border"
      placeholder="Enter a message..."
      bind:value={message}
      on:input={() => {
        if (stuff) stuff.value = "";
      }}
    />
    <button type="submit">Sign Message</button>
  </form>
</div>
<div class="flex">
  {#if signed}
    <div>
      <textarea rows="8" cols="40" bind:this={stuff} class="border">
        {output}
      </textarea>
    </div>
    <div>
      <button
        on:click={() => {
          stuff.select();
          document.execCommand("copy");
        }}>Copy to clipboard</button
      >
    </div>
  {/if}
</div>

<div>
  <a href="/">Back</a>
</div>
