<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Store } from "@tauri-apps/plugin-store";
    let secret = "";
    let stuff: HTMLTextAreaElement;
    let publicKey = "";
    async function loadKey() {
        [publicKey] = await invoke<[string, string]>("sign", {
            message: "",
        });
    }
    onMount(async () => {
        await loadKey();
    });
</script>

<div class="p-5 text-center">
    <h1 class="mb-3 text-3xl">
        <div>Ed25519</div>
        <div>Sign and Verify Utility</div>
    </h1>
    <div class="mb-5">
        <div>
            <div>Your public key is:</div>
            <div>
                <textarea class="border" rows="3" readonly>{publicKey}</textarea
                >
            </div>
            <div>If that's not right, import your secret key</div>
        </div>
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
                class="border p-2 mb-2"
                on:click={async () => {
                    secret = await invoke("export");
                }}>Export Secret Key</button
            >
            <div class="justify-center">
                {#if secret}
                    <div class="flex">
                        <textarea bind:this={stuff} class="border mt-1"
                            >{secret}</textarea
                        >

                        <button
                            class="border p-2 ml-3"
                            on:click={() => {
                                stuff.select();
                                document.execCommand("copy");
                            }}>Copy to clipboard</button
                        >
                    </div>
                    <div class="mb-2"></div>
                {/if}
            </div>
        </div>
        <div>
            <button
                class="border p-2"
                on:click={async () => {
                    const store = new Store("store.bin");
                    await store.delete("key");
                    await store.save();
                    await invoke("reset");
                    await loadKey();
                }}>Reset Secret Key</button
            >
        </div>
    </div>
</div>
