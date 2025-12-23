<script lang="ts">
    import "./app.css";
    import BookTable from "$lib/components/BookTable.svelte";
    import TableSelect from "$lib/components/TableSelect.svelte";
    import CustomerTable from "$lib/components/CustomerTable.svelte";
    import type { Book, Customer, BorrowJoined } from "$lib/types";
    import BorrowTable from "$lib/components/Borrow/BorrowTable.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    import * as Alert from "$lib/components/ui/alert/index.js";
    import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";
    import RawSql from "$lib/components/RawSql.svelte";
    const tables = [
        { value: "book", label: "Books" },
        { value: "customer", label: "Customers" },
        { value: "borrow", label: "Borrows" },
    ];
    let error: String | null = $state(null);

    let table_name = $state(tables[0].value);

    let book: Book | null = $state(null);
    async function get_book(page?: number) {
        const response = await fetch(`/api/book?page=${page ?? 0}&limit=1`);
        const res: Book[] = await response.json();
        if (res.length > 0) book = res[0];
        else book = null;
    }

    let customer: Customer | null = $state(null);
    async function get_customer(page?: number) {
        const response = await fetch(`/api/customer?page=${page ?? 0}&limit=1`);
        const res: Customer[] = await response.json();
        if (res.length > 0) customer = res[0];
        else customer = null;
    }

    let borrow: BorrowJoined | null = $state(null);
    async function get_borrow(page?: number) {
        const response = await fetch(`/api/borrow?page=${page ?? 0}&limit=1`);
        borrow = await response.json();
    }

    async function get_all() {
        await get_book();
        await get_borrow();
        await get_customer();
    }
</script>

<main>
    <h1 class="text-center m-10">Library Management System</h1>
    <br />
    <div class="flex m-auto w-[1600px]">
        <div class="w-[50%]">
            <div class="mr-10">
                {#if error !== null}
                    <Alert.Root variant="destructive">
                        <AlertCircleIcon />
                        <Alert.Description>{error}</Alert.Description>
                    </Alert.Root>
                    <br />
                {/if}
                <div class="flex justify-between mb-2">
                    <TableSelect bind:table_name {tables} />
                </div>
                {#if table_name == "book"}
                    {#await get_book() then}
                        <BookTable {book} {get_book} bind:error />
                    {/await}
                {:else if table_name == "customer"}
                    {#await get_customer() then}
                        <CustomerTable {customer} {get_customer} bind:error />
                    {/await}
                {:else if table_name == "borrow"}
                    {#await get_all() then}
                        <BorrowTable {borrow} {get_borrow} bind:error />
                    {/await}
                {/if}
            </div>
            <div class="mt-20">
                <RawSql />
            </div>
        </div>
        <Card.Root class="ml-10 w-[40%]">
            <Card.Header>
                <Card.Description class="whitespace-pre-line">
                    <pre>
CREATE TABLE IF NOT EXISTS book(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS customer(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    sex INTEGER NOT NULL CHECK(sex IN (0, 1, 2, 9)) -- ISO/IEC 5218
);
CREATE TABLE IF NOT EXISTS borrow(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER NOT NULL UNIQUE,
    customer_id INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    FOREIGN KEY(book_id) REFERENCES book(id),
    FOREIGN KEY(customer_id) REFERENCES customer(id)
);
            </pre>
                </Card.Description>
            </Card.Header>
            <Card.Content></Card.Content>
        </Card.Root>
    </div>
</main>
