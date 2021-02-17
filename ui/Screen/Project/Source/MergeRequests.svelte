<script lang="typescript">
  import { store } from "../../../src/screen/project/source";

  import {
    EmptyState,
    Error,
    MergeRequestList,
    Remote,
  } from "../../../DesignSystem/Component";
</script>

<Remote {store} let:data={{ mergeRequests }}>
  <MergeRequestList
    {mergeRequests}
    on:select={() => {
      console.log('Selected a merge request');
    }} />

  <div slot="empty">
    <EmptyState
      text="There’s nothing here yet, get started by opening your first merge request."
      primaryActionText="Start a new merge request"
      on:primaryAction={() => {
        console.log('Clicked new merge request');
      }} />
  </div>

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
