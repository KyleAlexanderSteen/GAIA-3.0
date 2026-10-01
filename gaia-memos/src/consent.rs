//! #1152 consent and retention ledger. Not a surveillance store.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retention {
    Session,
    Days30,
    Days365,
    UntilDelete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentGrant {
    pub subject: String,
    pub purpose: String,
    pub retention: Retention,
    pub third_party: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessReceipt {
    pub subject: String,
    pub action: &'static str,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentError {
    MissingSubject,
    ThirdPartyDenied,
    NoGrant,
}

#[derive(Debug, Default)]
pub struct ConsentLedger {
    grants: HashMap<String, ConsentGrant>,
    receipts: Vec<AccessReceipt>,
}

impl ConsentLedger {
    pub fn grant(&mut self, g: ConsentGrant) -> Result<(), ConsentError> {
        if g.subject.trim().is_empty() {
            return Err(ConsentError::MissingSubject);
        }
        if g.third_party {
            return Err(ConsentError::ThirdPartyDenied);
        }
        self.receipts.push(AccessReceipt {
            subject: g.subject.clone(),
            action: "grant",
            note: g.purpose.clone(),
        });
        self.grants.insert(g.subject.clone(), g);
        Ok(())
    }

    pub fn access(&mut self, subject: &str) -> Result<AccessReceipt, ConsentError> {
        if !self.grants.contains_key(subject) {
            return Err(ConsentError::NoGrant);
        }
        let r = AccessReceipt {
            subject: subject.into(),
            action: "read",
            note: "human-readable access".into(),
        };
        self.receipts.push(r.clone());
        Ok(r)
    }

    pub fn delete(&mut self, subject: &str) -> Result<AccessReceipt, ConsentError> {
        if self.grants.remove(subject).is_none() {
            return Err(ConsentError::NoGrant);
        }
        let r = AccessReceipt {
            subject: subject.into(),
            action: "delete",
            note: "erasure requested".into(),
        };
        self.receipts.push(r.clone());
        Ok(r)
    }

    pub fn receipts(&self) -> &[AccessReceipt] {
        &self.receipts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn third_party_denied() {
        let mut l = ConsentLedger::default();
        let err = l.grant(ConsentGrant {
            subject: "a".into(),
            purpose: "share".into(),
            retention: Retention::Days30,
            third_party: true,
        });
        assert_eq!(err, Err(ConsentError::ThirdPartyDenied));
    }

    #[test]
    fn grant_access_delete() {
        let mut l = ConsentLedger::default();
        l.grant(ConsentGrant {
            subject: "kyle".into(),
            purpose: "local notes".into(),
            retention: Retention::UntilDelete,
            third_party: false,
        })
        .unwrap();
        assert_eq!(l.access("kyle").unwrap().action, "read");
        assert_eq!(l.delete("kyle").unwrap().action, "delete");
        assert!(l.access("kyle").is_err());
    }
}
