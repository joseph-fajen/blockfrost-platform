use super::connection::NodeClient;
use crate::{cbor::haskell_types::TxValidationError, BlockfrostError};
use pallas_codec::minicbor::Decoder;
use pallas_crypto::hash::Hasher;
use pallas_network::{
    miniprotocols::{
        localstate,
        localtxsubmission::{EraTx, Response},
    },
    multiplexer::Error,
};
use tracing::{info, warn};

impl NodeClient {
    /// Submits a transaction to the connected Cardano node.
    /// This API meant to be fully compatible with cardano-submit-api.
    /// Should return HTTP 200 if the transaction was accepted by the node.
    /// If the transaction was rejected, should return HTTP 400 with a JSON body:
    /// * Swagger: <https://github.com/IntersectMBO/cardano-node/blob/6e969c6bcc0f07bd1a69f4d76b85d6fa9371a90b/cardano-submit-api/swagger.yaml#L52>
    /// * Haskell code: <https://github.com/IntersectMBO/cardano-node/blob/6e969c6bcc0f07bd1a69f4d76b85d6fa9371a90b/cardano-submit-api/src/Cardano/TxSubmit/Web.hs#L158>
    pub async fn submit_transaction(&mut self, tx: String) -> Result<String, BlockfrostError> {
        let tx = hex::decode(tx).map_err(|e| BlockfrostError::custom_400(e.to_string()))?;
        let txid = hex::encode(Hasher::<256>::hash_cbor(&tx));

        let current_era = self
            .with_statequery(|generic_client: &mut localstate::GenericClient| {
                Box::pin(async {
                    Ok(localstate::queries_v16::get_current_era(generic_client).await?)
                })
            })
            .await?;

        let era_tx = EraTx(current_era, tx);

        // Connect to the node
        let submission_client = self.client.as_mut().unwrap().submission();

        // Submit the transaction
        match submission_client.submit_tx(era_tx).await {
            Ok(Response::Accepted) => {
                info!("Transaction accepted by the node {}", txid);
                Ok(txid)
            }
            Ok(Response::Rejected(reason)) => {
                // The [2..] is a Pallas bug, cf. <https://github.com/txpipe/pallas/pull/548>.
                let reason = &reason.0[2..];

                match self.fallback_decoder.decode(reason).await {
                    Ok(submit_api_json) => {
                        let error_message = "TxSubmitFail".to_string();
                        warn!(
                            "{}: {} ~ {:?}",
                            error_message,
                            hex::encode(reason),
                            submit_api_json
                        );

                        Err(BlockfrostError::custom_400_details(
                            error_message,
                            submit_api_json,
                        ))
                    }

                    Err(e) => {
                        warn!("Failed to decode error reason: {:?}", e);

                        Err(BlockfrostError::custom_400(format!(
                            "Failed to decode error reason: {:?}",
                            e
                        )))
                    }
                }
            }
            Err(e) => {
                let error_message = format!("Error during transaction submission: {:?}", e);

                Err(BlockfrostError::custom_400(error_message))
            }
        }
    }

    pub fn try_decode_error(buffer: &[u8]) -> Result<TxValidationError, Error> {
        let maybe_error = Decoder::new(buffer).decode();

        match maybe_error {
            Ok(error) => Ok(error),
            Err(err) => {
                warn!(
                    "Failed to decode error: {:?}, buffer: {}",
                    err,
                    hex::encode(buffer)
                );

                // Decoding failures are not errors, but some missing implementation or mis-implementations on our side.
                // A decoding failure is a bug in our code, not a bug in the node.
                // It should not effect the program flow, but should be logged and reported.
                Err(Error::Decoding(err.to_string()))
            }
        }
    }

    #[cfg(test)]
    /// Mimicks the data structure of the error response from the cardano-submit-api
    /// This fucntion will be used by the native error serializer once it's ready.
    pub fn _unused_i_i_i_i_i_i_i_generate_error_response(
        error: TxValidationError,
    ) -> crate::cbor::haskell_types::TxSubmitFail {
        use crate::cbor::haskell_types::{
            TxCmdError::TxCmdTxSubmitValidationError, TxSubmitFail,
            TxValidationErrorInCardanoMode::TxValidationErrorInCardanoMode,
        };

        TxSubmitFail::TxSubmitFail(TxCmdTxSubmitValidationError(
            TxValidationErrorInCardanoMode(error),
        ))
    }
}
