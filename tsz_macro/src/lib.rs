#![allow(unused)]
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Token,
};


#[proc_macro_derive(DeltaEncodable)]
pub fn derive_delta_encodable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let delta_field_encoded_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i64, i128 -> i64

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        "i128" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();
 
    quote! {
        #[derive(Clone, Copy, Debug)]
        pub struct #delta_ident {
            #( #delta_field_names: #delta_field_types ),*
        }

        impl ::core::ops::Sub for #ident {
            type Output = #delta_ident;

            fn sub(self, rhs: Self) -> Self::Output {
                #delta_ident {
                    #( #delta_field_names: self.#delta_field_names as #delta_field_types - rhs.#delta_field_names as #delta_field_types),*
                }
            }
        }

        impl ::core::ops::Add<#delta_ident> for #ident {
            type Output = #ident;

            fn add(self, rhs: #delta_ident) -> Self::Output {
                #ident {
                    #( #delta_field_names: (self.#delta_field_names as #delta_field_types + rhs.#delta_field_names) as #field_types),*
                }
            }
        }

        impl ::core::ops::Sub for #delta_ident {
            type Output = #delta_ident;

            fn sub(self, rhs: Self) -> Self::Output {
                #delta_ident {
                    #( #delta_field_names: self.#delta_field_names - rhs.#delta_field_names),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(Compressible)]
pub fn derive_compressible(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let delta_field_encoded_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i64, i128 -> i64

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        "i128" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();
    let vlq_types = fields
        .iter()
        .map(|(_, ty)| {
            // Signed values will use tsz_compress::compress::Svlq, unsigned values will use tsz_compress::compress::Uvlq.

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { tsz_compress::svlq::Svlq },
                        "i16" => quote! { tsz_compress::svlq::Svlq },
                        "i32" => quote! { tsz_compress::svlq::Svlq },
                        "i64" => quote! { tsz_compress::svlq::Svlq },
                        "i128" => quote! { tsz_compress::svlq::Svlq },
                        "u8" => quote! { tsz_compress::uvlq ::Uvlq },
                        "u16" => quote! { tsz_compress::uvlq::Uvlq },
                        "u32" => quote! { tsz_compress::uvlq::Uvlq },
                        "u64" => quote! { tsz_compress::uvlq::Uvlq },
                        "u128" => quote! { tsz_compress::uvlq::Uvlq },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let vlq_ref_types = fields
        .iter()
        .map(|(_, ty)| {
            // Signed values will use tsz_compress::compress::Svlq, unsigned values will use tsz_compress::compress::Uvlq.

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { tsz_compress::svlq::SvlqRef },
                        "i16" => quote! { tsz_compress::svlq::SvlqRef },
                        "i32" => quote! { tsz_compress::svlq::SvlqRef },
                        "i64" => quote! { tsz_compress::svlq::SvlqRef },
                        "i128" => quote! { tsz_compress::svlq::SvlqRef },
                        "u8" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u16" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u32" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u64" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u128" => quote! { tsz_compress::uvlq::UvlqRef },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    // All i128 columns need to check if the values are out of supported bounds.
    let encode_delta_fn_calls = delta_field_names.iter().zip(delta_field_types.iter().zip(delta_field_encoded_types.iter())).enumerate()
    .map(|(idx, (field_name, (field_ty, encoded_field_ty)))| {
        // if the field_ty is i128, then encoded_field_ty will be i64
        // check if the field is in bounds of i64::MIN and i64::MAX for those fields

        let encode_fn_name = format_ident!("encode_delta_{}", encoded_field_ty.to_string().to_lowercase());
        let field_ty_name =syn::parse2::<syn::Type>(field_ty.clone()).unwrap(); 
        match field_ty_name {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i128" => {
                        quote! {
                            if self.#field_name < i64::MIN as i128 || self.#field_name > i64::MAX as i128 {
                                unimplemented!();
                            }
                            tsz_compress::delta::#encode_fn_name(self.#field_name as i64, out);
                        }
                    },
                    _ => {
                        quote! {
                            tsz_compress::delta::#encode_fn_name(self.#field_name, out);
                        }
                    }                    
                    _ => panic!("Unsupported type to delta encode/decode"),
                }
            }
            _ => panic!("Unsupported type"),
        }
    })
    .collect::<Vec<_>>();

    quote! {

        impl IntoCompressBits for #ident {
            fn into_bits(&self, out: &mut bv::BitVec) {
                #( out.extend(#vlq_types::from(self.#delta_field_names).bits); )*
            }
        }

        impl IntoCompressBits for #delta_ident {
            fn into_bits(&self, out: &mut bv::BitVec) {
                #(
                    #encode_delta_fn_calls
                )*
            }
        }

        impl Compress for #ident {
            type Full = #ident;
            type Delta = #delta_ident;

            fn into_full(&self) -> Self::Full {
                *self
            }

            fn into_delta(&self, prev: &Self::Full) -> Self::Delta {
                *self - *prev
            }

            fn into_deltadelta(&self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta {
                (*self - *prev_row) - (*prev_row - *prev_prev_row)
            }
        }
    }
    .into()
}

