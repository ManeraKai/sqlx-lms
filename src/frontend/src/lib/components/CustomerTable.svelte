<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import XIcon from "@lucide/svelte/icons/x";
  import { Input } from "$lib/components/ui/input/index.js";
  import type { Customer, NewCustomer } from "$lib/types";

  let newCustomer: NewCustomer | null = $state(null);
  let {
    customers,
    get_customers,
  }: {
    customers: Customer[] | null;
    get_customers: () => Promise<void>;
  } = $props();

  async function insertCustomer() {
    if (newCustomer === null) {
      newCustomer = { name: null, age: null, sex: null };
      return;
    }
    const response = await fetch(`/api/customer`, {
      method: "POST",
      body: JSON.stringify(newCustomer),
      headers: { "Content-Type": "application/json" },
    });
    if (response.status === 200) {
      newCustomer = null;
      await get_customers();
    }
  }

  async function deleteCustomer(id: Number) {
    const response = await fetch(`/api/customer/${id}`, {
      method: "DELETE",
    });
    if (response.status === 200) {
      await get_customers();
    }
  }
</script>

<Table.Root class="w-[800px]">
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
    {#each customers as customer}
      <Table.Row>
        <Table.Cell>{customer.id}</Table.Cell>
        <Table.Cell>{customer.name}</Table.Cell>
        <Table.Cell>{customer.age}</Table.Cell>
        <Table.Cell>{customer.sex}</Table.Cell>
        <Table.Cell>{customer.crimes}</Table.Cell>
        <Table.Cell>
          <Button variant="ghost" onclick={() => deleteCustomer(customer.id)}
            ><XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/each}
    {#if newCustomer}
      <Table.Row>
        <Table.Cell></Table.Cell>
        <Table.Cell>
          <Input bind:value={newCustomer.name} />
        </Table.Cell>
        <Table.Cell>
          <Input bind:value={newCustomer.age} />
        </Table.Cell>
        <Table.Cell>
          <Input bind:value={newCustomer.sex} />
        </Table.Cell>
        <Table.Cell></Table.Cell>
        <Table.Cell>
          <Button variant="ghost" onclick={() => (newCustomer = null)}>
            <XIcon />
          </Button>
        </Table.Cell>
      </Table.Row>
    {/if}
  </Table.Body>
  <Button class="mt-2" onclick={insertCustomer}>
    {#if newCustomer}
      Submit
    {:else}
      New record
    {/if}
  </Button>
</Table.Root>
