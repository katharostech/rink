/// Expressions valid in the root of the story
#[derive(Debug)]
pub enum Expr {
    Var(Var),
    Line(Line),
    Knot(String),
    Stitch(String),
    Opt(Opt),
    Divert(String),
    Gather,
}

/// A normal line in the story
#[derive(Debug)]
pub struct Line(pub Vec<LineExpr>);

/// Expressions valid inside of a line
#[derive(Debug)]
pub enum LineExpr {
    Text(String),
    Divert(String),
    Glue,
}

/// Variable or const definition
#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub value: String,
    pub constant: bool,
}

/// An option line
#[derive(Debug)]
pub struct Opt {
    pub line: Line,
    pub option_kind: OptKind,
}

/// The kind of option
#[derive(Debug)]
pub enum OptKind {
    Plus,
    Star,
}
