extern crate proc_macro;

use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(BitSerialize)]
pub fn bit_serialize(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let name = input.ident;
    let type_params = input.generics.type_params();
    let type_params2 = input.generics.type_params();

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            if let Fields::Named(fields_named) = data_struct.fields {
                let field_names = fields_named.named.into_iter().map(|el| el.ident.unwrap());
                quote! {
                    impl<
                        W_derive_bit_serialize: std::io::Write,
                        E_derive_bit_serialize: bitstream_io::Endianness,
                        #(#type_params: BitSerialize<W_derive_bit_serialize, E_derive_bit_serialize>),*
                    > BitSerialize<W_derive_bit_serialize, E_derive_bit_serialize> for #name<#(#type_params2),*>
                    {
                    fn bit_serialize(self, bw: &mut bitstream_io::BitWriter<W_derive_bit_serialize, E_derive_bit_serialize>) -> Result<(), std::io::Error> {
                            #(
                                self.#field_names.bit_serialize(bw)?;
                            )*
                            Ok(())
                        }
                    }
                }
            } else {
                unimplemented!()
            }
        }
        Data::Enum(data_enum) => {
            let mut variant_names_unnamed = Vec::new();
            let mut variant_names_unit_discr = Vec::new();
            let mut variant_values_unit_discr = Vec::new();
            for variant in data_enum.variants {
                if let Fields::Unnamed(_) = variant.fields {
                    variant_names_unnamed.push(variant.ident);
                } else if let Fields::Unit = variant.fields {
                    if let Some((_, expr)) = variant.discriminant {
                        variant_names_unit_discr.push(variant.ident);
                        variant_values_unit_discr.push(expr);
                    }
                }
            }
            quote! {
                    impl<
                        W_derive_bit_serialize: std::io::Write,
                        E_derive_bit_serialize: bitstream_io::Endianness,
                        #(#type_params: BitSerialize<W_derive_bit_serialize, E_derive_bit_serialize>),*
                    > BitSerialize<W_derive_bit_serialize, E_derive_bit_serialize> for #name<#(#type_params2),*>
                    {
                    fn bit_serialize(self, bw: &mut bitstream_io::BitWriter<W_derive_bit_serialize, E_derive_bit_serialize>) -> Result<(), std::io::Error> {
                        match self {
                            #(
                                Self::#variant_names_unnamed(val) => val.bit_serialize(bw),
                            )*
                            #(
                                Self::#variant_names_unit_discr => (#variant_values_unit_discr).bit_serialize(bw),
                            )*
                            _ => unimplemented!()
                        }
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!(),
    };
    TokenStream::from(expanded)
}
