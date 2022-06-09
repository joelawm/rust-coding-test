/*-------------
/db/transaction.rs

This is a Database file used to model out the transaction data.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use serde::Deserialize;

/// This is a transaction make up given invoming data from services.
/// 
/// # Info
/// * `t` - (String) This is the type of transaction 'deposit' or 'withdrawl'.
/// * `client` - (u16) Client ID is unique to each client.
/// * `tx` - (tx) Transactions are between two client accounts.
/// * `amount` - (f32) The amount is decimal precsion up to 4 places.
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
	#[serde(rename = "type")]
	pub t: TransactionType,
	#[serde(rename = "client")]
	pub client_id: u16,
	pub tx: u32,
    #[serde(default)]
	pub amount: Option<f32>,
    #[serde(skip_deserializing)]
	pub dispute: bool,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(t: TransactionType, client_id: u16, tx: u32, amount: Option<f32>) -> Self{
        Self {t, client_id, tx, amount, dispute: false}
    }
}

/// A enum representing all of the possible transaction types.
/// We set to lowercase for automatching so we dont have to impl.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}