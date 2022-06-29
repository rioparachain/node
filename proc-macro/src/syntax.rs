extern crate proc_macro;
use std::string::String;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{
  parse_macro_input, parse_quote,
  visit_mut::{self, VisitMut},
  BinOp, Expr, ExprClosure, ItemFn, Lit, Macro, Pat, Stmt,
};

pub fn transform(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let mut ast = parse_macro_input!(input as ItemFn);
  Syntax::default().visit_item_fn_mut(&mut ast);
  TokenStream::from(ast.to_token_stream())
}

struct ReplacePath {
  name: String,
  expr: Expr,
}

impl VisitMut for ReplacePath {
  fn visit_expr_mut(&mut self, node: &mut Expr) {
    match node {
      | Expr::Path(a) => {
        if a.path.segments.first().unwrap().ident == self.name {
          *node = self.expr.clone();
        }
      }
      | _ => {
        visit_mut::visit_expr_mut(self, node);
      }
    }
  }

  fn visit_macro_mut(&mut self, node: &mut Macro) {
    let mut ast = syn::parse_macro_input::parse::<Expr>(node.tokens.clone().into()).unwrap();
    self.visit_expr_mut(&mut ast);
    let mut tokens = quote!();
    match ast {
      | Expr::Block(a) => {
        for stmt in a.block.stmts {
          tokens = quote!(#tokens#stmt);
        }
      }
      | _ => {
        tokens = quote!(#ast);
      }
    }
    node.tokens = tokens.into();
  }
}

struct Syntax {
  expr_stack: Vec<Option<Expr>>,
}

impl Default for Syntax {
  fn default() -> Self { Syntax { expr_stack: vec![] } }
}

impl VisitMut for Syntax {
  fn visit_stmt_mut(&mut self, node: &mut Stmt) {
    if let Stmt::Local(local) = node {
      if let Some((_, expr)) = &mut local.init {
        match *expr.clone() {
          | Expr::Closure(cls) => match cls.inputs.first() {
            | Some(Pat::Lit(a)) => match &*a.expr {
              | Expr::Lit(b) => match &b.lit {
                | Lit::Str(c) => {
                  let mut body = cls.body.clone();
                  let d = c.value();
                  match d.as_str() {
                    | "fxp" => {
                      self.visit_expr_mut(&mut body);
                      *expr = body;
                      *node = parse_quote!(rio_syntax_rewrite!(lwa,"fxp",#node););
                    }

                    | "fxp?" => {
                      self.visit_expr_mut(&mut body);
                      *expr = body;
                      *node = parse_quote!(rio_syntax_rewrite!(lwa,"fxp?",#node););
                    }

                    | _ => (),
                  }
                }
                | _ => (),
              },
              | _ => (),
            },
            | _ => (),
          },
          | _ => (),
        }
      }
    }
    visit_mut::visit_stmt_mut(self, node);
  }

  fn visit_expr_mut(&mut self, node: &mut Expr) {
    self.expr_stack.push(None);
    visit_mut::visit_expr_mut(self, node);
    match self.expr_stack.last().unwrap() {
      | None => (),
      | Some(expr) => {
        *node = expr.clone();
      }
    }
    self.expr_stack.pop();
    match node {
      | Expr::Binary(eb) => {
        let lhs = &eb.left;
        let op = &eb.op;
        let rhs = &eb.right;
        match op {
          | BinOp::Add(_) => {
            *node = parse_quote!(((#lhs).checked_add(#rhs).ok_or(
                sp_runtime::DispatchError::Arithmetic(sp_runtime::ArithmeticError::Overflow))?));
          }

          | BinOp::Sub(_) => {
            *node = parse_quote!(((#lhs).checked_sub(#rhs).ok_or(
                sp_runtime::DispatchError::Arithmetic(sp_runtime::ArithmeticError::Overflow))?));
          }

          | BinOp::Mul(_) => {
            *node = parse_quote!(((#lhs).checked_mul(#rhs).ok_or(
                sp_runtime::DispatchError::Arithmetic(sp_runtime::ArithmeticError::Overflow))?));
          }

          | BinOp::Div(_) => {
            *node = parse_quote!(((#lhs).checked_div(#rhs).ok_or(
                sp_runtime::DispatchError::Arithmetic(sp_runtime::ArithmeticError::DivisionByZero))?));
          }

          | _ => (),
        }
      }
      /*
            | Expr::Binary(eb) => {
              use BinOp::*;
              let lhs = &eb.left;
              let op = &eb.op;
              let rhs = &eb.right;
              match op {
                | Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_) | AddEq(_) | SubEq(_) | MulEq(_) | DivEq(_)
                | RemEq(_) => {
                  *node = parse_quote!(rio_syntax_rewrite!(bie,(#op),#lhs,#rhs,#node));
                }
                | _ => (),
              }
            }
      */
      | _ => (),
    }
  }

  fn visit_expr_closure_mut(&mut self, node: &mut ExprClosure) {
    match node.inputs.first() {
      | Some(Pat::Lit(a)) => match &*a.expr {
        | Expr::Lit(b) => match &b.lit {
          | Lit::Str(c) => {
            let body = &mut node.body;
            self.visit_expr_mut(body);
            let d = c.value();
            if d != "fxp" && d != "fxp?" {
              let expr = match (d.chars().nth(0), d.chars().nth(d.len() - 1)) {
                | (Some('.'), _) => {
                  let name = String::from("jt62eeeylffpu");
                  let code = format!("({}){}", name, d);
                  let mut expr = syn::parse_str::<Expr>(&code).unwrap();
                  ReplacePath { name, expr: *body.clone() }.visit_expr_mut(&mut expr);
                  expr.clone()
                }

                | (Some(_a), Some('!')) => {
                  let name = String::from("lih2cr4qknriy");
                  let re = Regex::new(r"\s(\$)\s").unwrap();
                  let mut close = String::new();
                  let mut d2 = d.clone();
                  for cap in re.captures_iter(&d) {
                    let mat = cap.get(1).unwrap();
                    d2.replace_range(mat.start()..mat.end(), "(");
                    close.push(')');
                  }
                  let _fix = false;
                  let code = match &**body {
                    | Expr::Block(_) => {
                      format!("{}{{{}}}{}", d2, name, close)
                    }
                    | _ => format!("{}({}){}", d2, name, close),
                  };
                  let mut expr = syn::parse_str::<Expr>(&code).unwrap();
                  ReplacePath { name, expr: *body.clone() }.visit_expr_mut(&mut expr);
                  expr.clone()
                }

                | (Some(_a), _) => {
                  let name = String::from("lih2cr4qknriy");
                  let re = Regex::new(r"\s(\$)\s").unwrap();
                  let mut close = String::new();
                  let mut d2 = d.clone();
                  for cap in re.captures_iter(&d) {
                    let mat = cap.get(1).unwrap();
                    d2.replace_range(mat.start()..mat.end(), "(");
                    close.push(')');
                  }
                  let code = format!("{}({}){}", d2, name, close);
                  let mut expr = syn::parse_str::<Expr>(&code).unwrap();
                  ReplacePath { name, expr: *body.clone() }.visit_expr_mut(&mut expr);
                  expr.clone()
                }

                | _ => unimplemented!(),
              };
              *self.expr_stack.last_mut().unwrap() = Some(expr.clone());
            }
          }
          | _ => (),
        },
        | _ => (),
      },
      | _ => {
        visit_mut::visit_expr_closure_mut(self, node);
      }
    }
  }
}
