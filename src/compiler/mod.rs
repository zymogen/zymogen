mod analysis;
mod desugar;
mod ir;

use super::*;
pub use analysis::analyze;
pub use desugar::desugar;
pub use ir::hir;
pub use ir::mir;
