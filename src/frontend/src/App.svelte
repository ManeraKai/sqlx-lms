<script lang="ts">
  import "./app.css";
  import BookTable from "$lib/components/BookTable.svelte";
  import TableSelect from "$lib/components/TableSelect.svelte";
  import CustomerTable from "$lib/components/CustomerTable.svelte";
  import type { Book, Customer, BorrowJoined } from "$lib/types";
  import BorrowTable from "$lib/components/BorrowTable.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import ClockIcon from "@lucide/svelte/icons/clock";

  import * as Alert from "$lib/components/ui/alert/index.js";
  import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";
  const tables = [
    { value: "book", label: "Books" },
    { value: "customer", label: "Customers" },
    { value: "borrow", label: "Borrows" },
  ];
  let error: String | null = $state(null);

  let table_name = $state(tables[0].value);

  let books: Book[] | null = $state(null);
  async function get_books() {
    const response = await fetch(`/api/book`);
    books = await response.json();
  }

  let customers: Customer[] | null = $state(null);

  async function get_customers() {
    const response = await fetch(`/api/customer`);
    customers = await response.json();
  }

  let borrows: BorrowJoined[] | null = $state(null);
  async function get_borrows() {
    const response = await fetch(`/api/borrow`);
    borrows = await response.json();
  }

  async function get_all() {
    await get_books();
    await get_borrows();
    await get_customers();
  }

  async function advance_in_time() {
    const response = await fetch("/api/borrow/advance_in_time", {
      method: "POST",
    });
    if (response.status == 200) {
      error = null;
      await get_borrows();
    } else {
      error = await response.text();
    }
  }
</script>

<main>
  <h1 class="text-center m-10">Library Management System</h1>
  <br />
  <div class="m-auto w-[800px]">
    {#if error !== null}
      <Alert.Root variant="destructive">
        <AlertCircleIcon />
        <Alert.Title>Database Error</Alert.Title>
        <Alert.Description>{error}</Alert.Description>
      </Alert.Root>
      <br/>
    {/if}
    <div class="flex justify-between">
      <TableSelect bind:table_name {tables} />
      <Button variant="outline" onclick={advance_in_time}>
        Advance In Time<ClockIcon />
      </Button>
    </div>
    {#if table_name == "book"}
      {#await get_books() then}
        <BookTable {books} {get_books} bind:error />
      {/await}
    {:else if table_name == "customer"}
      {#await get_customers() then}
        <CustomerTable {customers} {get_customers} bind:error />
      {/await}
    {:else if table_name == "borrow"}
      {#await get_all() then}
        <BorrowTable {borrows} {get_borrows} {books} {customers} bind:error />
      {/await}
    {/if}
  </div>
</main>
