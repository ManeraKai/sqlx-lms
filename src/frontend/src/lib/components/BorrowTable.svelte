<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import XIcon from "@lucide/svelte/icons/x";
  import { Input } from "$lib/components/ui/input/index.js";
  import type { Book, Borrow, Customer, NewBorrow } from "$lib/types";
  import BookSelect from "./BookSelect.svelte";
  import CustomerSelect from "./CustomerSelect.svelte";

  let newBorrow: NewBorrow | null = $state(null);

  let {
    borrows,
    get_borrows,
    books,
    customers,
  }: {
    borrows: Borrow[] | null;
    get_borrows: () => Promise<void>;
    books: Book[] | null;
    customers: Customer[] | null;
  } = $props();

  async function insertBorrow() {
    if (newBorrow === null) {
      newBorrow = { book_id: null, customer_id: null, duration: null };
      return;
    }
    const response = await fetch(`/api/borrow`, {
      method: "POST",
      body: JSON.stringify(newBorrow),
      headers: { "Content-Type": "application/json" },
    });
    if (response.status === 200) {
      newBorrow = null;
      await get_borrows();
    }
  }

  async function deleteBorrow(id: Number) {
    const response = await fetch(`/api/borrow/${id}`, {
      method: "DELETE",
    });
    if (response.status === 200) {
      await get_borrows();
    }
  }
</script>

<Table.Root class="w-[800px]">
  <Table.Header>
    <Table.Row>
      <Table.Head>ID</Table.Head>
      <Table.Head>Book ID</Table.Head>
      <Table.Head>Customer ID</Table.Head>
      <Table.Head>Duration (days)</Table.Head>
    </Table.Row>
  </Table.Header>
  <Table.Body>
    {#each borrows as borrow}
      <Table.Row>
        <Table.Cell>{borrow.id}</Table.Cell>
        <Table.Cell>{borrow.book_id}</Table.Cell>
        <Table.Cell>{borrow.customer_id}</Table.Cell>
        <Table.Cell>{borrow.duration}</Table.Cell>

        <Table.Cell>
          <Button variant="ghost" onclick={() => deleteBorrow(borrow.id)}
            ><XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/each}
    {#if newBorrow}
      <Table.Row>
        <Table.Cell></Table.Cell>
        <Table.Cell>
          <BookSelect {books} bind:book_id={newBorrow.book_id} />
        </Table.Cell>
        <Table.Cell>
          <CustomerSelect {customers} bind:customer_id={newBorrow.customer_id} />
        </Table.Cell>
        <Table.Cell>
          <Input bind:value={newBorrow.duration} />
        </Table.Cell>
        <Table.Cell>
          <Button variant="ghost" onclick={() => (newBorrow = null)}>
            <XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/if}
  </Table.Body>
  <Button class="mt-2" onclick={insertBorrow}>
    {#if newBorrow}
      Submit
    {:else}
      New record
    {/if}
  </Button>
</Table.Root>
