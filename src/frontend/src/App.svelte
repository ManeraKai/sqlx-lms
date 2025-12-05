<script lang="ts">
  import "./app.css";
  import BookTable from "$lib/components/BookTable.svelte";
  import TableSelect from "$lib/components/TableSelect.svelte";

  type Book = {
    id: Number;
    name: String;
  };

  type NewBook = {
    name: String | null;
  };

  const tables = [
    {
      value: "book",
      label: "Books",
    },
    {
      value: "customer",
      label: "Customers",
    },
    {
      value: "borrow",
      label: "Borrows",
    },
  ];

  let open = $state(false);
  let table_name = $state(tables[0].value);
  let triggerRef = $state<HTMLButtonElement>(null!);
  let books: Book[] | null = $state(null);
  let newBook: NewBook | null = $state(null);

  const selectedValue = $derived(
    tables.find((f) => f.value === table_name)?.label
  );

  async function get_books() {
    const response = await fetch(`/api/book`);
    const records = await response.json();
    books = records;
  }
</script>

<main>
  <h1 class="text-center m-10">Library Management System</h1>
  <br />
  <div class="m-auto w-[500px]">
    <br /><br />
    <TableSelect
      bind:open
      {selectedValue}
      bind:table_name
      {tables}
      bind:triggerRef
    />
    {#if table_name == "book"}
      {#await get_books() then}
        <BookTable {books} {newBook} {get_books} />
      {/await}
    {/if}
  </div>
</main>
