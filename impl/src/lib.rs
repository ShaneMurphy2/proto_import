use easy_ext::ext;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

#[ext(InModules)]
pub(crate) impl TokenStream {
    fn wrapped_in_modules(self, module_names: &[Ident]) -> TokenStream {
        match &module_names {
            [] => self,
            [ident, rest @ ..] => {
                let inner = self.wrapped_in_modules(rest);
                quote! {
                    pub mod #ident {
                        #inner
                    }
                }
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn import(input: TokenStream) -> TokenStream {
    let path: Path = match syn::parse2(input) {
        Ok(input) => input,
        Err(err)  => return err.to_compile_error(),
    };

    let num_segments = path.segments.len();
    let mut module_path = Vec::<Ident>::with_capacity(num_segments);
    let mut package_path = Vec::<String>::with_capacity(num_segments);

    for segment in &path.segments {
        module_path.push(segment.ident.clone());
        package_path.push(segment.ident.to_string());
    }

    let package_path = package_path.join(".");
    let import = quote! {
        include!(concat!(env!("OUT_DIR"), concat!("/", #package_path, ".rs")));
    };

    import.wrapped_in_modules(&module_path)
}
