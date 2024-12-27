use std::any::TypeId;

#[derive(Clone, Debug)]
pub struct TypeSignature {
  pub name: String,
  pub id: TypeId,
  pub is_const: bool,
  pub is_mut: bool,
  pub is_unsafe: bool,
  pub is_impl_trait: bool,
  pub is_dyn_trait: bool,
  pub is_infer: bool,
  pub is_macro: bool,
  pub is_never: bool,
  pub is_paren: bool,
  pub is_path: bool,
  pub is_tuple: bool,
  pub is_array: bool,
  pub is_slice: bool,
  pub is_closure: bool,
  pub is_ref: bool,
  pub is_ptr: bool,
  pub is_verbatim: bool,
  pub len: Option<usize>,
  pub lifetimes: Vec<String>,
  pub children: Vec<TypeSignature>,
}