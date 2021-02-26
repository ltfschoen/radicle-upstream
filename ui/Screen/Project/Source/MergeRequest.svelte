<script lang="typescript">
  import { pop } from "svelte-spa-router";
  import { getContext } from "svelte";

  import { isMaintainer } from "../../../src/project";
  import {
    mergeRequestDetails as store,
    selectCommit,
  } from "../../../src/screen/project/source";
  import type { CommitHeader } from "../../../src/source";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import { Header, Remote } from "../../../DesignSystem/Component";
  import History from "../../../DesignSystem/Component/SourceBrowser/History.svelte";
  import CheckoutMergeRequestButton from "./CheckoutMergeRequestButton.svelte";
  import AcceptMergeRequestButton from "./AcceptMergeRequestButton.svelte";

  const onSelect = ({ detail: commit }: { detail: CommitHeader }) => {
    selectCommit(commit);
  };
</script>

<style>
  .merge-request-page {
    max-width: var(--content-max-width);
    margin: 0 auto;
    padding: 0 var(--content-padding);
    min-width: var(--content-min-width);
  }

  .metadata {
    display: flex;
    flex-direction: column;
  }

  .row {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }

  .row:last-child {
    margin-bottom: 0;
  }
</style>

<div class="merge-request-page" data-cy="merge-request-page">
  <Remote {store} let:data={{ mergeRequest, commits }}>
    <Header.Back style="padding: 1rem; z-index: 0;" on:arrowClick={() => pop()}>
      <div style="display: flex; justify-content: space-between;">
        <h3 style="margin-bottom: .75rem">
          <Icon.Revision />
          {mergeRequest.id}
        </h3>
        {#if isMaintainer(mergeRequest.identity.urn, getContext('project-page').project)}
          <div style="display: flex;">
            <CheckoutMergeRequestButton
              id={mergeRequest.id}
              peerId={mergeRequest.identity.peerId} />
            <AcceptMergeRequestButton id={mergeRequest.id} />
          </div>
        {/if}
      </div>
      <div class="metadata">
        <span class="row">
          <span style="display:flex;">
            Opened by
            <Avatar
              avatarFallback={mergeRequest.identity.avatarFallback}
              size="small"
              style="display: flex; justify-content: flex-start; margin-left: 0.5rem;"
              title={mergeRequest.identity.metadata.handle}
              variant="circle" />
          </span>
        </span>
      </div>
    </Header.Back>
    <History history={commits} on:select={onSelect} />
  </Remote>
</div>
