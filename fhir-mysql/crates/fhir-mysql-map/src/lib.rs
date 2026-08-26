//! fhir-mysql-map: the relational map model plus the generic engine that shreds
//! FHIR resources into rows and reconstructs them losslessly.
// Nothing here has any business dereferencing a raw pointer: this code
// parses and reshapes untrusted clinical data, and memory safety is the
// property that keeps a malformed resource from becoming a vulnerability.
#![forbid(unsafe_code)]

pub mod canon;
pub mod ddl;
pub mod error;
pub mod fold;
pub mod model;
pub mod reconstruct;
pub mod shred;
pub mod value;

pub use error::ShredError;
pub use model::{
    ColTy, Column, Elem, ElemKind, Node, Prim, PrimCol, RefCols, RelMap, ResourceMap, Table,
    TableKind,
};
pub use reconstruct::{InRow, ReconIn, reconstruct};
pub use shred::{DeepRow, ExtRow, Row, ShredOut, SqlVal, shred};
pub use value::LeafVal;
