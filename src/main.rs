#![allow(dead_code)]

use Stmt::*;
use Expr::*;

#[derive(Debug, Clone)]
struct Ast(Vec<Stmt>);

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

use std::ops::Drop;

struct Scope<'a>(&'a mut Interpreter);

impl <'a> Scope<'a> {
    fn new(interpreter: &'a mut Interpreter) -> Self {
        interpreter.in_scope();
        Scope(interpreter)
    }
}

impl <'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        self.0.out_scope();
    }
}

use std::ops::{Deref, DerefMut};

impl <'a> Deref for Scope<'a> {
    type Target = Interpreter;
    fn deref(&self) -> &Interpreter {
        self.0
    }
}

impl <'a> DerefMut for Scope<'a> {
    fn deref_mut(&mut self) -> &mut Interpreter {
        self.0
    }
}


use std::collections::HashMap;

struct Interpreter {
    symbol_tables: Vec<HashMap<String, Expr>>,
    pos: usize,
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter { symbol_tables: Vec::new(), pos: 0 }
    }

    fn in_scope(&mut self) {
        let pos = self.pos;

        if self.symbol_tables.len() <= pos {
            self.symbol_tables.push(HashMap::new());
        } else {
            self.symbol_tables[pos-1].clear();
        }
        self.pos += 1;
    }

    fn out_scope(&mut self) {
        self.pos -= 1;
    }

    fn add_symbol(&mut self, name: String, expr: Expr) {
        let pos = self.pos - 1;
        self.symbol_tables[pos].insert(name, expr);
    }

    fn find_symbol(&self, name: &str) -> Expr {
        let pos = self.pos;
        for table in self.symbol_tables[0..pos].iter().rev() {
            if let Some(e) = table.get(name) {
                return e.clone()
            }
        }
        panic!("reference to unknown variable")
    }

    fn run(&mut self, ast: Ast) {

        let mut scope = Scope::new(self);

        // self.in_scope();

        for stmt in ast.0 {
            // self.run_stmt(stmt);
            scope.run_stmt(stmt);
        }

        // self.out_scope();

        // scope will be dropped and out_scope() will be called.
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
                let mut scope = Scope::new(self);
                // self.in_scope();
                for s in stmts {
                    // self.run_stmt(s);
                    scope.run_stmt(s);
                }
                // self.out_scope();
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

    let ast = Ast(vec![line1,
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
