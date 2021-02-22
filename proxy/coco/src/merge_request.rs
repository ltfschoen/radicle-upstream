#[derive(Debug, Clone)]
pub struct MergeRequest {
    pub id: String,
    pub merged: bool,
    pub peer: crate::project::Peer<crate::project::peer::Status<crate::Person>>,
    // commits: coco::source::commits,
}

/// TODO
///
/// # Errors
pub async fn list(
    state: &crate::State,
    project: crate::Urn,
) -> Result<Vec<MergeRequest>, crate::state::Error> {
    let mut merge_requests = Vec::new();
    for peer in state.list_project_peers(project.clone()).await? {
        let default_branch = state
            .get_branch(project.clone(), peer.peer_id(), None)
            .await?;
        let tags = state
            .with_browser(default_branch, |browser| crate::source::tags(browser))
            .await?;

        for tag in tags {
            merge_requests.push(MergeRequest {
                id: tag.0,
                merged: false,
                peer: peer.clone(),
            })
        }
    }
    Ok(merge_requests)
}
