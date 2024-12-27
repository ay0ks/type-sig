use type_sig_type::TypeSignature;
use type_sig_proc_macro::type_sig;

pub fn type_sig<T: 'static>() -> TypeSignature {
  type_sig!(T)
}