//! Custom EthClassResponse class so include a gas_used field in responses.
use alloy_primitives::Bytes;
use alloy_rpc_types_eth::EthCallResponse as AlloyEthCallResponse;
use serde::{Deserialize, Serialize};

/// Extended version of EthCallResponse with gas_used field
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Extended response structure for Ethereum call operations.
///
/// This struct contains the result of an `eth_call` operation along with additional
/// metadata such as gas usage. It accommodates both successful calls (with a value)
/// and failed calls (with an error message).
///
/// # Fields
///
/// * `value` - The return data from a successful call operation, represented as bytes.
///   This field is only present when the call did not encounter errors.
///
/// * `error` - Error message if the call operation failed. This field is only
///   present when an error occurred during execution.
///
/// * `gas_used` - The amount of gas consumed by the call operation, regardless of
///   whether it succeeded or failed. Useful for gas estimation and optimization.
pub struct ExtendedEthCallResponse {
    /// eth_call output (if no error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Bytes>,

    /// eth_call output (if error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Gas used by the call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<u64>,
}

impl From<AlloyEthCallResponse> for ExtendedEthCallResponse {
    fn from(response: AlloyEthCallResponse) -> Self {
        Self { value: response.value, error: response.error, gas_used: None }
    }
}

impl From<ExtendedEthCallResponse> for AlloyEthCallResponse {
    fn from(response: ExtendedEthCallResponse) -> Self {
        Self { value: response.value, error: response.error }
    }
}
