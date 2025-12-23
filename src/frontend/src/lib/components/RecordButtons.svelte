<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";

    let {
        record,
        newRecord,
        editedRecord,
        previousRecord,
        nextRecord,
        insert,
        edit,
        remove,
    }: {
        record: any;
        newRecord: any;
        editedRecord: any;
        previousRecord: () => Promise<void>;
        nextRecord: () => Promise<void>;
        insert: () => Promise<void>;
        edit: () => Promise<void>;
        remove: () => Promise<void>;
    } = $props();
</script>

<div class="flex justify-center">
    <Button
        class="mt-2 mx-2"
        onclick={previousRecord}
        disabled={newRecord !== null || editedRecord !== null}
    >
        Previous Record
    </Button>
    <Button
        class="mt-2 mx-2"
        onclick={nextRecord}
        disabled={record == null || newRecord !== null || editedRecord !== null}
    >
        Next Record
    </Button>
</div>
<div class="flex justify-center">
    <Button class="mt-2 mx-2" onclick={insert} disabled={editedRecord !== null}>
        {#if newRecord}
            Submit
        {:else}
            New record
        {/if}
    </Button>
    <Button
        class="mt-2 mx-2"
        onclick={edit}
        disabled={record == null || newRecord !== null}
    >
        {#if editedRecord}
            Submit
        {:else}
            Edit Record
        {/if}
    </Button>
    <Button
        class="mt-2 mx-2"
        onclick={remove}
        disabled={record == null || newRecord !== null || editedRecord !== null}
    >
        Delete Record
    </Button>
</div>
