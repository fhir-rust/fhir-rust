//! PublicationStatus
//!
//! TODO

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicationStatus {}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PublicationStatus;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {};
        assert_eq!(actual, expect);
    }
}
