use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{AttributeArgs, FnArg, ItemFn, Lit, Meta, NestedMeta, Pat, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut name_override = None;
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("name") {
                if let Lit::Str(litstr) = nv.lit {
                    name_override = Some(litstr.value());
                }
            }
        }
    }

    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident.clone();
    let handler_ident = format_ident!("__crosslic_handler_{}", fn_name);
    let cmd_name = name_override.unwrap_or_else(|| fn_name.to_string());

    let params: Vec<(Box<Pat>, Box<Type>)> = input_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some((pat_type.pat.clone(), pat_type.ty.clone()))
            } else {
                None
            }
        })
        .collect();

    let param_count = params.len();
    let param_names: Vec<_> = params.iter().map(|(pat, _)| pat).collect();

    let mut deserialization = quote! {};
    for (i, (pat, ty)) in params.iter().enumerate() {
        let index = syn::Index::from(i);
        deserialization.extend(quote! {
            let #pat: #ty = serde_json::from_value(args[#index].clone())
                .map_err(|e| format!("Argument {}: {}", #i, e))?;
        });
    }

    input_fn.sig.output = syn::parse_quote!(-> Result<impl serde::Serialize, String>);

    TokenStream::from(quote! {
        #input_fn

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #handler_ident: () = {
            fn handler(data: serde_json::Value) -> Result<serde_json::Value, String> {
                let args: Vec<serde_json::Value> = serde_json::from_value(data)
                    .map_err(|e| e.to_string())?;

                if args.len() != #param_count {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        #param_count,
                        args.len()
                    ));
                }

                #deserialization

                let result = #fn_name(#(#param_names),*)?;
                serde_json::to_value(result).map_err(|e| e.to_string())
            }

            crosslic::inventory::submit! {
                crosslic::CommandDescriptor {
                    name: #cmd_name,
                    handler
                }
            }
        };
    })
}
