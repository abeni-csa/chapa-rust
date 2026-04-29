//! Models related to balace .
use crate::models::bank::Currency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Data Structer for Swap API
#[derive(Debug, Serialize)]
pub struct SwapOptions {
    /// The ammout of swapable balace
    pub amount: u64,
    /// The currency of the balance swaped form (e.g., "ETB", "USD").
    pub from: Currency,
    /// The currency of the balance to swap (e.g., "ETB", "USD").
    pub to: Currency,
}
/// Data Structer for Balace Represenation
#[derive(Debug, Deserialize)]
pub struct Balance {
    /// The currency for the balance (e.g., "ETB", "USD").
    pub currency: Currency,
    /// This is the amount that is available for immediate use.
    /// You can use this balance for transfers, withdrawals, and other transactions.
    pub available_balance: f64,
    ///This represents the balance in your account,
    /// which are funds that are not yet available for use (funds that have not yet been settled).
    pub ledger_balance: f64,
}
/// Data Structure for Swap API for converting USD to ETB (Ethiopian Birr).
#[derive(Debug, Deserialize)]
pub struct SwapData {
    /// The status of the swap balace.
    pub status: String,
    ///  The reference ID of the swap procces (Chapa reference)
    pub ref_id: String,
    /// The currency of the balance swaped form (e.g., "ETB", "USD").
    pub from_currency: Currency,
    /// The currency of the balance to swap (e.g., "ETB", "USD").
    pub to_currency: Currency,
    /// The ammout of swapable balace
    pub amount: f64,
    /// total amoute of exchage
    pub exchanged_amount: u64,
    /// amout of charege by chapa
    pub charge: u32,
    /// rate fo convetion rate (dev NOTE:to be honest idk what this mean )
    pub rate: f64,
    /// The creation timestamp of the swap entry.
    pub created_at: DateTime<Utc>,
    /// The last updated timestamp of the swap entry.
    pub updated_at: DateTime<Utc>,
}
