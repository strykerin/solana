use {
    crate::rpc_filter::RpcFilterType,
    solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig},
    solana_sdk::{
        clock::{Epoch, Slot},
        commitment_config::{CommitmentConfig, CommitmentLevel},
    },
    solana_transaction_status::{TransactionDetails, UiTransactionEncoding},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSignatureStatusConfig {
    pub search_transaction_history: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSendTransactionConfig {
    #[serde(default)]
    pub skip_preflight: bool,
    pub preflight_commitment: Option<CommitmentLevel>,
    pub encoding: Option<UiTransactionEncoding>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateTransactionConfig {
    #[serde(default)]
    pub sig_verify: bool,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
    pub encoding: Option<UiTransactionEncoding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcLargestAccountsFilter {
    Circulating,
    NonCirculating,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcLargestAccountsConfig {
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
    pub filter: Option<RpcLargestAccountsFilter>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcEpochConfig {
    pub epoch: Option<Epoch>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcAccountInfoConfig {
    pub encoding: Option<UiAccountEncoding>,
    pub data_slice: Option<UiDataSliceConfig>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcProgramAccountsConfig {
    pub filters: Option<Vec<RpcFilterType>>,
    #[serde(flatten)]
    pub account_config: RpcAccountInfoConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcTransactionLogsFilter {
    All,
    AllWithVotes,
    Mentions(Vec<String>), // base58-encoded list of addresses
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionLogsConfig {
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcTokenAccountsFilter {
    Mint(String),
    ProgramId(String),
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcSignatureSubscribeConfig {
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
    pub enable_received_notification: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcGetConfirmedSignaturesForAddress2Config {
    pub before: Option<String>, // Signature as base-58 string
    pub until: Option<String>,  // Signature as base-58 string
    pub limit: Option<usize>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcEncodingConfigWrapper<T> {
    Deprecated(Option<UiTransactionEncoding>),
    Current(Option<T>),
}

impl<T: EncodingConfig + Default + Copy> RpcEncodingConfigWrapper<T> {
    pub fn convert_to_current(&self) -> T {
        match self {
            RpcEncodingConfigWrapper::Deprecated(encoding) => T::new_with_encoding(encoding),
            RpcEncodingConfigWrapper::Current(config) => config.unwrap_or_default(),
        }
    }
}

pub trait EncodingConfig {
    fn new_with_encoding(encoding: &Option<UiTransactionEncoding>) -> Self;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConfirmedBlockConfig {
    pub encoding: Option<UiTransactionEncoding>,
    pub transaction_details: Option<TransactionDetails>,
    pub rewards: Option<bool>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

impl EncodingConfig for RpcConfirmedBlockConfig {
    fn new_with_encoding(encoding: &Option<UiTransactionEncoding>) -> Self {
        Self {
            encoding: *encoding,
            ..Self::default()
        }
    }
}

impl RpcConfirmedBlockConfig {
    pub fn rewards_only() -> Self {
        Self {
            transaction_details: Some(TransactionDetails::None),
            ..Self::default()
        }
    }

    pub fn rewards_with_commitment(commitment: Option<CommitmentConfig>) -> Self {
        Self {
            transaction_details: Some(TransactionDetails::None),
            commitment,
            ..Self::default()
        }
    }
}

impl From<RpcConfirmedBlockConfig> for RpcEncodingConfigWrapper<RpcConfirmedBlockConfig> {
    fn from(config: RpcConfirmedBlockConfig) -> Self {
        RpcEncodingConfigWrapper::Current(Some(config))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConfirmedTransactionConfig {
    pub encoding: Option<UiTransactionEncoding>,
    #[serde(flatten)]
    pub commitment: Option<CommitmentConfig>,
}

impl EncodingConfig for RpcConfirmedTransactionConfig {
    fn new_with_encoding(encoding: &Option<UiTransactionEncoding>) -> Self {
        Self {
            encoding: *encoding,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcConfirmedBlocksConfigWrapper {
    EndSlotOnly(Option<Slot>),
    CommitmentOnly(Option<CommitmentConfig>),
}

impl RpcConfirmedBlocksConfigWrapper {
    pub fn unzip(&self) -> (Option<Slot>, Option<CommitmentConfig>) {
        match &self {
            RpcConfirmedBlocksConfigWrapper::EndSlotOnly(end_slot) => (*end_slot, None),
            RpcConfirmedBlocksConfigWrapper::CommitmentOnly(commitment) => (None, *commitment),
        }
    }
}
