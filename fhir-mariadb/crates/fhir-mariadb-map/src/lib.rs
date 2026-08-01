//! fhir-mariadb-map: the relational map model plus the generic engine that shreds
//! FHIR resources into rows and reconstructs them losslessly.

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