#[proc_macro_derive(Decompressible)]
pub fn derive_decompressible(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let ident = input.ident.clone();
    let delta_ident = format_ident!("{}Delta", input.ident);
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    let fields = named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();
    let delta_field_names = fields.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let delta_field_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i128, i128 -> i128
            // u8 -> i16, u16 -> i32, u32 -> i64, u64 -> i128, u128 -> i128

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i128 },
                        "i128" => quote! { i128 },
                        "u8" => quote! { i16 },
                        "u16" => quote! { i32 },
                        "u32" => quote! { i64 },
                        "u64" => quote! { i128 },
                        "u128" => quote! { i128 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    let delta_field_encoded_types = fields
        .iter()
        .map(|(_, ty)| {
            // Find the next highest type that can represent the delta.
            // If non primitive, then panic.
            // i8 -> i16, i16 -> i32, i32 -> i64, i64 -> i64, i128 -> i64

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { i16 },
                        "i16" => quote! { i32 },
                        "i32" => quote! { i64 },
                        "i64" => quote! { i64 },
                        "i128" => quote! { i64 },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|(_, ty)| ty).collect::<Vec<_>>();

    let vlq_ref_types = fields
        .iter()
        .map(|(_, ty)| {
            // Signed values will use tsz_compress::compress::Svlq, unsigned values will use tsz_compress::compress::Uvlq.

            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { tsz_compress::svlq::SvlqRef },
                        "i16" => quote! { tsz_compress::svlq::SvlqRef },
                        "i32" => quote! { tsz_compress::svlq::SvlqRef },
                        "i64" => quote! { tsz_compress::svlq::SvlqRef },
                        "i128" => quote! { tsz_compress::svlq::SvlqRef },
                        "u8" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u16" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u32" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u64" => quote! { tsz_compress::uvlq::UvlqRef },
                        "u128" => quote! { tsz_compress::uvlq::UvlqRef },
                        _ => panic!("Unsupported type"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

            // ty
        })
        .collect::<Vec<_>>();
    
    // functions to call for the typ like, decode_delta_i8, decode_delta_i16, etc.
    let decode_delta_fns = delta_field_encoded_types
        .iter()
        .map(|type_token_stream| {
            // parse the type token stream to the type
            let ty = syn::parse2::<syn::Type>(type_token_stream.clone()).unwrap();
            match ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = segment.ident.clone();
                    match ident.to_string().as_str() {
                        "i8" => quote! { decode_delta_i8 },
                        "i16" => quote! { decode_delta_i16 },
                        "i32" => quote! { decode_delta_i32 },
                        "i64" => quote! { decode_delta_i64 },
                        _ => panic!("Unsupported type to delta encode/decode"),
                    }
                }
                _ => panic!("Unsupported type"),
            }

        })
        .collect::<Vec<_>>();

    // All but the last call should include a check for early EOF.
    // #(
    //     let (#delta_field_names, input) = #decode_delta_fns(input)?;
    //     let Some(input) = input else {
    //         return Err("Early EOF");
    //     };
    // )*

    let decode_delta_fn_calls = delta_field_names.iter().zip(decode_delta_fns.iter()).enumerate()
        .map(|(idx, (field_name, fn_name))| {
            if idx != decode_delta_fns.len() - 1 {
                quote! {
                    let (#field_name, input) = tsz_compress::delta::#fn_name(input)?;
                    let Some(input) = input else {
                        return Err("Early EOF");
                    };
                }
            } else {
                quote! {
                    let (#field_name, input) = tsz_compress::delta::#fn_name(input)?;
                    let input = input.unwrap_or_default();
                }
            }
        })
        .collect::<Vec<_>>();


    quote! {
        impl FromCompressBits for #ident {
            fn from_bits(input: &bv::BitSlice) -> Result<(Self, &bv::BitSlice), &'static str> {
                #( 
                    let (#delta_field_names, read) = <(#field_types, usize)>::try_from(#vlq_ref_types(input))?;
                    let input = &input[read..];
                )*

                Ok((Self {
                    #( #delta_field_names, )*
                }, input))
            }
        }

        impl FromCompressBits for #delta_ident {
            fn from_bits(input: &bv::BitSlice) -> Result<(Self, &bv::BitSlice), &'static str> {
                #(
                    #decode_delta_fn_calls
                )*

                Ok((Self {
                    #( #delta_field_names: #delta_field_names as #delta_field_types, )*
                }, input))
            }
        }
        
        impl Decompress for #ident {
            type Full = #ident;
            type Delta = #delta_ident;

            fn from_full<'a>(bits: &'a bv::BitSlice) -> Result<(Self, &'a bv::BitSlice), &'static str> {
                #ident::from_bits(bits).map_err(|_| "failed to unmarshal full row")
            }

            fn from_delta<'a>(bits: &'a bv::BitSlice, prev_row: &Self) -> Result<(Self, &'a bv::BitSlice), &'static str> {
                let delta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal delta row")?;
                Ok((*prev_row + delta.0, delta.1))
            }

            fn from_deltadelta<'a>(bits: &'a bv::BitSlice, prev_row: &Self, prev_prev_row: &Self) -> Result<(Self, &'a bv::BitSlice), &'static str> {
                let deltadelta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal deltadelta row")?;
                Ok((*prev_row + (*prev_row - *prev_prev_row) + deltadelta.0, deltadelta.1))
            }
        }
    }
    .into()}
