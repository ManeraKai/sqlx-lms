<script lang="ts">
    import * as Table from "$lib/components/ui/table/index.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import XIcon from "@lucide/svelte/icons/x";
    import { Input } from "$lib/components/ui/input/index.js";
    import type { Book, NewBook } from "$lib/types";
    import RecordButtons from "./RecordButtons.svelte";

    let page: number = 0;

    let newRecord: NewBook | null = $state(null);
    let editedRecord: NewBook | null = $state(null);
    let {
        book: record,
        get_book,
        error = $bindable(),
    }: {
        book: Book | null;
        get_book: (page?: number) => Promise<void>;
        error: String | null;
    } = $props();

    async function nextRecord() {
        error = null;
        if (record) page++;
        await get_book(page);
    }

    async function previousRecord() {
        error = null;
        if (page > 0) page--;
        await get_book(page);
    }

    async function insert() {
        error = null;
        if (newRecord === null) {
            newRecord = { name: null };
            return;
        }
        const response = await fetch(`/api/book`, {
            method: "POST",
            body: JSON.stringify(newRecord),
            headers: { "Content-Type": "application/json" },
        });
        if (response.status === 200) {
            newRecord = null;
            await nextRecord();
        } else {
            error = await response.text();
        }
    }

    async function edit() {
        if (record === null) return;
        error = null;
        if (editedRecord === null) {
            editedRecord = record;
            return;
        }
        const response = await fetch(`/api/book/${record.id}`, {
            method: "POST",
            body: JSON.stringify(editedRecord),
            headers: { "Content-Type": "application/json" },
        });
        if (response.status === 200) {
            editedRecord = null;
            await get_book(page);
        } else {
            error = await response.text();
        }
    }

    async function remove() {
        if (record === null) return;
        error = null;
        const response = await fetch(`/api/book/${record.id}`, {
            method: "DELETE",
        });
        if (response.status === 200) {
            await previousRecord();
        } else {
            error = await response.text();
        }
    }

    function discard() {
        newRecord = null;
        editedRecord = null;
        error = null;
    }
</script>

<Table.Root class="w-[500px]">
    <Table.Header>
        <Table.Row>
            <Table.Head>ID</Table.Head>
            <Table.Head>Name</Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#if record && !newRecord && !editedRecord}
            <Table.Row>
                <Table.Cell>{record.id}</Table.Cell>
                <Table.Cell>{record.name}</Table.Cell>
            </Table.Row>
        {:else if record && editedRecord}
            <Table.Row>
                <Table.Cell>{record.id}</Table.Cell>
                <Table.Cell><Input bind:value={editedRecord.name} /></Table.Cell
                >
                <Table.Cell>
                    <Button variant="ghost" onclick={discard}>
                        <XIcon />
                    </Button>
                </Table.Cell>
            </Table.Row>
        {:else if newRecord}
            <Table.Row>
                <Table.Cell></Table.Cell>
                <Table.Cell><Input bind:value={newRecord.name} /></Table.Cell>
                <Table.Cell>
                    <Button variant="ghost" onclick={discard}>
                        <XIcon />
                    </Button>
                </Table.Cell>
            </Table.Row>
        {:else}
            <Table.Row>
                <Table.Cell colspan={4}>...</Table.Cell>
            </Table.Row>
        {/if}
    </Table.Body>
</Table.Root>
<RecordButtons
    {record}
    {newRecord}
    {editedRecord}
    {previousRecord}
    {nextRecord}
    {insert}
    {edit}
    {remove}
/>
