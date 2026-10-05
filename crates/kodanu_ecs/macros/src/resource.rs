use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Result},
};

pub fn derive_resource(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    Ok(quote! {
        impl ::kodanu_ecs::Resource for #name {}
    })
}
