//! Command macro implementation logic.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse::Parser,
    punctuated::Punctuated,
    token::Comma,
    Error, Expr, ExprLit, ItemFn, Lit, Meta, Result,
};

/// Metadata parsed from the `#[command(...)]` attributes.
struct CommandMetadata {
    name: String,
    syntax: String,
    short: String,
    docs: String,
}

impl CommandMetadata {
    /// Parses command metadata from attribute arguments.
    fn parse(input: TokenStream) -> Result<Self> {
        let parser = Punctuated::<Meta, Comma>::parse_terminated;
        let args = parser.parse2(input)?;

        let mut name = None;
        let mut syntax = None;
        let mut short = None;
        let mut docs = None;

        for meta in args {
            if let Meta::NameValue(nv) = meta {
                let ident = nv.path.get_ident().ok_or_else(|| {
                    Error::new_spanned(&nv.path, "Expected identifier")
                })?;

                // Extract string value from Expr
                let value = match &nv.value {
                    Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => s.value(),
                    _ => return Err(Error::new_spanned(&nv.value, "Expected string literal")),
                };

                match ident.to_string().as_str() {
                    "name" => name = Some(value),
                    "syntax" => syntax = Some(value),
                    "short" => short = Some(value),
                    "docs" => docs = Some(value),
                    _ => return Err(Error::new_spanned(ident, "Unknown attribute")),
                }
            } else {
                return Err(Error::new_spanned(meta, "Expected name-value pair"));
            }
        }

        Ok(Self {
            name: name.ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing 'name' attribute"))?,
            syntax: syntax.ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing 'syntax' attribute"))?,
            short: short.ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing 'short' attribute"))?,
            docs: docs.ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing 'docs' attribute"))?,
        })
    }
}

/// Generates the Command trait implementation for the annotated function.
pub fn generate_command_impl(attr: TokenStream, input: ItemFn) -> Result<TokenStream> {
    let metadata = CommandMetadata::parse(attr)?;

    // Extract function information
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = &input.block;
    let fn_attrs = &input.attrs;
    let is_async = input.sig.asyncness.is_some();

    // Extract parameter information
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();

    for input_param in &input.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input_param {
            // Extract parameter name
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                param_names.push(pat_ident.ident.clone());
            } else {
                return Err(Error::new_spanned(pat_type, "Expected simple identifier pattern"));
            }

            // Extract parameter type
            param_types.push(&*pat_type.ty);
        } else {
            return Err(Error::new_spanned(input_param, "Expected typed parameter"));
        }
    }

    let param_count = param_names.len();

    // Generate the command struct name
    let command_struct_name = format_ident!("{}Command", capitalize_first(&fn_name.to_string()));

    // Generate argument parsing code
    let mut arg_parsing = Vec::new();
    for (i, (param_name, param_type)) in param_names.iter().zip(param_types.iter()).enumerate() {
        let type_name = quote!(#param_type).to_string();
        arg_parsing.push(quote! {
            let #param_name: #param_type = args[#i].parse()
                .map_err(|_| ::quark::CommandError::TypeConversionError {
                    arg: args[#i].clone(),
                    target_type: #type_name,
                })?;
        });
    }

    // Generate the function call
    let fn_call = if is_async {
        quote! { #fn_name(#(#param_names),*).await }
    } else {
        quote! { #fn_name(#(#param_names),*) }
    };

    // Metadata fields
    let cmd_name = &metadata.name;
    let cmd_syntax = &metadata.syntax;
    let cmd_short = &metadata.short;
    let cmd_docs = &metadata.docs;

    // Generate the implementation based on sync/async
    let command_impl = if is_async {
        quote! {
            impl ::quark::Command for #command_struct_name {
                fn name(&self) -> &str {
                    #cmd_name
                }

                fn syntax(&self) -> &str {
                    #cmd_syntax
                }

                fn short(&self) -> &str {
                    #cmd_short
                }

                fn docs(&self) -> &str {
                    #cmd_docs
                }

                fn is_async(&self) -> bool {
                    true
                }

                fn execute_async<'a>(
                    &'a self,
                    args: Vec<String>,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::quark::Result<()>> + Send + 'a>> {
                    Box::pin(async move {
                        if args.len() != #param_count {
                            return Err(::quark::CommandError::ArgumentCountMismatch {
                                expected: #param_count,
                                got: args.len(),
                            });
                        }

                        #(#arg_parsing)*

                        #fn_call;
                        Ok(())
                    })
                }
            }
        }
    } else {
        quote! {
            impl ::quark::Command for #command_struct_name {
                fn name(&self) -> &str {
                    #cmd_name
                }

                fn syntax(&self) -> &str {
                    #cmd_syntax
                }

                fn short(&self) -> &str {
                    #cmd_short
                }

                fn docs(&self) -> &str {
                    #cmd_docs
                }

                fn is_async(&self) -> bool {
                    false
                }

                fn execute(&self, args: Vec<String>) -> ::quark::Result<()> {
                    if args.len() != #param_count {
                        return Err(::quark::CommandError::ArgumentCountMismatch {
                            expected: #param_count,
                            got: args.len(),
                        });
                    }

                    #(#arg_parsing)*

                    #fn_call;
                    Ok(())
                }
            }
        }
    };

    // Generate the original function (possibly async)
    let fn_signature = if is_async {
        quote! { #fn_vis async fn #fn_name(#(#param_names: #param_types),*) }
    } else {
        quote! { #fn_vis fn #fn_name(#(#param_names: #param_types),*) }
    };

    // Generate the complete output
    Ok(quote! {
        // The original function
        #(#fn_attrs)*
        #fn_signature {
            #fn_block
        }

        // The command struct
        #[allow(non_camel_case_types)]
        #fn_vis struct #command_struct_name;

        // The Command trait implementation
        #command_impl
    })
}

/// Capitalizes the first character of a string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
