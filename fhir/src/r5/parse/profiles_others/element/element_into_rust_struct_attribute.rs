//! Element into Rust struct attribute

use crate::SourceCodeString;
use crate::r5::parse::profiles_others::*;
use ::fhir_core::util::*;

/// Indent the start of each Rust attribute line of source code
const RUST_ATTRIBUTE_INDENT: &str = "    ";

/// Given one element, generate Rust struct attribute source code.
#[allow(dead_code)]
pub fn element_into_rust_struct_attribute(element: &Element) -> SourceCodeString {
    let name = last_word(&element.id);
    format!(
        "{}/// {}\n{}{}: ? // {}\n",
        RUST_ATTRIBUTE_INDENT,
        element
            .short
            .as_ref()
            .unwrap_or(&String::from("Short description goes here.")),
        RUST_ATTRIBUTE_INDENT,
        name,
        match element.r#type.as_ref() {
            Some(x) => x.first().unwrap().code.as_str(),
            None => "?",
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_into_rust_struct_attribute() {
        let element = Element {
            id: "Alfa.bravo".into(),
            short: Some(String::from("Short comment")),
            ..Element::default()
        };
        let actual = element_into_rust_struct_attribute(&element);
        let expect = concat!("    /// Short comment\n", "    bravo: ? // ?\n",);
        assert_eq!(actual, expect);
    }
}
