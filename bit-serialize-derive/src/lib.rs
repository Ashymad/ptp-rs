extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(BitSerialize)]
pub fn bit_serialize(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            if let Fields::Named(fields_named) = data_struct.fields {
                let field_names = fields_named.named.into_iter().map(|el| el.ident.unwrap());
                quote! {
                    impl<W: std::io::Write, E: bitstream_io::Endianness> BitSerialize<W, E> for #name 
                    {
                        fn bit_serialize(self, bw: &mut bitstream_io::BitWriter<W, E>) -> Result<(), std::io::Error> {
                            #(
                                if let Err(err) = self.#field_names.bit_serialize(bw) {
                                    return Err(err)
                                }
                            )*
                            Ok(())
                        }
                    }
                }
            } else {
                unimplemented!()
            }
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!()
    };
    TokenStream::from(expanded)
}
