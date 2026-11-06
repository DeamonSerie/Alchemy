use serde::{Deserialize, Serialize};
use alchemy_core::WalletId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vote {
    pub voter: WalletId,
    pub proposal_id: u64,
    pub approve: bool,
    pub is_checker: bool,
}
