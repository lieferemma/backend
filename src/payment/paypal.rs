// https://developer.paypal.com/docs/integration/direct/payments/paypal-payments/#request-1
// curl -v -X GET https://api.sandbox.paypal.com/v1/payments/payment/PAY-1B56960729604235TKQQIYVY \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer Access-Token"

use url::{Url};
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_transaction_id() {
        assert!(matches!(TransactionId::from_str("PAY-TOOSHORT"), Err(_)));
        assert!(matches!(TransactionId::from_str("PAY-TOOLONGSTRINGNOTFITTINGAVALIDTRANSACTIONID"), Err(_)));
        assert!(matches!(TransactionId::from_str("PAY-1B56960729604235TKQQIY/Y"), Err(_)));
        assert!(matches!(TransactionId::from_str("PAY-1B56960729604235TKQQIYVY"), Ok(_)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransactionId(String);

impl FromStr for TransactionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 28 {
            return Err(anyhow!("Invalid transaction id length {} != 28: {}", s.len(), s));
        }
        if &s[0..=3] != "PAY-" {
            return Err(anyhow!("Invalid prefix of transaction id: {}", s));
        }

        for c in s[4..=27].chars(){
            if !c.is_digit(10) && !c.is_ascii_uppercase() {
                return Err(anyhow!("Invalid character in suffix of transaction id: {}", s));
            }
        }

        Ok(TransactionId(s.to_string()))
    }
}

// fn verify_transaction(transaction_id: TransactionId) -> Result<()> {
//     let paypal_api = Url::parse("https://api.sandbox.paypal.com/v1/payments/payment/")?;
//     let paypal_url = paypal_api.join(transaction_id);
//     let body = reqwest::get("https://www.rust-lang.org")
//     .await?
//     .text()
//     .await?;
// }
