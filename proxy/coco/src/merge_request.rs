#[derive(Debug, Clone)]
pub struct MergeRequest {
    pub id: String,
    pub merged: bool,
    pub peer: crate::project::Peer<crate::project::peer::Status<crate::Person>>,
}

/// TODO
///
/// # Errors
pub async fn list(
    state: &crate::State,
    project: crate::Urn,
) -> Result<Vec<MergeRequest>, crate::state::Error> {
    let mut merge_requests = Vec::new();
    // TODO: only browse specifc peer
    for peer in state.list_project_peers(project.clone()).await? {
        let default_branch = state.find_default_branch(project.clone()).await?;
        let tags = state
            .with_browser(default_branch, |browser| {
                let tags = browser.list_tags()?;
                Ok(tags)
            })
            .await?;

        for tag in tags {
            let name = tag.name().to_string();
            if let Some(id) = name.strip_prefix("merge-request/") {
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
