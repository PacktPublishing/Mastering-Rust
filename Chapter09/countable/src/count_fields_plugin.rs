extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(Countable)]
pub fn countable_fields(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let expanded = expand_countable_fields(&ast);

    expanded.parse().unwrap()
//    quote!(#ast #expanded).to_string().parse().unwrap()
}

fn expand_countable_fields(ast: &syn::MacroInput) -> quote::Tokens {
    let n = match ast.body {
        syn::Body::Struct(ref data) => data.fields().len(),
        syn::Body::Enum(_) => panic!("#[derive(Countable)] can only be used with structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::Countable for #name #ty_generics #where_clause {
            fn count_fields(&self) -> usize {
                #n
            }
        }
    }
}
