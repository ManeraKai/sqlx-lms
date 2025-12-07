<script lang="ts">
  import "./app.css";
  import BookTable from "$lib/components/BookTable.svelte";
  import TableSelect from "$lib/components/TableSelect.svelte";
  import CustomerTable from "$lib/components/CustomerTable.svelte";
  import type {
    Book,
    NewBook,
    Customer,
    NewCustomer,
    NewBorrow,
    Borrow,
  } from "$lib/types";
  import BorrowTable from "$lib/components/BorrowTable.svelte";

  const tables = [
    { value: "book", label: "Books" },
    { value: "customer", label: "Customers" },
    { value: "borrow", label: "Borrows" },
  ];

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

  let borrows: Borrow[] | null = $state(null);
  async function get_borrows() {
    const response = await fetch(`/api/borrow`);
    borrows = await response.json();
  }

  async function get_all() {
    await get_books();
    await get_borrows();
    await get_customers();
  }
</script>

<main>
  <h1 class="text-center m-10">Library Management System</h1>
  <br />
  <div class="m-auto w-[800px]">
    <div>
      <TableSelect bind:table_name {tables} />
    </div>
    {#if table_name == "book"}
      {#await get_books() then}
        <BookTable {books} {get_books} />
      {/await}
    {:else if table_name == "customer"}
      {#await get_customers() then}
        <CustomerTable {customers} {get_customers} />
      {/await}
    {:else if table_name == "borrow"}
      {#await get_all() then}
        <BorrowTable {borrows} {get_borrows} {books} {customers} />
      {/await}
    {/if}
  </div>
</main>
