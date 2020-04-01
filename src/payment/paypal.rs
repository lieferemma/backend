//! https://developer.paypal.com/docs/checkout/reference/server-integration/get-transaction/
//! https://developer.paypal.com/docs/integration/direct/payments/paypal-payments/#request-1
//! curl -v -X GET https://api.sandbox.paypal.com/v1/payments/payment/PAY-1B56960729604235TKQQIYVY \
//!   -H "Content-Type: application/json" \
//!   -H "Authorization: Bearer Access-Token"

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::str::FromStr;
use url::Url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_transaction_id() {
        assert!(matches!(TransactionId::from_str("PAY-TOOSHORT"), Err(_)));
        assert!(matches!(
            TransactionId::from_str("PAY-TOOLONGSTRINGNOTFITTINGAVALIDTRANSACTIONID"),
            Err(_)
        ));
        assert!(matches!(
            TransactionId::from_str("PAY-1B56960729604235TKQQIY/Y"),
            Err(_)
        ));
        assert!(matches!(
            TransactionId::from_str("PAY-1B56960729604235TKQQIYVY"),
            Ok(_)
        ));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransactionId(String);

impl FromStr for TransactionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 28 {
            return Err(anyhow!(
                "Invalid transaction id length {} != 28: {}",
                s.len(),
                s
            ));
        }
        if &s[0..=3] != "PAY-" {
            return Err(anyhow!("Invalid prefix of transaction id: {}", s));
        }

        for c in s[4..=27].chars() {
            if !c.is_digit(10) && !c.is_ascii_uppercase() {
                return Err(anyhow!(
                    "Invalid character in suffix of transaction id: {}",
                    s
                ));
            }
        }

        Ok(TransactionId(s.to_string()))
    }
}

#[derive(Deserialize)]
struct Payment {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "create_time")]
    create_time: String,
    #[serde(rename = "update_time")]
    update_time: String,
    #[serde(rename = "state")]
    state: String,
    #[serde(rename = "intent")]
    intent: String,
    #[serde(rename = "payer")]
    payer: Payer,
    #[serde(rename = "transactions")]
    transactions: Vec<Transaction>,
}

#[derive(Deserialize)]
struct Payer {
    #[serde(rename = "payment_method")]
    payment_method: String,
}

#[derive(Deserialize)]
struct Money {
    #[serde(rename = "currency_code")]
    currency_code: String,
    #[serde(rename = "value")]
    value: String,
}

#[derive(Deserialize)]
struct Transaction {
    #[serde(rename = "amount")]
    amount: Money,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "note_to_payer")]
    note_to_payer: String,
}

// fn verify_transaction(transaction_id: TransactionId) -> Result<()> {
//     let paypal_api = Url::parse("git")?;
//     let paypal_url = paypal_api.join(&transaction_id.0);
//     let body = reqwest::get(paypal_url)
//     .await?
//     .text()
//     .await?;
// }
