use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Attribute, Meta, Expr, ExprAssign, ExprLit};

#[proc_macro_derive(HelloMacro, attributes(hello_macro))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

        // Handle the struct's fields
        let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(ref fields), .. }) = ast.data {
            fields
        } else {
            panic!("HelloMacro can only be derived for structs with named fields");
        };
        
    
        let mut field_greetings = Vec::new();
    
        // Iterate over the struct's fields
        for field in &fields.named {
            let field_name = &field.ident;
    
            // Get the greeting from the field's attributes, or use a default greeting
            let greeting = get_greeting_from_attributes(&field.attrs).unwrap_or_else(|| {
                format!("Hello, field {}!", field_name.as_ref().unwrap())
            });
    
            // Generate code to print the field's greeting
            field_greetings.push(quote! {
                println!("Field {}: {}", stringify!(#field_name), #greeting);
            });
        }

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
                #(#field_greetings)*
            }
        }
    };
    gen.into()
}

fn get_greeting_from_attributes(attrs: &[Attribute]) -> Option<String> {
    // Iterate over the attributes to find the `hello_macro` attribute
    for attr in attrs {
        if attr.path().is_ident("hello_macro") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the tokens inside the attribute to extract the greeting
                if let Some(greeting) = parse_greeting_from_tokens(&meta_list.tokens) {
                    return Some(greeting);
                }
            }
        }
    }
    None
}

fn parse_greeting_from_tokens(tokens: &proc_macro2::TokenStream) -> Option<String> {
    // Parse the tokens into an expression
    if let Ok(expr) = syn::parse2::<Expr>(tokens.clone()) {
        // Check if the expression is an assignment (e.g., `greeting = "Large size!"`)
        if let Expr::Assign(ExprAssign { left, right, .. }) = expr {
            // Check if the left-hand side is `greeting`
            if let Expr::Path(path) = &*left {
                if path.path.is_ident("greeting") {
                    // Check if the right-hand side is a string literal
                    if let Expr::Lit(ExprLit { lit: syn::Lit::Str(lit_str), .. }) = &*right {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}