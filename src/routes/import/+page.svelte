<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Store } from "@tauri-apps/plugin-store";
    const store = new Store("store.bin");
    let key = "";
    let result = "";
    let stuff: HTMLTextAreaElement;
    async function imp(): Promise<string> {
        await store.set("key", key);
        await store.save();

        return invoke<string>("import", { key }).catch((e) => {
            return e;
        });
    }
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
                result = await imp();
            }}>Import</button
        >
    </div>
    <div>{result}</div>
    <div class="mt-5"><a href="/" class="border p-2">Back</a></div>
</div>
