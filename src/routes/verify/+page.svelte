<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let message = "";
  let signature = "";
  let verification = "";
  let publicKey = "";
  let tokenStr = "";
  let stuff: HTMLTextAreaElement;

  interface Token {
    publicKey: string;
    signature: string;
    message: string;
  }

  function parseToken(str: string): Token {
    if (!str) str = "{}";
    let token = JSON.parse(str);
    return token;
  }
  $: output = JSON.stringify({ publicKey, signature, message });
  $: token = parseToken(tokenStr);

  async function verify() {
    verification = await invoke<string>("verify", token as {}).catch(
      (e) => (verification = e),
    );
  }
</script>

<textarea
  rows="8"
  class="border mt-10 w-full"
  placeholder="copy and paste into here"
  bind:this={stuff}
  bind:value={tokenStr}
  on:click={() => {
    stuff.select();
  }}
></textarea>
<form on:submit|preventDefault={verify}>
  <button type="submit">Verify Message</button>
</form>

<p>{verification}</p>
<div>
  <a href="/">Back</a>
</div>
