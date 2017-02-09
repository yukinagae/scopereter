#![allow(dead_code)]

use Stmt::*;
use Expr::*;

#[derive(Debug, Clone)]
struct AST(Vec<Stmt>);

#[derive(Debug, Clone)]
enum Stmt {
    SubStmt(String, Expr),
    Print(Vec<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
enum Expr {
    Str(String),
    Num(isize),
    Var(String),
}

use std::collections::HashMap;

struct Interpreter {
    symbol_tables: Vec<HashMap<String, Expr>>,
    pos: usize,
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter { symbol_table: HashMap::new() }
    }

    fn add_symbol(&mut self, name: String, expr: Expr) {
        self.symbol_table.insert(name, expr);
    }

    fn find_symbol(&self, name: &str) -> Expr {
        self.symbol_table.get(name).expect("reference to unknown variable").clone()
    }

    fn run(&mut self, ast: AST) {
        for stmt in ast.0 {
            self.run_stmt(stmt);
        }
    }

    fn run_stmt(&mut self, stmt: Stmt) {
        // TODO
        match stmt {
            SubStmt(name, expr) => {
                let evaled_expr = self.eval(expr);
                self.add_symbol(name, evaled_expr);
            },
            Print(exprs) => {
                for expr in exprs {
                    self.print_expr(expr);
                }
                println!("");
            },
            Block(stmts) => {
                for s in stmts {
                    self.run_stmt(s);
                }
            },
        }
    }

    fn eval(&self, expr: Expr) -> Expr {
        match expr {
            Var(name) => self.find_symbol(&name),
            e @ Str(_) => e,
            e @ Num(_) => e,
        }
    }

    fn print_expr(&self, expr: Expr) {
        match self.eval(expr) {
            Str(ref s) => print!("{}", s),
            Num(ref n) => print!("{}", n),
            Var(_) => panic!("reference to unknown variable"),
        }
    }
}


fn main() {

    // input code
    // x = 1
    // y = 2
    // println("x = ", x)
    // println("y = ", y)

    // println("--")

    // {
    //     x = 3
    //     println("x = ", x)
    //     println("y = ", y)
    // }

    // println("--")

    // println("x = ", x)
    // println("y = ", y)

    let line1 = SubStmt("x".to_string(), Num(1));
    let line2 = SubStmt("y".to_string(), Num(2));
    let line3 = Print(vec![Str("x = ".to_string()), Var("x".to_string())]);
    let line4 = Print(vec![Str("y = ".to_string()), Var("y".to_string())]);
    let line5 = Print(vec![Str("--".to_string())]);
    let line6 = Block(vec![
                    SubStmt("x".to_string(), Num(3)),
                    Print(vec![Str("x = ".to_string()), Var("x".to_string())]),
                    Print(vec![Str("y = ".to_string()), Var("y".to_string())]),
                ]);
    let line7 = Print(vec![Str("--".to_string())]);
    let line8 = Print(vec![Str("x = ".to_string()), Var("x".to_string())]);
    let line9 = Print(vec![Str("y = ".to_string()), Var("y".to_string())]);

    let ast = AST(vec![line1,
                       line2,
                       line3,
                       line4,
                       line5,
                       line6,
                       line7,
                       line8,
                       line9]);

    let mut interpreter = Interpreter::new();

    interpreter.run(ast);
}
