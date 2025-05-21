use ruff_python_ast::{self as ast, name::Name, ExceptHandler, Stmt, Suite};
use ruff_python_parser::parse_module;

#[derive(Debug)]
pub struct ItemComplexity {
    pub scope: Vec<String>,
    pub name: String,
    pub complexity: usize,
}

pub fn module_complexities(source: &str) -> Vec<ItemComplexity> {
    let module = parse_module(&source).unwrap();

    let mut complexities = Vec::new();
    get_complexity_number(
        &module.into_suite(),
        &mut complexities,
        &mut Vec::new(),
        Context::Module,
    );
    complexities
}

#[derive(Clone, Copy, PartialEq)]
enum Context {
    Module,
    Class,
    Function,
}

fn get_complexity_number(
    suite: &Suite,
    complexities: &mut Vec<ItemComplexity>,
    scope: &mut Vec<Name>,
    context: Context,
) -> usize {
    let mut complexity = 0;

    for stmt in suite {
        match stmt {
            Stmt::If(ast::StmtIf {
                body,
                elif_else_clauses,
                ..
            }) => {
                complexity += 1;
                complexity += get_complexity_number(body, complexities, scope, context);
                for clause in elif_else_clauses {
                    if clause.test.is_some() {
                        complexity += 1;
                    }
                    complexity += get_complexity_number(&clause.body, complexities, scope, context);
                }
            }
            Stmt::For(ast::StmtFor { body, orelse, .. }) => {
                complexity += 1;
                complexity += get_complexity_number(body, complexities, scope, context);
                complexity += get_complexity_number(orelse, complexities, scope, context);
            }
            Stmt::With(ast::StmtWith { body, .. }) => {
                complexity += get_complexity_number(body, complexities, scope, context);
            }
            Stmt::While(ast::StmtWhile { body, orelse, .. }) => {
                complexity += 1;
                complexity += get_complexity_number(body, complexities, scope, context);
                complexity += get_complexity_number(orelse, complexities, scope, context);
            }
            Stmt::Match(ast::StmtMatch { cases, .. }) => {
                for case in cases {
                    complexity += 1;
                    complexity += get_complexity_number(&case.body, complexities, scope, context);
                }
                if let Some(last_case) = cases.last() {
                    // The complexity of an irrefutable pattern is similar to an `else` block of an `if` statement.
                    //
                    // For example:
                    // ```python
                    // match subject:
                    //     case 1: ...
                    //     case _: ...
                    //
                    // match subject:
                    //     case 1: ...
                    //     case foo: ...
                    // ```
                    if last_case.guard.is_none() && last_case.pattern.is_irrefutable() {
                        complexity -= 1;
                    }
                }
            }
            Stmt::Try(ast::StmtTry {
                body,
                handlers,
                orelse,
                finalbody,
                ..
            }) => {
                complexity += get_complexity_number(body, complexities, scope, context);
                if !orelse.is_empty() {
                    complexity += 1;
                }
                complexity += get_complexity_number(orelse, complexities, scope, context);
                complexity += get_complexity_number(finalbody, complexities, scope, context);
                for handler in handlers {
                    complexity += 1;
                    let ExceptHandler::ExceptHandler(ast::ExceptHandlerExceptHandler {
                        body, ..
                    }) = handler;
                    complexity += get_complexity_number(body, complexities, scope, context);
                }
            }
            Stmt::FunctionDef(ast::StmtFunctionDef { body, name, .. }) => {
                complexity += 1;
                complexity += get_complexity_number(body, complexities, scope, Context::Function);

                if context != Context::Function {
                    complexities.push(ItemComplexity {
                        scope: scope
                            .iter()
                            .map(|name| name.to_string())
                            .collect::<Vec<String>>(),
                        name: name.to_string(),
                        complexity,
                    })
                }
            }
            Stmt::ClassDef(ast::StmtClassDef { body, name, .. }) => {
                scope.push(name.id.clone());
                complexity += get_complexity_number(body, complexities, scope, Context::Class);
            }
            _ => {}
        }
    }

    complexity
}
