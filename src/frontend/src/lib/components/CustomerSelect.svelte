<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import type { Book } from "$lib/types";
  import { cn } from "$lib/utils.js";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";

  let open = $state(false);

  let {
    customer_id = $bindable(),
    customers,
  }: {
    customer_id: String | null;
    customers: Book[] | null;
  } = $props();

  const selectedValue = $derived(
    customers && customers.find((f) => f.id.toString() === customer_id)?.name
  );
</script>

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        variant="outline"
        class="w-[200px] justify-between"
        {...props}
        role="combobox"
        aria-expanded={open}
      >
        {selectedValue || "Select a Customer..."}
        <ChevronsUpDownIcon class="ms-2 size-4 shrink-0 opacity-50" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-[200px] p-0">
    <Command.Root>
      <Command.List>
        <Command.Group>
          {#each customers as customer}
            <Command.Item
              value={customer.id.toString()}
              onSelect={() => {
                customer_id = customer.id.toString();
                open = false;
              }}
            >
              <CheckIcon
                class={cn(
                  "me-2 size-4",
                  customer_id !== customer.id.toString() && "text-transparent"
                )}
              />
              {customer.name}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
