<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import XIcon from "@lucide/svelte/icons/x";
  import { Input } from "$lib/components/ui/input/index.js";

  let { newBook, books, get_books } = $props();

  async function insertBook() {
    if (newBook === null) {
      newBook = { name: null };
      return;
    }
    const response = await fetch(`/api/book`, {
      method: "POST",
      body: JSON.stringify(newBook),
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (response.status === 200) {
      newBook = null;
      await get_books();
    }
  }

  async function deleteBook(id: Number) {
    const response = await fetch(`/api/book/${id}`, {
      method: "DELETE",
    });
    if (response.status === 200) {
      await get_books();
    }
  }
</script>

<Table.Root>
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
          <Button variant="ghost" onclick={() => (newBook = null)}
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
