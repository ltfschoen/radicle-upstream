//! Abstractions and utilities to run and interact with link and surf.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_used,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]
#![allow(
    clippy::clone_on_ref_ptr,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::missing_inline_in_public_items,
    clippy::multiple_crate_versions,
    clippy::multiple_inherent_impl,
    clippy::similar_names,
    clippy::too_many_lines
)]
#![feature(duration_zero, hash_set_entry, or_patterns)]

use std::net::SocketAddr;

pub use librad::{
    git::{self, identities::local::LocalIdentity, include, local::url::LocalUrl},
    identities::{Person, Project, Urn},
    keys,
    net::{self, discovery},
    paths::Paths,
    peer::PeerId,
    profile, signer,
};

pub use radicle_git_ext as git_ext;

pub use radicle_git_helpers::remote_helper;

pub use radicle_surf::{
    diff::{Diff, FileDiff},
    vcs::git::Stats,
};

pub mod config;
pub mod control;
pub mod convert;
pub mod git_helper;
mod identifier;
pub use identifier::Identifier;
pub mod keystore;
pub mod merge_request;
pub mod peer;
pub use peer::{Control as PeerControl, Event as PeerEvent, Peer, RunConfig, Status as PeerStatus};
pub mod state;
pub use state::State;
pub mod project;
pub mod request;

pub mod seed;

pub mod source;

mod spawn_abortable;
pub use spawn_abortable::{Error as SpawnAbortableError, SpawnAbortable};

/// Constructs a [`Peer`] and [`State`] pair from a [`net::peer::PeerConfig`].
///
/// # Errors
///
/// * peer construction from config fails.
/// * accept on the peer fails.
pub async fn boostrap<D>(
    config: net::peer::PeerConfig<signer::BoxedSigner>,
    disco: D,
    signer: signer::BoxedSigner,
    store: kv::Store,
    run_config: RunConfig,
) -> Result<(Peer, State), state::Error>
where
    D: net::discovery::Discovery<Addr = SocketAddr> + Send,
    <D as net::discovery::Discovery>::Stream: 'static,
{
    let peer = librad::net::peer::Peer::bootstrap(config, disco).await?;
    let (api, run_loop) = peer.accept()?;

    let state = State::new(api, signer);
    let peer = Peer::new(run_loop, state.clone(), store, run_config);

    Ok((peer, state))
}
