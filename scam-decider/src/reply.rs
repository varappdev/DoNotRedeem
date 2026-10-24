use serde::{Deserialize, Serialize};

use crate::ScamType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reply {
    pub is_scam: f64,
    pub kind: ScamType,
    pub sign: String,
}
