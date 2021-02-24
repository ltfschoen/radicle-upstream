<script lang="typescript">
  import { pop } from "svelte-spa-router";

  import { mergeRequestDetails as store } from "../../../src/screen/project/source";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import { Header, Remote } from "../../../DesignSystem/Component";
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
  <Remote {store} let:data={mergeRequest}>
    <Header.Back style="padding: 1rem; z-index: 0;" on:arrowClick={() => pop()}>
      <h3 style="margin-bottom: .75rem">
        <Icon.Revision />
        {mergeRequest.id}
      </h3>
      <div class="metadata">
        <span class="row"> <span>{mergeRequest.title}</span> </span>
        <span class="row"> <span>by {mergeRequest.description}</span> </span>
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
  </Remote>
</div>
