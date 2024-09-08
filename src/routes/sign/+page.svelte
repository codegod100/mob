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

<div class="p-5">
    <form on:submit|preventDefault={sign}>
        <input
            class="border p-2"
            placeholder="Enter a message..."
            bind:value={message}
            on:input={() => {
                if (stuff) stuff.value = "";
            }}
        />
        <button type="submit" class="border p-2 ml-2">Sign Message</button>
    </form>
    <div class=" mt-5">
        {#if signed}
            <div>
                <textarea rows="8" cols="40" bind:this={stuff} class="border">
                    {output}
                </textarea>
            </div>
            <div>
                <button
                    class="border p-2"
                    on:click={() => {
                        stuff.select();
                        document.execCommand("copy");
                    }}>Copy to clipboard</button
                >
            </div>
        {/if}
    </div>

    <div class="mt-5">
        <a href="/" class="border p-2">Back</a>
    </div>
</div>
