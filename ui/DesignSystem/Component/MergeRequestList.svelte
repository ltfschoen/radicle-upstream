<script lang="typescript">
  import type { MergeRequest } from "../../src/source";

  import List from "./List.svelte";
  import MergeRequestCard from "./MergeRequestCard.svelte";

  export let mergeRequests: MergeRequest[];
</script>

<style>
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
  }
</style>

<List
  dataCy="project-list"
  items={mergeRequests}
  on:select
  let:item={mergeRequest}
  style="margin: 0 auto;">
  <div class="list-item" data-cy={`project-list-entry-${mergeRequest}`}>
    <MergeRequestCard {mergeRequest} />
  </div>
</List>
