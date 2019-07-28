use peg::parser;

mod ast;
use ast::*;

parser! {
grammar ink_parser() for str {
    // Parse the file into a set of `Expr`s
    pub rule parse() -> Vec<Expr> =
        wn() exprs:(
            var:var()       { Expr::Var(var) } /
            knot:knot()     { Expr::Knot(knot) } /
            stitch:stitch() { Expr::Stitch(stitch) } /
            option:option() { Expr::Opt(option) } /
            divert:divert() { Expr::Divert(divert) } /
            cond:cond()     { Expr::Conditional(cond) } /
            todo:todo()     { Expr::Todo(todo) } /
            gather() /
            line:line()     { Expr::Line(line) }
        ) ** wn() /*Catch everything else after this temporarily*/ [_]* ![_] { exprs }

    // Whitespace character
    rule whitespace_char() = ['\t' | ' ']

    // Line comment
    rule line_comment() = "//" (!"\n" [_])* ("\n" / ![_])

    // Inline comment
    rule inline_comment() = "/*" (!"*/" [_])* "*/"

    // Whitespace including comments
    rule w() = (whitespace_char() / inline_comment())*

    // Whitespace including newlines and line comments
    rule wn() = (whitespace_char() / "\n" / inline_comment() / line_comment())*

    // A variable name
    rule identifier() = ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*

    // CONST/VAR
    rule var() -> Var =
        w() def:$("CONST" / "VAR") w() name:$identifier() w() "=" w() value:$((!"\n" [_])*) w() "\n"
        {
            Var {
                name: name.into(),
                value: value.into(),
                constant: def == "CONST",
            }
        }

    // A line
    rule line() -> Line =
        exprs:(
            text:text() { LineExpr::Text(text.into()) } /
            text_stop:text_stop() { text_stop }
        )+
        "\n"
        { Line(exprs) }

    // Tokens that stop text parsing in a line
    rule text_stop() -> LineExpr = 
        divert:divert() { LineExpr::Divert(divert) } /
        conditional:cond() { LineExpr::Conditional(conditional) } /
        glue()

    // Normal text inside of a line
    rule text() -> String = text:$((!(text_stop() / "\n") [_])+)
        { text.into() }

    // A divert
    rule divert() -> String = "-> " name:$identifier() { name.into() }

    // Glue
    rule glue() -> LineExpr = g:"<>" { LineExpr::Glue }

    // A knot
    rule knot() -> String = w() "===" w() name:$identifier() w() "\n" { name.into() }

    // A stitch
    rule stitch() -> String = w() "=" w() name:$identifier() w() "\n" { name.into() }
    
    // A gather
    rule gather() -> Expr = w() n:$("-" " "?)+ { Expr::Gather(n.len() as u16) }

    // An option
    rule option() -> Opt = w() def:($("*" " "?)+ / $("+" " "?)+) w() cond:cond()? " "? line:line()
        {
            Opt {
                line,
                option_kind: match def[0].trim() {
                    "*" => OptKind::Star,
                    "+" => OptKind::Plus,
                    _ => unreachable!()
                },
                depth: def.len() as u16,
                condition: cond,
            }
        }

    // A todo
    rule todo() -> String = "TODO:" todo:$((!"\n" [_])*) ("\n" / ![_]) { todo.into() }
    
    // A conditional
    rule cond() -> Conditional = "{" content:$((!"}" [_])*) "}" { Conditional(content.into()) }
}}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    let contents = std::fs::read_to_string(file)?;

    println!("Dumping AST for file: {}", file);

    let ast = ink_parser::parse(&contents);

    println!("{:#?}", ast);

    Ok(())
}
