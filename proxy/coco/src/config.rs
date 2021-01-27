//! Crate configuration.

use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use tokio::sync::{mpsc, watch};

use librad::{
    git::fetch,
    net,
    net::discovery,
    paths,
    peer::PeerId,
    signer::{BoxedSigner, Signer},
};

use crate::seed;

lazy_static::lazy_static! {
    /// Localhost binding to any available port, i.e. `127.0.0.1:0`.
    pub static ref LOCALHOST_ANY: SocketAddr =
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

    /// Binds to all local interfaces and any available port.
    pub static ref INADDR_ANY: SocketAddr =
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));
}

/// The environment variable that points to where librad data lives.
pub const RAD_HOME: &str = "RAD_HOME";

/// The default name for a user's remote, which is `"rad"`.
pub const RAD_REMOTE: &str = "rad";

/// Provide the default config.
///
/// Address: 127.0.0.1:0
/// No seeds.
/// Default gossip parameters.
///
/// # Errors
///
/// Results in an error if the [`paths::Paths`] could not be created.
pub fn default(
    signer: BoxedSigner,
    path: impl AsRef<std::path::Path>,
) -> Result<net::peer::PeerConfig<BoxedSigner>, io::Error> {
    let paths = paths::Paths::from_root(path)?;
    Ok(configure(paths, signer, *LOCALHOST_ANY))
}

/// Configure a [`net::peer::PeerConfig`].
#[allow(clippy::as_conversions)]
#[must_use]
pub fn configure<S>(
    paths: paths::Paths,
    signer: S,
    listen_addr: SocketAddr,
) -> net::peer::PeerConfig<S>
where
    S: Signer + Clone + Send + Sync + 'static,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    let gossip_params = net::gossip::MembershipParams::default();
    let storage_config = net::peer::StorageConfig::default();
    let fetch_limit = fetch::Limit::default();
    let network = net::Network::default();

    net::peer::PeerConfig {
        signer,
        paths,
        listen_addr,
        gossip_params,
        storage_config,
        fetch_limit,
        network,
    }
}

/// Builds a static discovery over the list of given `seeds`.
#[allow(clippy::as_conversions)]
#[must_use]
pub fn static_seed_discovery(seeds: &[seed::Seed]) -> discovery::Static {
    discovery::Static::resolve(
        seeds
            .iter()
            .map(|seed| (seed.peer_id, seed.addrs.as_slice())),
    )
    .expect("unable to resolve seeds")
}

/// Stream based discovery based on a watch.
pub struct StreamDiscovery {
    /// Stream of new sets of seeds coming from configuration changes.
    seeds_receiver: watch::Receiver<Vec<seed::Seed>>,
}

impl StreamDiscovery {
    /// Returns a new streaming discovery.
    #[must_use]
    pub fn new(seeds_receiver: watch::Receiver<Vec<seed::Seed>>) -> Self {
        Self { seeds_receiver }
    }
}

impl discovery::Discovery for StreamDiscovery {
    type Addr = SocketAddr;
    type Stream = mpsc::Receiver<(PeerId, Vec<SocketAddr>)>;

    fn discover(mut self) -> Self::Stream {
        let (mut sender, receiver) = mpsc::channel(1024);

        tokio::spawn(async move {
            while let Some(seeds) = self.seeds_receiver.recv().await {
                for seed in seeds {
                    sender.send(seed.into()).await.ok();
                }
            }
        });

        receiver
    }
}
