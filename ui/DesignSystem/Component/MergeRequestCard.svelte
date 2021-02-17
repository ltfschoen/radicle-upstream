<script lang="ts">
  import { Avatar, Icon } from "../../DesignSystem/Primitive";

  import type { MergeRequest } from "../../src/source";

  export let mergeRequest: MergeRequest;

  const mergeInfo = mergeRequest && mergeRequest.merged ? "Closed" : "Opened";
</script>

<style>
  .project-card {
    display: flex;
    max-height: 3.2rem;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
    margin-right: 1.5rem;
  }

  .title-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
  }

  .title-row p {
    color: var(--color-foreground-level-6);
  }

  .desc-row {
    color: var(--color-foreground-level-6);
    display: flex;
    margin-top: 0.125rem;
    width: -webkit-fill-available;
  }
</style>

{#if mergeRequest}
  <div class="project-card">
    <div class="title-row">
      <Icon.Revision />
      <p class="typo-text-bold typo-overflow-ellipsis" title={mergeRequest.id}>
        {mergeRequest.id}
      </p>
    </div>
    <div class="desc-row">
      <p>{mergeInfo} by</p>
      <Avatar
        avatarFallback={mergeRequest.identity.avatarFallback}
        size="small"
        style="display: flex; justify-content: flex-start; margin-right: 0.5rem;"
        variant="circle" />
      <p class="typo-text-bold" style="color: var(--color-foreground-level-6);">
        {mergeRequest.identity.metadata.handle}
      </p>
    </div>
  </div>
{/if}
