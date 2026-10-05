use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Result},
};

pub fn derive_event(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    Ok(quote! {
        impl ::kodanu_ecs::Event for #name {}
    })
}
