use alloy::transports::TransportErrorKind;
use alloy_json_rpc::RpcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcClientError {
    #[error("rpc error: {0}")]
    Rpc(#[from] RpcError<TransportErrorKind>),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
