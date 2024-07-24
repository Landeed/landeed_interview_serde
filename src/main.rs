use anyhow::Result;
use serde;
use serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Payment {
    pub id: String,
    pub status: String,
    pub refund_status: Option<String>,
    pub order_id: String,
    pub customer_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Refund {
    pub acquirer_data: serde_json::Value,
}

pub fn payment_failed(_payment: Payment) -> Result<()> {
    todo!();
}

pub fn payment_captured(_payment: Payment) -> Result<()> {
    todo!();
}

pub fn refund_processed(_payment: Payment, _refund: Refund) -> Result<()> {
    todo!();
}

pub fn handle_webhook(_request: serde_json::Value) -> Result<()> {
    todo!();
}

fn main() -> Result<()> {
    Ok(())
}
