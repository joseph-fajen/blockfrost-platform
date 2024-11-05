use crate::errors::BlockfrostError;
use chrono::{Duration, TimeZone, Utc};
use metrics::gauge;
use pallas_crypto::hash::Hasher;
use pallas_network::{
    facades::NodeClient,
    miniprotocols,
    miniprotocols::{
        localstate,
        localtxsubmission::{EraTx, Response},
    },
};
use pallas_traverse::wellknown;
use tracing::{info, warn};

pub struct Node {
    network_magic: u64,
    socket: String,
}

impl Node {
    /// Creates a new `Node` instance
    pub fn new(socket: &str, network_magic: u64) -> Self {
        Self {
            socket: socket.to_string(),
            network_magic,
        }
    }

    /// Establishes a new NodeClient connection.
    async fn connect(&self) -> Result<NodeClient, BlockfrostError> {
        info!("Connecting to node socket {} ...", self.socket);
        let node_gauge = gauge!("cardano_node_connected");

        match NodeClient::connect(&self.socket, self.network_magic).await {
            Ok(client) => {
                info!("Connection to node was successfully established.");
                node_gauge.set(1);
                Ok(client)
            }
            Err(e) => {
                warn!("Failed to connect to node: {:?}", e);
                node_gauge.set(0);
                Err(BlockfrostError::custom_400(e.to_string()))
            }
        }
    }

    /// Submits a transaction to the connected Cardano node.
    pub async fn submit_transaction(&self, tx: String) -> Result<String, BlockfrostError> {
        let tx = hex::decode(tx).map_err(|e| BlockfrostError::custom_400(e.to_string()))?;
        let txid = hex::encode(Hasher::<256>::hash_cbor(&tx));

        let era_tx = EraTx(6, tx);

        // Connect to the node
        let mut client = self.connect().await?;
        let submission_client = client.submission();

        // Submit the transaction
        match submission_client.submit_tx(era_tx).await {
            Ok(Response::Accepted) => {
                info!("Transaction accepted by the node {}", txid);
                Ok(txid)
            }
            Ok(Response::Rejected(reason)) => {
                let reason_hex = hex::encode(&reason.0);
                warn!("Transaction was rejected: {}", reason_hex);
                Err(BlockfrostError::custom_400(reason_hex))
            }
            Err(e) => {
                warn!("Error during transaction submission: {:?}", e);
                Err(BlockfrostError::custom_400(e.to_string()))
            }
        }
    }

    pub async fn sync_progress(&mut self) -> Result<SyncProgress, BlockfrostError> {
        async fn action(
            generic_client: &mut localstate::GenericClient,
        ) -> Result<SyncProgress, BlockfrostError> {
            let current_era = localstate::queries_v16::get_current_era(generic_client).await?;

            let epoch =
                localstate::queries_v16::get_block_epoch_number(generic_client, current_era)
                    .await?;

            let geneses =
                localstate::queries_v16::get_genesis_config(generic_client, current_era).await?;
            let genesis = geneses.first().ok_or_else(|| {
                BlockfrostError::internal_server_error("Expected at least one genesis".to_string())
            })?;

            let system_start = localstate::queries_v16::get_system_start(generic_client).await?;
            let chain_point = localstate::queries_v16::get_chain_point(generic_client).await?;
            let slot = chain_point.slot_or_default();

            // FIXME: this is debatable, because it won’t work for custom networks; we should rather
            // get this information by calling `Ouroboros.Consensus.HardFork.History.Qry.slotToWallclock`
            // like both cardano-cli (through cardano-api) and Ogmios do, but it’s not implemented
            // in pallas_network yet.
            let wellknown_genesis = wellknown::GenesisValues::from_magic(
                genesis.network_magic.into(),
            )
            .ok_or_else(|| {
                BlockfrostError::internal_server_error(format!(
                    "Only well-known networks are supported (unsupported network magic: {})",
                    genesis.network_magic
                ))
            })?;

            let year: i32 = system_start.year.try_into().map_err(|e| {
                BlockfrostError::internal_server_error(format!("Failed to convert year: {}", e))
            })?;

            let base_date = Utc
                .with_ymd_and_hms(year, 1, 1, 0, 0, 0)
                .single()
                .ok_or_else(|| {
                    BlockfrostError::internal_server_error("Invalid base date".to_string())
                })?;

            let days = Duration::days((system_start.day_of_year - 1).into());

            let nanoseconds: i64 = (system_start.picoseconds_of_day / 1_000)
                .try_into()
                .map_err(|e| {
                    BlockfrostError::internal_server_error(format!(
                        "Failed to convert picoseconds: {}",
                        e
                    ))
                })?;

            let duration_ns = Duration::nanoseconds(nanoseconds);

            let utc_start = base_date + days + duration_ns;

            let slot_time_secs: i64 = wellknown_genesis
                .slot_to_wallclock(slot)
                .try_into()
                .map_err(|e| {
                    BlockfrostError::internal_server_error(format!(
                        "Failed to convert slot time: {}",
                        e
                    ))
                })?;

            let utc_slot = Utc
                .timestamp_opt(slot_time_secs, 0)
                .single()
                .ok_or_else(|| {
                    BlockfrostError::internal_server_error("Invalid slot timestamp".to_string())
                })?;

            let utc_now = Utc::now();

            let utc_slot_capped = std::cmp::min(utc_now, utc_slot);

            let tolerance = 60; // [s]
            let percentage = if (utc_now - utc_slot_capped).num_seconds() < tolerance {
                1.0
            } else {
                let network_duration = (utc_now - utc_start).num_seconds() as f64;
                let duration_up_to_slot = (utc_slot_capped - utc_start).num_seconds() as f64;
                duration_up_to_slot / network_duration
            };

            let block = match chain_point {
                miniprotocols::Point::Origin => String::new(),
                miniprotocols::Point::Specific(_, block) => hex::encode(&block),
            };

            Ok(SyncProgress {
                percentage,
                era: current_era,
                epoch,
                slot,
                block,
            })
        }

        // Connect to the node
        let mut client = self.connect().await?;
        let generic_client = client.statequery();

        // Acquire the client
        generic_client.acquire(None).await?;

        // Run the action and ensure the client is released afterward
        let result = action(generic_client).await;

        // Always release the client, even if action fails
        if let Err(e) = generic_client.send_release().await {
            warn!("Failed to release client: {:?}", e);
        }

        result
    }
}

#[derive(serde::Serialize)]
pub struct SyncProgress {
    percentage: f64,
    era: u16,
    epoch: u32,
    slot: u64,
    block: String,
}
