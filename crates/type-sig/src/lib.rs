pub use type_sig_type::TypeSignature;
pub use type_sig_proc_macro::type_sig;

#[cfg(test)]
mod tests {
  use super::{type_sig, TypeSignature};

  #[test]
  pub fn test_001() {
    let sig = type_sig!(i32);
    assert_eq!(sig.name, "i32");
    assert_eq!(sig.is_const, false);
    assert_eq!(sig.is_mut, false);
    assert_eq!(sig.is_unsafe, false);
    assert_eq!(sig.is_impl_trait, false);
    assert_eq!(sig.is_dyn_trait, false);
    assert_eq!(sig.is_infer, false);
    assert_eq!(sig.is_macro, false);
    assert_eq!(sig.is_never, false);
    assert_eq!(sig.is_paren, false);
    assert_eq!(sig.is_path, true);
    assert_eq!(sig.is_array, false);
    assert_eq!(sig.is_slice, false);
    assert_eq!(sig.is_tuple, false);
    assert_eq!(sig.is_closure, false);
    assert_eq!(sig.is_ref, false);
    assert_eq!(sig.is_ptr, false);
    assert_eq!(sig.is_verbatim, false);
    assert_eq!(sig.len, None);
    assert_eq!(sig.lifetimes, Vec::<String>::new());
    assert_eq!(sig.children, Vec::<TypeSignature>::new());
  }

  #[test]
  pub fn test_002() {
    let sig = type_sig!(*const i32);
    let sig_child = type_sig!(i32);
    assert_eq!(sig.name, "*const i32");
    assert_eq!(sig.is_const, true);
    assert_eq!(sig.is_mut, false);
    assert_eq!(sig.is_unsafe, false);
    assert_eq!(sig.is_impl_trait, false);
    assert_eq!(sig.is_dyn_trait, false);
    assert_eq!(sig.is_infer, false);
    assert_eq!(sig.is_macro, false);
    assert_eq!(sig.is_never, false);
    assert_eq!(sig.is_paren, false);
    assert_eq!(sig.is_path, false);
    assert_eq!(sig.is_array, false);
    assert_eq!(sig.is_slice, false);
    assert_eq!(sig.is_tuple, false);
    assert_eq!(sig.is_closure, false);
    assert_eq!(sig.is_ref, false);
    assert_eq!(sig.is_ptr, true);
    assert_eq!(sig.is_verbatim, false);
    assert_eq!(sig.len, None);
    assert_eq!(sig.lifetimes, Vec::<String>::new());
    assert_eq!(sig.children, Vec::<TypeSignature>::from([sig_child]));
  }
}
