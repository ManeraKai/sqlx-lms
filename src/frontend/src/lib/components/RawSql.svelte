<script lang="ts">
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    let query: string = $state("");
    let msg: string = $state("");

    async function post() {
        if (!query) return;
        const response = await fetch(`/api/raw_sql`, {
            method: "POST",
            body: JSON.stringify({ query: query }),
            headers: { "Content-Type": "application/json" },
        });
        msg = await response.text();
    }
</script>

<Textarea bind:value={query} />
<Button class="my-2" onclick={post}>Run SQL</Button>
<Card.Root class="my-4 w-full">
    <Card.Header>
        <Card.Description class="whitespace-pre-line">{msg}</Card.Description>
    </Card.Header>
    <Card.Content></Card.Content>
</Card.Root>
<p class="border-b-black border-solid"></p>
