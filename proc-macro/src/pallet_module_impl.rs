extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit::Visit, Ident, ImplItemMethod, ItemImpl};

pub fn transform(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as ItemImpl);
  let mut pm = PalletModule::default();
  let mut tokens = quote!();
  pm.visit_item_impl(&ast);
  for (ident, arity) in pm.methods {
    let name = ident.clone().to_string();
    if name.len() < 5 {
      continue;
    }
    let mut end = quote!();
    let macro_ident = if &name[0..4] == "try_" {
      end = quote!(?);
      Ident::new(&name[4..name.len()], ident.span())
    } else {
      ident.clone()
    };
    match arity {
      | 0 => {
        tokens = quote!(#tokens macro_rules! #macro_ident(() => { GEN_PATH!(Pallet,#ident)()#end }););
      }
      | 1 => {
        tokens =
          quote!(#tokens macro_rules! #macro_ident(($a:expr) => { GEN_PATH!(Pallet,#ident)($a)#end }););
      }
      | 2 => {
        tokens = quote!(#tokens macro_rules! #macro_ident(($a:expr,$b:expr) => { GEN_PATH!(Pallet,#ident)($a,$b)#end }););
      }
      | 3 => {
        tokens = quote!(#tokens macro_rules! #macro_ident(
                 ($a:expr,$b:expr,$c:expr) => { GEN_PATH!(Pallet,#ident)($a,$b,$c)#end }
             ); );
      }
      | _ => unimplemented!(),
    }
  }
  TokenStream::from(quote!(#tokens #ast))
}

struct PalletModule {
  methods: Vec<(Ident, usize)>,
}

impl Default for PalletModule {
  fn default() -> Self { PalletModule { methods: vec![] } }
}

impl Visit<'_> for PalletModule {
  fn visit_impl_item_method(&mut self, node: &ImplItemMethod) {
    let ident = &node.sig.ident;
    let arity = node.sig.inputs.len();
    self.methods.push((ident.clone(), arity));
  }
}
