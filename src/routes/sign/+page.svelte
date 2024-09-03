<script>
  import { invoke } from "@tauri-apps/api/core";
  let message = "";
  let publicKey = "";
  let signature = "";
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
      id="sign-input"
      placeholder="Enter a message..."
      bind:value={message}
      on:input={() => {
        console.log("changing");
        let field = document.getElementById("stuff");
        if (field) field.value = "";
      }}
    />
    <button type="submit">Sign Message</button>
  </form>
</div>
<div class="flex">
  {#if signed}
    <div>
      <textarea rows="8" cols="40" id="stuff" class="border">
        {output}
      </textarea>
    </div>
    <div>
      <button
        on:click={() => {
          let copyText = document.querySelector("#stuff");
          copyText.select();
          document.execCommand("copy");
        }}>Copy to clipboard</button
      >
    </div>
  {/if}
</div>

<div>
  <a href="/">Back</a>
</div>
