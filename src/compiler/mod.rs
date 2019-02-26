use super::*;

mod ir;
use ir::Value;

mod analysis;
mod desugar;
mod normalize;
mod symbol;
mod bytecode;

pub use analysis::analyze;
pub use desugar::desugar;
pub use ir::hir;
pub use ir::mir;
pub use normalize::{normalize_expr};
pub use symbol::{Symbol, SymbolTable};
pub use bytecode::{Context};
