
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Operation {
    Bind(usize),
    /// Reference an unbound variable
    Var(String), 
    /// Reference a bound variable   
    Bound(usize),
    /// Reference to constant table
    Constant(usize),
    JumpNotEqual(usize),
    /// Call procedure with N-args
    Call(usize),
}
