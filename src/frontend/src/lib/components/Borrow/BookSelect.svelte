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
        book_id = $bindable(),
        books,
    }: {
        book_id: String | null;
        books: Book[];
    } = $props();

    const selectedValue = $derived(
        books.find((f) => f.id.toString() === book_id)?.name,
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
                {selectedValue || "Select Book..."}
                <ChevronsUpDownIcon class="ms-2 size-4 shrink-0 opacity-50" />
            </Button>
        {/snippet}
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0">
        <Command.Root>
            <Command.List>
                <Command.Group>
                    {#each books as book}
                        <Command.Item
                            value={book.id.toString()}
                            onSelect={() => {
                                book_id = book.id.toString();
                                open = false;
                            }}
                        >
                            <CheckIcon
                                class={cn(
                                    "me-2 size-4",
                                    book_id !== book.id.toString() &&
                                        "text-transparent",
                                )}
                            />
                            {book.name}
                        </Command.Item>
                    {/each}
                </Command.Group>
            </Command.List>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
