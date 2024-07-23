extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Error, ItemFn, ReturnType, Visibility};

/// This macro is used to define a public function that can be called from the seq wasm runtime.
/// The function must be declared as `pub fn function_name() {}` without any additional modifiers (unsafe, extern, const, async, etc.).
/// It will be exported with the same name as the function name.
/// with the signature `pub extern "C" fn function_name(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool`.
/// The function should return a boolean value indicating whether the function executed successfully.
/// it will unpack msg_sender and block_time_stamp from the TxContext; these values can be used in the function body.
#[proc_macro_attribute]
pub fn public(_metadata: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Check if function declared is public, has no inputs, no return type, and no additional modifiers
    let is_valid = matches!(input_fn.vis, Visibility::Public(_))
        && input_fn.sig.inputs.is_empty()
        && matches!(input_fn.sig.output, ReturnType::Default)
        && input_fn.sig.unsafety.is_none()
        && input_fn.sig.abi.is_none()
        && input_fn.sig.constness.is_none()
        && input_fn.sig.asyncness.is_none();

    if !is_valid {
        // If the function doesn't meet the criteria, return an error
        let error_message = "Function must be declared as `pub fn function_name() {}` without any additional modifiers (unsafe, extern, const, async, etc.)";
        return Error::new_spanned(input_fn.sig, error_message)
            .to_compile_error()
            .into();
    }

    let function_name = input_fn.sig.ident;
    let function_body = input_fn.block;
    let doc_attrs: Vec<&Attribute> = input_fn
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .collect();
    TokenStream::from(quote! {
        #(#doc_attrs)*
        #[cfg_attr(all(target_arch = "wasm32"), export_name = stringify!(#function_name))]
        #[no_mangle]
        pub extern "C" fn #function_name(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
            let tx_ctx = TxContext::unpack(tx_context);
            let msg_sender = tx_ctx.msg_sender();
            let block_time_stamp = tx_ctx.time_stamp();
            #function_body
        }
    })
}
