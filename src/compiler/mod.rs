mod analysis;
mod desugar;
mod ir;
//mod optimize;

use super::*;
use ir::Value;

pub use analysis::analyze;
pub use desugar::desugar;
pub use ir::hir;
pub use ir::mir;
//pub use optimize::eliminate_bindings;
//pub use optimize::Context;
