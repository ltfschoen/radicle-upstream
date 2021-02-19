<script lang="ts">
  import { getContext } from "svelte";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import CheckoutMergeRequestButton from "./CheckoutMergeRequestButton.svelte";
  import AcceptMergeRequestButton from "./AcceptMergeRequestButton.svelte";

  import type { MergeRequest } from "../../../src/source";
  import { isMaintainer } from "../../../src/project";

  export let mergeRequest: MergeRequest;

  const mergeInfo = mergeRequest && mergeRequest.merged ? "Closed" : "Opened";
  const showCheckout =
    mergeRequest &&
    isMaintainer(mergeRequest.identity.urn, getContext("project-page").project);
</script>

<style>
  .merge-request-card {
    display: flex;
    max-height: 3.2rem;
  }

  .info-column {
    display: flex;
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

  .actions-column {
    /* TODO: this should be replaced by having the column at the right end */
    margin-left: 30rem;
    display: flex;
    flex-direction: row;
  }
</style>

{#if mergeRequest}
  <div class="merge-request-card">
    <Icon.Revision />
    <div class="info-column">
      <div class="title-row">
        <p
          class="typo-text-bold typo-overflow-ellipsis"
          title={mergeRequest.id}>
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
        <p
          class="typo-text-bold"
          style="color: var(--color-foreground-level-6);">
          {mergeRequest.identity.metadata.handle}
        </p>
      </div>
    </div>
    <div class="actions-column">
      {#if showCheckout}
        <CheckoutMergeRequestButton
          id={mergeRequest.id}
          peerId={mergeRequest.identity.peerId} />
        <AcceptMergeRequestButton id={mergeRequest.id} />
      {/if}
    </div>
  </div>
{/if}
