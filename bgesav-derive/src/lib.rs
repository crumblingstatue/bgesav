use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(SavExt)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let Data::Struct(DataStruct{fields: Fields::Named(FieldsNamed{ref named, ..}), ..}) = data else {
        panic!("Expected struct with named fields")
    };

    let mut load_impl_part_1 = quote! {};
    let mut load_impl_part2 = quote! {};
    let mut save_impl = quote! {};
    for field in named {
        let name = &field.ident;
        let ty = &field.ty;
        load_impl_part_1.extend(quote! {
            let #name = #ty::read(&mut f)?;
        });
        load_impl_part2.extend(quote! {#name,});
        save_impl.extend(quote!({self.#name.write(&mut f)?;}));
    }
    let load_impl = quote! {
        #load_impl_part_1
        Ok(Self {#load_impl_part2})
    };
    let output = quote! {
        impl crate::SavExt for #ident {
            fn load_from_file(path: &Path) -> io::Result<Self> {
                let mut f = File::open(path)?;
                #load_impl
            }
            fn save_to_file(&self, path: &Path) -> io::Result<()> {
                let mut f = OpenOptions::new().write(true).open(path)?;
                #save_impl
                Ok(())
            }
        }
    };
    output.into()
}
