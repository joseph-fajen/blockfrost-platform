use super::connection::NodeClient;
use crate::{
    cbor::haskell_types::{TxSubmitFail, TxValidationError},
    BlockfrostError,
};
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
    /// Should return Http 200 if the transaction was accepted by the node.
    /// If the transaction was rejected, should return Http 400 with a JSON body.
    /// swagger: https://github.com/IntersectMBO/cardano-node/blob/6e969c6bcc0f07bd1a69f4d76b85d6fa9371a90b/cardano-submit-api/swagger.yaml#L52
    /// Haskell code:  https://github.com/IntersectMBO/cardano-node/blob/6e969c6bcc0f07bd1a69f4d76b85d6fa9371a90b/cardano-submit-api/src/Cardano/TxSubmit/Web.hs#L158
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
                let reason = &reason.0[2..];
                let msg_res = Self::try_decode_error(&reason);

                match msg_res {
                    Ok(decoded_error) => {
                        let error_response = Self::generate_error_response(decoded_error);
                        let error_message = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| "Failed to serialize error response".to_string());

                        warn!(
                            "reason in cbor: {}, error message: {}",
                            hex::encode(reason),
                            error_message
                        );

                        Err(BlockfrostError::custom_400(error_message))
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

    /// Mimicks the data structure of the error response from the cardano-submit-api
    fn generate_error_response(error: TxValidationError) -> TxSubmitFail {
        use crate::cbor::haskell_types::{
            TxCmdError::TxCmdTxSubmitValidationError, TxSubmitFail::TxSubmitFail,
            TxValidationErrorInCardanoMode::TxValidationErrorInCardanoMode,
        };

        TxSubmitFail(TxCmdTxSubmitValidationError(
            TxValidationErrorInCardanoMode(error),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use pallas_codec::minicbor::display;
    use serde::Deserialize;
    use serde_json::Value;

    use crate::cbor::haskell_types::{
        ApplyConwayTxPredError::*, ApplyTxError, ShelleyBasedEra::*, TxValidationError::*,
    };

    use super::*;

    #[test]
    fn test_generate_error_response_with_multiple_errors() {
        let validation_error = ShelleyTxValidationError {
            error: ApplyTxError(vec![
                ConwayMempoolFailure("error1".to_string()),
                ConwayMempoolFailure("error2".to_string()),
            ]),
            era: ShelleyBasedEraConway,
        };

        let error_string =
            serde_json::to_string(&NodeClient::generate_error_response(validation_error))
                .expect("Failed to convert error to JSON");
        let expected_error_string = r#"{"tag":"TxSubmitFail","contents":{"tag":"TxCmdTxSubmitValidationError","contents":{"tag":"TxValidationErrorInCardanoMode","contents":{"kind":"ShelleyTxValidationError","error":["MempoolFailure (error1)","MempoolFailure (error2)"],"era":"ShelleyBasedEraConway"}}}}"#;

        assert_eq!(error_string, expected_error_string);
    }

    #[test]
    fn test_try_decode_error() {
        /*assert_decoding(
                    "818206828201820083061b00000002362a77301b0000000253b9c11d8201820083051a00028bfd18ad",
                    2,
                );
                assert_decoding("818206828201820083051a000151351a00074b8582076162", 2);
        */
        assert_decoding("8182068382076ff3b0bfaf61f0acb5a4f0a4b5a84d5a820486581ca385b023b264e8a4c3db4ad56a43084a75390599c5998f4dcc0fd4ef581c00a0dcf8d75a363dfe651e1616b5ba68290b3986f3a6f9df24433f31581c28f126a72f2e93ed348eb9ac3ff2f821853db12b290342f162bf7666581cd6a08dab949e3a7b9a922e871a86889776941e19ad73b55f42990ca8581c0f8a0c9f687f5fb24f749e8e725006049e3184995b129d2ab1f4a92d581cc50365d93a13b5e4739b8a2558ffe2d4c13c44e9028f2fda1fbd5b4c82038208841a0009d4c2581de0a5ec35eae62c7182feef89843ccfcd150437307181623c4058f95ac28302a8581df06b81acc1bdcac7bbae3394663ed756f59db74c4db2472ed36f4ef3421a000cb1d5581df0ac5152ff0a2fa6738a056f80183c071067cbca3db5f09d84a3e4c7fe1a000bf101581df0f679488d698af573f4e6e356944f44b505d2064735dbed445e318d111a0001353c581de08dd6df19505b5aa2b5a43a14eb1d65f2a91b1bc46d81bd9c2dca9a191a000e8adc581df1ce111952d479facae4dcdc72c59ab6988925181beca0a93f9f8830371a0001654c581df1dc3148180ded332e0d5477beaaa46b9ba49604a25874cd0d81437e051a000a3336581de16b5fd4dc72f45ed2f5b7a9e0d7b3b1c4c36a9e42d550ff2ee8d5ccb11a0009989d581de1c756a089f80e9fd59c0a02bba848bdf5ae3e29f67d50f801773cae871a0004e81a581c2f6ab67f71d2e4a765d972c4d8351c0f1a7a2951f39021954ba353dd82782468747470733a2f2f5a78766d5a7451697743636b6778784a49514b30716649412e636f6d5820ac6df2c8b71f17e7a8c81fa69c2526d4e310462729a68cf0c4954831b24c2961", 3);
    }
    fn assert_decoding(cbor_hex: &str, error_count: usize) {
        let buffer = hex::decode(cbor_hex).unwrap();

        let error = NodeClient::try_decode_error(&buffer);

        match error {
            Ok(ShelleyTxValidationError {
                error: ApplyTxError(errors),
                era,
            }) => {
                assert!(
                    era == ShelleyBasedEraConway,
                    "Expected ShelleyBasedEraConway"
                );
                assert_eq!(errors.len(), error_count, "Errors count mismatch",);
            }
            Err(error) => panic!(
                "Failed to decode cbor: {:?}, error: {:?}, cbor repr: {}",
                cbor_hex,
                error,
                display(&buffer)
            ),
            _ => panic!("Expected ShelleyTxValidationError"),
        }
    }

    #[test]
    fn test_decoding_with_cases() {

        let case_files = get_file_list_from_folder();

        for case_file in case_files {
            let cases = read_cases_from_file(&case_file);

            for case in cases.test_cases {
                let buffer = match hex::decode(&case.cbor) {
                    Ok(buffer) => buffer,
                    Err(e) => {
                        panic!("Failed to decode hex: {:?} {:?}", e, &case);
                    }
                };

                let error = match NodeClient::try_decode_error(&buffer) {
                    Ok(error) => error,
                    Err(e) => {
                        panic!(
                            "Failed to decode error: {:?}\n case: {:?} \n file: {:?}\n\n",
                            e, &case, &case_file
                        );
                    }
                };

                let error_response = NodeClient::generate_error_response(error);

                let generated_json = match serde_json::to_value(&error_response) {
                    Ok(json) => json,
                    Err(e) => {
                        panic!("Failed to convert error response to JSON: {:?} \n case: {:?} \n file: {:?}\n\n", e, &case, &case_file);
                    }
                };

                
                assert_eq!(
                    &case.json, &generated_json, 
                    "Failed to match JSON: \n case: {:?} \n file: {:?}\n\n",
                    &case, &case_file
                );
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "camelCase"))]
    pub struct CborTestCases {
        _seed: u64,
        test_cases: Vec<CborTestCase>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "camelCase"))]
    /// A struct representing a CBOR test case.
    ///
    /// # Fields
    ///
    /// * `cbor` - A string representing the CBOR-encoded error reasons.
    /// * `haskell_repr` - A string representation of the transaction in Haskell format. This field contains the transaction error as a Haskell string.
    /// * `json` - A vector of strings representing the errors in JSON format.
    pub struct CborTestCase {
        cbor: String,
        haskell_repr: String,
        json: Value,
    }

    fn get_file_list_from_folder() -> Vec<String> {
        let folder_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cbor");

        let mut path_list = fs::read_dir(folder_path)
            .unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        path_list.sort();

        path_list
    }
    /// Reads CBOR test cases from a JSON file located at `tests/fixtures/cbor/cases.json`.
    ///
    /// The file path is constructed using the `CARGO_MANIFEST_DIR` environment variable to ensure
    /// it is relative to the project's root directory.
    ///
    /// # Returns
    ///
    /// * `CborTestCases` - A struct containing the parsed test cases from the JSON file.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// * The file cannot be opened.
    /// * The JSON content cannot be parsed.
    ///
    fn read_cases_from_file(case_file_path: &String) -> CborTestCases {
        let file = std::fs::File::open(case_file_path)
            .unwrap_or_else(|_| panic!("Failed to open file: {}", case_file_path));

        let reader = std::io::BufReader::new(file);

        serde_json::from_reader(reader).expect("Failed to parse JSON")
    }
}
