use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Result},
};

pub fn derive_component(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    Ok(quote! {
        impl ::kodanu_ecs::Component for #name {}
    })
}
