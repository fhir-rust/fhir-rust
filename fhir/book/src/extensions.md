# Extensions

FHIR extensions are a `Vec<Extension>` keyed by `url`. The `ExtensionExt` trait
(in the prelude) adds the everyday operations to every resource and datatype
that carries extensions.

```rust
use fhir::r5::resources::Patient;
use fhir::r5::types::Extension;
use fhir::r5::types::String as FhirString;
use fhir::r5::extension_ext::ExtensionExt;

const EYE_COLOR: &str = "http://example.org/eye-color";

let mut patient = Patient::default();

// Set an extension (replaces any existing one with the same url).
patient.set_extension(Extension {
    url: FhirString(EYE_COLOR.to_string()),
    ..Default::default()
});

// Read it back by url.
assert!(patient.extension(EYE_COLOR).is_some());
assert!(patient.extension("http://other").is_none());
```

The trait methods:

- `extension(url) -> Option<&Extension>` — the first with that url.
- `extensions(url) -> Vec<&Extension>` — all with that url.
- `set_extension(ext)` — set, replacing any with the same url.
- `add_extension(ext)` — append without replacing.

A parallel `ModifierExtensionExt` trait provides the same operations for
`modifierExtension` on resources and backbone elements.

## Extension values

An extension's value is `value[x]` — the `ExtensionValue` choice enum. Primitive
variants carry the paired `_value<Type>` extension via `Primitive<T>`:

```rust
use fhir::r5::types::Extension;
use fhir::r5::types::extension::ExtensionValue;
use fhir::r5::choice::Primitive;
use fhir::r5::types::String as FhirString;

let ext = Extension {
    url: FhirString("http://example.org/note".to_string()),
    value: Some(ExtensionValue::String(Primitive::new(FhirString("hi".into())))),
    ..Default::default()
};
assert_eq!(serde_json::to_value(&ext).unwrap()["valueString"], "hi");
```

See [Primitive extensions](json-serialization.md#primitive-extensions-_field)
for the `_field` sibling representation on ordinary primitive elements.
