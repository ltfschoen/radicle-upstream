use radicle_surf::git::BranchType;

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
        let default_branch = state.find_default_branch(project.clone()).await?;
        let branches = state
            .with_browser(default_branch, |browser| {
                let branches = browser.list_branches(Some(BranchType::Remote {
                    name: Some(peer.peer_id().to_string()),
                }))?;
                Ok(branches)
            })
            .await?;

        for branch in branches {
            let name = branch.name.to_string();
            if let Some(id) = name.strip_prefix("merge-requests/") {
                merge_requests.push(MergeRequest {
                    id: id.to_owned(),
                    merged: false,
                    peer: peer.clone(),
                })
            }
        }
    }
    Ok(merge_requests)
}
