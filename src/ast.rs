/// Expressions valid in the root of the story
#[derive(Debug)]
pub enum Expr {
    Var(Var),
    Line(Line),
    Knot(String),
    Stitch(String),
    Opt(Opt),
    Divert(String),
    /// The u16 represents depth
    Gather(u16),
    Todo(String),
    Conditional(Conditional),
}

/// A normal line in the story
#[derive(Debug)]
pub struct Line(pub Vec<LineExpr>);

/// Expressions valid inside of a line
#[derive(Debug)]
pub enum LineExpr {
    Text(String),
    Divert(String),
    Conditional(Conditional),
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
    pub depth: u16,
    pub condition: Option<Conditional>,
}

/// The kind of option
#[derive(Debug)]
pub enum OptKind {
    Plus,
    Star,
}

#[derive(Debug)]
pub struct Conditional(pub String);
