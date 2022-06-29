extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Ident, ItemMacro};

pub fn transform(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as ItemMacro);
  let dsl = ast.mac.tokens.to_string();
  let re_a = Regex::new(r"[^{]*\{([^\}]*)\}.*").unwrap();
  let re_b = Regex::new(r"\sget\(fn\s+([^\)]+)\)").unwrap();
  let mut tokens = quote!();
  if let Some(capts) = re_a.captures(&dsl) {
    let mat_a = capts.get(1).unwrap();
    for cap in re_b.captures_iter(mat_a.as_str()) {
      let mat_b = cap.get(1).unwrap();
      let ident = Ident::new(mat_b.as_str(), Span::call_site());
      tokens = quote!(#tokens macro_rules! #ident(($($t:tt)*) => { Self::#ident($($t)*) }););
    }
  }
  TokenStream::from(quote!(#ast #tokens))
}
