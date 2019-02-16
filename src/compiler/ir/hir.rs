pub type Sequence = Vec<Expression>;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Expression {
    Derived(DerivedExpr),
    Primitive(PrimitiveExpr),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct LetBindings {
    var: String,
    expr: Expression,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LetExpr {
    Let(Vec<LetBindings>, Sequence),
    NamedLet(String, Vec<LetBindings>, Sequence),
    Letstar(Vec<LetBindings>, Sequence),
}

/// Expressions that are not semantically primitive, e.g. they can be expressed
/// in terms of [`PrimitiveExpr`]'s.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum DerivedExpr {
    Let(LetExpr),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
/// Semantic primitives
pub enum PrimitiveExpr {
    Literal(Literal),
    Variable(String),
    Quotation,
    Call,
    Lambda,
    Conditional,
    Assignment,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Literal {
    Quote(Box<Literal>),
    Boolean(bool),
    Integer(i64),
    String(String),
    Identifier(String),
    List(Vec<Literal>),
}
