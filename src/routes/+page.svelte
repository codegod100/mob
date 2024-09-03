<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let message = "";
  let signature = "";
  let verification = "";
  let publicKey = "";
  let tokenStr = "";

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

  async function sign() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    [publicKey, signature] = await invoke("sign", { message });
  }
  async function verify() {
    verification = await invoke<string>("verify", token as {}).catch(
      (e) => (verification = e),
    );
  }
</script>

<div class="container">
  <h1>Welcome to Tauri!</h1>

  <form class="row" on:submit|preventDefault={sign}>
    <input
      id="sign-input"
      placeholder="Enter a message..."
      bind:value={message}
    />
    <button type="submit">Sign Message</button>
  </form>
  <div id="output">
    {output}
  </div>
  <textarea
    rows="8"
    placeholder="copy and paste into here"
    bind:value={tokenStr}
  ></textarea>
  <form id="inputs" on:submit|preventDefault={verify}>
    <button type="submit">Verify Message</button>
  </form>

  <p>{verification}</p>
</div>

<style>
  #output {
    width: 100%;
    text-align: left;
    white-space: pre-wrap;
    overflow-wrap: break-word;
  }
  #inputs {
    display: block;
  }

  pre {
    display: inline-block;
    width: 600px;
    text-align: left;
  }

  .logo.vite:hover {
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
  }
</style>
