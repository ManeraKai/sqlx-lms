<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import XIcon from "@lucide/svelte/icons/x";
  import { Input } from "$lib/components/ui/input/index.js";
  import type { Book, NewBook } from "$lib/types";

  let newBook: NewBook | null = $state(null);
  let {
    books,
    get_books,
    error = $bindable(),
  }: {
    books: Book[] | null;
    get_books: () => Promise<void>;
    error: String | null;
  } = $props();

  async function insertBook() {
    error = null;
    if (newBook === null) {
      newBook = { name: null };
      return;
    }
    const response = await fetch(`/api/book`, {
      method: "POST",
      body: JSON.stringify(newBook),
      headers: { "Content-Type": "application/json" },
    });
    if (response.status === 200) {
      newBook = null;
      await get_books();
    } else {
      error = await response.text();
    }
  }

  async function deleteBook(id: Number) {
    error = null;
    const response = await fetch(`/api/book/${id}`, {
      method: "DELETE",
    });
    if (response.status === 200) {
      await get_books();
    } else {
      error = await response.text();
    }
  }

    function discard() {
    newBook = null;
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
    {#each books as book}
      <Table.Row>
        <Table.Cell>{book.id}</Table.Cell>
        <Table.Cell>{book.name}</Table.Cell>
        <Table.Cell>
          <Button variant="ghost" onclick={() => deleteBook(book.id)}
            ><XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/each}
    {#if newBook}
      <Table.Row>
        <Table.Cell></Table.Cell>
        <Table.Cell><Input bind:value={newBook.name} /></Table.Cell>
        <Table.Cell>
          <Button variant="ghost" onclick={discard}
            ><XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/if}
  </Table.Body>
  <Button class="mt-2" onclick={insertBook}>
    {#if newBook}
      Submit
    {:else}
      New record
    {/if}
  </Button>
</Table.Root>
