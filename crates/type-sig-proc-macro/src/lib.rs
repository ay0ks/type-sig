extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
  self, 
  Expr, 
  ExprLit,
  Lit, 
  LitInt,
  Type, 
  TypeArray, 
  TypeBareFn,
  TypeGroup,
  TypeParen,
  TypePtr, 
  TypeReference,
  TypeSlice, 
  TypeTuple,
};

// type_sig format:
//
// NAME
// FLAGS...
// TYPE_ID
// ADDITIONAL_TYPE_IDS...

#[proc_macro]
pub fn type_sig(input: TokenStream) -> TokenStream {
  extern crate self as type_sig_proc_macro;
  let input_ = input.clone();
  let input__ = input_.clone();
  let input_: Type = syn::parse_macro_input!(input_);
  let ty_name = quote! { unsafe { std::any::type_name::<#input_>() } };
  let ty_id = quote! { unsafe { std::any::TypeId::of::<#input_>() } };
  let (
    mut is_const,
    mut is_mut,
    mut is_unsafe,
    mut is_impl_trait,
    mut is_dyn_trait,
    mut is_infer,
    mut is_macro,
    mut is_never,
    mut is_paren,
    mut is_group,
    mut is_path,
    mut is_array,
    mut is_slice,
    mut is_tuple,
    mut is_closure,
    mut is_ref,
    mut is_ptr,
    mut is_verbatim,
    mut ty_len,
  ) = (
    false, false, false, 
    false, false, false, 
    false, false, false, 
    false, false, false,
    false, false, false,
    false, false, false,
    None as Option<Expr>,
  );
  let mut ty_lts: Vec<String> = Vec::new();
  let mut ty_children: Vec<Expr> = Vec::new();
  match input_ {
    Type::Array(TypeArray {
      elem: ref ty,
      len,
      ..
    }) => {
      is_array = true;
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
      ty_len = Some(len);
    }
    Type::BareFn(TypeBareFn {
      unsafety: is_unsafe_,
      inputs: ref ty_args,
      output: ref ty_ret,
      ..
    }) => {
      is_closure = true;
      is_unsafe = is_unsafe_.is_some();
      for ty_arg in ty_args {
        let ty_child = type_sig(ty_arg.ty.clone().into_token_stream().into());
        let ty_child: Expr = syn::parse_macro_input!(ty_child);
        ty_children.push(ty_child);
      }
      let ty_child = type_sig(ty_ret.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::ImplTrait(_) => {
      is_impl_trait = true;
    }
    Type::Infer(_) => {
      is_infer = true;
    }
    Type::Macro(_) => {
      is_macro = true;
    }
    Type::Never(_) => {
      is_never = true;
    }
    Type::Group(TypeGroup {
      elem: ref ty,
      ..
    }) => {
      is_group = true;
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::Paren(TypeParen {
      elem: ref ty,
      ..
    }) => {
      is_paren = true;
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::Path(_) => {
      is_path = true;
    }
    Type::Ptr(TypePtr {
      const_token: is_const_,
      mutability: is_mut_,
      elem: ref ty,
      ..
    }) => {
      is_ptr = true;
      is_const = is_const_.is_some();
      is_mut = is_mut_.is_some();
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::Reference(TypeReference {
      lifetime: ref ty_lt,
      mutability: is_mut_,
      elem: ref ty,
      ..
    }) => {
      is_ref = true;
      is_const = is_mut_.is_none();
      is_mut = is_mut_.is_some();
      if let Some(ty_lt) = ty_lt {
        ty_lts.push(ty_lt.ident.to_string());
      }
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::Slice(TypeSlice {
      elem: ref ty,
      ..
    }) => {
      is_slice = true;
      let ty_child = type_sig(ty.into_token_stream().into());
      let ty_child: Expr = syn::parse_macro_input!(ty_child);
      ty_children.push(ty_child);
    }
    Type::TraitObject(_) => {
      is_dyn_trait = true;
    }
    Type::Tuple(TypeTuple {
      elems: ref ty_args,
      ..
    }) => {
      is_tuple = true;
      for ty_arg in ty_args {
        let ty_child = type_sig(ty_arg.into_token_stream().into());
        let ty_child: Expr = syn::parse_macro_input!(ty_child);
        ty_children.push(ty_child);
      }
      ty_len = Some(Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Int(LitInt::new(&ty_args.len().to_string(), Span::call_site())),
      }));
    }
    Type::Verbatim(_) => {
      is_verbatim = true;
    }
    _ => {}
  }

  let ty_len = match ty_len {
    Some(_) => quote! { Option::<usize>::Some(#ty_len) },
    None => quote! { Option::<usize>::None },
  };

  quote! {
    ::type_sig_type::TypeSignature {
      name: (#ty_name).to_string(),
      id: #ty_id,
      is_const: #is_const,
      is_mut: #is_mut,
      is_unsafe: #is_unsafe,
      is_impl_trait: #is_impl_trait,
      is_dyn_trait: #is_dyn_trait,
      is_infer: #is_infer,
      is_macro: #is_macro,
      is_never: #is_never,
      is_paren: #is_paren,
      is_path: #is_path,
      is_group: #is_group,
      is_tuple: #is_tuple,
      is_array: #is_array,
      is_slice: #is_slice,
      is_closure: #is_closure,
      is_ref: #is_ref,
      is_ptr: #is_ptr,
      is_verbatim: #is_verbatim,
      len: #ty_len,
      lifetimes: vec![#(#ty_lts),*],
      children: vec![#(#ty_children),*],
    }
  }.into()
}