use crate::ast::expressions;
use crate::ast::expressions::Expr;

pub fn print_expression(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            right,
            operator,
        } => parenthesise(&operator.lexeme, &[left, right]),
        Expr::Unary { operator, right } => parenthesise(&operator.lexeme, &[right]),
        Expr::Grouping { expression } => parenthesise("group", &[expression]),
        Expr::Literal { value } => {
            format!("{:?}", value)
        }
        Expr::Assign { name, value } => parenthesise(&name.lexeme, &[value]),
        Expr::Logical {
            left,
            operator,
            right,
        } => parenthesise(&operator.lexeme, &[left, right]),
        Expr::FunctionCall {
            callee, arguments, ..
        } => {
            let mut refs: Vec<&Expr> = vec![callee];
            refs.extend(arguments.iter());
            parenthesise("call", &refs)
        }
        Expr::Get { object, name } => parenthesise(&name.lexeme, &[object]),
        Expr::Set {
            object,
            name,
            value,
        } => parenthesise(&name.lexeme, &[object, value]),
        Expr::Super { keyword, method } => {
            format!("(super {})", method.lexeme)
        }
        Expr::Variable { name } => name.lexeme.clone(),
        Expr::This { keyword } => keyword.lexeme.clone(),
        _ => String::from("Unrecognised expression"),
    }
}

fn parenthesise(name: &str, exprs: &[&Expr]) -> String {
    let mut res = String::new();
    res.push('(');
    res.push_str(name);

    for expr in exprs.iter() {
        res.push_str(" ");
        res.push_str(print_expression(*expr).as_str());
    }

    res.push(')');
    return res;
}
