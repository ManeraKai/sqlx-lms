<script lang="ts">
    import * as Table from "$lib/components/ui/table/index.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import XIcon from "@lucide/svelte/icons/x";
    import { Input } from "$lib/components/ui/input/index.js";
    import type {
        Book,
        Borrow,
        BorrowJoined,
        Customer,
        NewBorrow,
    } from "$lib/types";
    import BookSelect from "./BookSelect.svelte";
    import CustomerSelect from "./CustomerSelect.svelte";
    import RecordButtons from "../RecordButtons.svelte";

    let page: number = 0;

    let newRecord: NewBorrow | null = $state(null);
    let editedRecord: Borrow | null = $state(null);
    let {
        borrow: record,
        get_borrow,
        error = $bindable(),
    }: {
        borrow: BorrowJoined | null;
        get_borrow: (page?: number) => Promise<void>;
        error: String | null;
    } = $props();

    async function nextRecord() {
        error = null;
        if (record) page++;
        await get_borrow(page);
    }

    async function previousRecord() {
        error = null;
        if (page > 0) page--;
        await get_borrow(page);
    }

    async function insert() {
        error = null;
        if (newRecord === null) {
            newRecord = { book_id: null, customer_id: null, duration: null };
            return;
        }
        const response = await fetch(`/api/borrow`, {
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
                id: record.id,
                book_id: record.book_id.toString(),
                customer_id: record.customer_id.toString(),
                duration: record.duration.toString(),
            };
            return;
        }
        const response = await fetch(`/api/borrow/${record.id}`, {
            method: "POST",
            body: JSON.stringify(editedRecord),
            headers: { "Content-Type": "application/json" },
        });
        if (response.status === 200 && record) {
            editedRecord = null;
            await get_borrow(page);
        } else {
            error = await response.text();
        }
    }

    async function remove() {
        if (record === null) return;
        error = null;
        const response = await fetch(`/api/borrow/${record.id}`, {
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
    let all_books: Book[] | null = $state(null);
    let all_customers: Customer[] | null = $state(null);
    async function get_all_selects() {
        const book_resp = await fetch(`/api/book?limit=100`);
        all_books = await book_resp.json();
        console.log("all_books", all_books);

        const customer_resp = await fetch(`/api/customer?limit=100`);
        all_customers = await customer_resp.json();
        console.log("all_customers", all_customers);
    }
</script>

<Table.Root class="w-[500px]">
    <Table.Header>
        <Table.Row>
            <Table.Head>ID</Table.Head>
            <Table.Head>Book</Table.Head>
            <Table.Head>Customer</Table.Head>
            <Table.Head>Duration (days)</Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#if record && !newRecord && !editedRecord}
            <Table.Row>
                <Table.Cell>{record.id}</Table.Cell>
                <Table.Cell>{record.book_name}</Table.Cell>
                <Table.Cell>{record.customer_name}</Table.Cell>
                <Table.Cell>{record.duration}</Table.Cell>
            </Table.Row>
        {:else if editedRecord}
            {#await get_all_selects() then}
                {#if all_books && all_customers}
                    <Table.Row>
                        <Table.Cell>{editedRecord.id}</Table.Cell>
                        <Table.Cell>
                            <BookSelect
                                books={all_books}
                                bind:book_id={editedRecord.book_id}
                            />
                        </Table.Cell>
                        <Table.Cell>
                            <CustomerSelect
                                customers={all_customers}
                                bind:customer_id={editedRecord.customer_id}
                            />
                        </Table.Cell>
                        <Table.Cell
                            ><Input
                                bind:value={editedRecord.duration}
                            /></Table.Cell
                        >
                        <Table.Cell>
                            <Button variant="ghost" onclick={discard}>
                                <XIcon />
                            </Button>
                        </Table.Cell>
                    </Table.Row>
                {/if}
            {/await}
        {:else if newRecord}
            {#await get_all_selects() then}
                {#if all_books && all_customers}
                    <Table.Row>
                        <Table.Cell></Table.Cell>
                        <Table.Cell>
                            <BookSelect
                                books={all_books}
                                bind:book_id={newRecord.book_id}
                            />
                        </Table.Cell>
                        <Table.Cell>
                            <CustomerSelect
                                customers={all_customers}
                                bind:customer_id={newRecord.customer_id}
                            />
                        </Table.Cell>
                        <Table.Cell>
                            <Input bind:value={newRecord.duration} />
                        </Table.Cell>
                        <Table.Cell>
                            <Button variant="ghost" onclick={discard}>
                                <XIcon />
                            </Button>
                        </Table.Cell>
                    </Table.Row>
                {/if}
            {/await}
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
