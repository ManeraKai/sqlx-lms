<script lang="ts">
    import * as Table from "$lib/components/ui/table/index.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import XIcon from "@lucide/svelte/icons/x";
    import { Input } from "$lib/components/ui/input/index.js";
    import type { Customer, NewCustomer } from "$lib/types";
    import RecordButtons from "./RecordButtons.svelte";

    let page: number = 0;

    let newRecord: NewCustomer | null = $state(null);
    let editedRecord: NewCustomer | null = $state(null);
    let {
        customer: record,
        get_customer,
        error = $bindable(),
    }: {
        customer: Customer | null;
        get_customer: (page?: number) => Promise<void>;
        error: String | null;
    } = $props();

    async function nextRecord() {
        error = null;
        if (record) page++;
        await get_customer(page);
    }

    async function previousRecord() {
        error = null;
        if (page > 0) page--;
        await get_customer(page);
    }

    async function insert() {
        error = null;
        if (newRecord === null) {
            newRecord = { name: null, age: null, sex: null, crimes: "0" };
            return;
        }
        const response = await fetch(`/api/customer`, {
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
        if (editedRecord === null && record) {
            editedRecord = {
                name: record.name,
                age: record.age.toString(),
                sex: record.sex.toString(),
                crimes: record.crimes.toString(),
            };
            return;
        }
        const response = await fetch(`/api/customer/${record.id}`, {
            method: "POST",
            body: JSON.stringify(editedRecord),
            headers: { "Content-Type": "application/json" },
        });
        console.log(response.status);
        if (response.status === 200) {
            editedRecord = null;
            await get_customer(page);
        } else {
            error = await response.text();
        }
    }

    async function remove() {
        if (record === null) return;
        error = null;
        const response = await fetch(`/api/customer/${record.id}`, {
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
            <Table.Head>Age</Table.Head>
            <Table.Head>Sex</Table.Head>
            <Table.Head>Crimes</Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#if record && !newRecord && !editedRecord}
            <Table.Row>
                <Table.Cell>{record.id}</Table.Cell>
                <Table.Cell>{record.name}</Table.Cell>
                <Table.Cell>{record.age}</Table.Cell>
                <Table.Cell>{record.sex}</Table.Cell>
                <Table.Cell>{record.crimes}</Table.Cell>
            </Table.Row>
        {:else if editedRecord && record}
            <Table.Row>
                <Table.Cell>{record.id}</Table.Cell>
                <Table.Cell>
                    <Input bind:value={editedRecord.name} />
                </Table.Cell>
                <Table.Cell>
                    <Input bind:value={editedRecord.age} />
                </Table.Cell>
                <Table.Cell>
                    <Input bind:value={editedRecord.sex} />
                </Table.Cell>
                <Table.Cell>
                    <Input bind:value={editedRecord.crimes} />
                </Table.Cell>
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
                    <Input bind:value={newRecord.age} />
                </Table.Cell>
                <Table.Cell>
                    <Input bind:value={newRecord.sex} />
                </Table.Cell>
                <Table.Cell />
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
