//! Primitive

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    /// Short description
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Primitive;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            description: String::default(),
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "description": ""
                }
            );
            let actual: Primitive = ::serde_json::from_value(json).expect("from_value");
            let expect: Primitive = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
                {
                    "description": ""
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
