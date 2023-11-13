use itertools::{multiunzip, Itertools};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

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

    // All i128 columns need to check if the values are out of supported bounds.
    let encode_delta_fn_calls = delta_field_names.iter().zip(delta_field_types.iter().zip(delta_field_encoded_types.iter()))
    .map(| (field_name, (field_ty, encoded_field_ty))| {
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
                }
            }
            _ => panic!("Unsupported type"),
        }
    })
    .collect::<Vec<_>>();

    quote! {

        impl IntoCompressBits for #ident {
            fn into_bits(self, out: &mut tsz_compress::prelude::BitBuffer) {
                #( out.extend(#vlq_types::from(self.#delta_field_names).bits); )*
            }
        }

        impl IntoCompressBits for #delta_ident {
            fn into_bits(self, out: &mut tsz_compress::prelude::BitBuffer) {
                #(
                    #encode_delta_fn_calls
                )*
            }
        }

        impl Compress for #ident {
            type Full = #ident;
            type Delta = #delta_ident;

            fn into_full(self) -> Self::Full {
                self
            }

            fn into_delta(self, prev: &Self::Full) -> Self::Delta {
                self - *prev
            }

            fn into_deltadelta(self, prev_prev_row: &Self, prev_row: &Self) -> Self::Delta {
                (self - *prev_row) - (*prev_row - *prev_prev_row)
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

    let decode_delta_fn_calls = delta_field_names
        .iter()
        .zip(decode_delta_fns.iter())
        .enumerate()
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
            fn from_bits(input: &tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &tsz_compress::prelude::BitBufferSlice), &'static str> {
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
            fn from_bits(input: &tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &tsz_compress::prelude::BitBufferSlice), &'static str> {
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

            fn from_full<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                #ident::from_bits(bits).map_err(|_| "failed to unmarshal full row")
            }

            fn from_delta<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice, prev_row: &Self) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                let delta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal delta row")?;
                Ok((*prev_row + delta.0, delta.1))
            }

            fn from_deltadelta<'a>(bits: &'a tsz_compress::prelude::BitBufferSlice, prev_row: &Self, prev_prev_row: &Self) -> Result<(Self, &'a tsz_compress::prelude::BitBufferSlice), &'static str> {
                let deltadelta = #delta_ident::from_bits(bits).map_err(|_| "failed to unmarshal deltadelta row")?;
                Ok((*prev_row + (*prev_row - *prev_prev_row) + deltadelta.0, deltadelta.1))
            }
        }
    }
    .into()
}

fn get_fields_of_struct(input: syn::DeriveInput) -> Vec<(syn::Ident, syn::Type)> {
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Expected fields in derive(Builder) struct"),
    };
    let named_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("Expected named fields in derive(Builder) struct"),
    };
    named_fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>()
}

///
/// CompressV2 is a procedural macro that will inspect the fields of
/// a struct and generate a StructCompressor with statically sized columnar
/// compression for the fields.
///
#[proc_macro_derive(CompressV2)]
pub fn derive_compressv2(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);
    let ident = input.ident.clone();

    // We will define a struct by this name
    let compressor_ident = format_ident!("{}CompressorImpl", input.ident);

    // We will compress each of the fields as columns
    let columns = get_fields_of_struct(input);
    let (col_idents, col_tys): (Vec<_>, Vec<_>) = multiunzip(columns);
    let col_delta_comp_queue_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_compressor_queue", ident))
        .collect_vec();
    let col_delta_delta_comp_queue_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_delta_compressor_queue", ident))
        .collect_vec();
    let col_delta_buf_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_output_buffer", ident))
        .collect_vec();
    let col_delta_delta_buf_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_delta_output_buffer", ident))
        .collect_vec();
    let num_columns = col_idents.len();
    let col_values_emitted_delta = col_idents
        .iter()
        .map(|ident| format_ident!("{}_columns_values_emitted_delta_compression", ident))
        .collect_vec();
    let col_values_emitted_delta_delta = col_idents
        .iter()
        .map(|ident| format_ident!("{}_columns_values_emitted_delta_delta_compression", ident))
        .collect_vec();
    let upgraded_col_tys = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { i16 },
                    "i16" => quote! { i32 },
                    "i32" => quote! { i64 },
                    "i64" => quote! { i128 },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();
    let compressor_struct = quote! {
        struct #compressor_ident {
            #( #col_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue<#upgraded_col_tys, 10>,)*
            #( #col_delta_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue<#upgraded_col_tys, 10>,)*
            #( #col_delta_buf_idents: Option<::tsz_compress::prelude::BitBuffer>,)*
            #( #col_delta_delta_buf_idents: Option<::tsz_compress::prelude::BitBuffer>,)*
            #( #col_values_emitted_delta: usize,)*
            #( #col_values_emitted_delta_delta: usize,)*
            prev_prev_row: Option<#ident>,
            prev_row: Option<#ident>,
        }

        impl TszCompressV2 for #compressor_ident {
            type T = #ident;

            /// Sets up two compression queues: one for delta compression and one for delta-delta compression, along with their respective output buffers.
            /// Initializes counters for the number of column values emitted during the delta and delta-delta compression processes.
            fn new() -> Self {
                #compressor_ident {
                    #( #col_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue::<#upgraded_col_tys, 10>::new(),)*
                    #( #col_delta_delta_comp_queue_idents: ::tsz_compress::prelude::CompressionQueue::<#upgraded_col_tys, 10>::new(),)*
                    #( #col_delta_buf_idents: Some(::tsz_compress::prelude::BitBuffer::new()),)*
                    #( #col_delta_delta_buf_idents: Some(::tsz_compress::prelude::BitBuffer::new()),)*
                    #( #col_values_emitted_delta: 0,)*
                    #( #col_values_emitted_delta_delta: 0,)*
                    prev_prev_row: None,
                    prev_row: None,
                }
            }

            /// Performs compression using either delta or delta-delta compression, selecting the method that yields the smallest compressed values.
            fn compress(&mut self, row: Self::T) {
                let COMPRESSION_SIZE_FACTOR: usize = 5;

                let mut row: Option<Self::T> = Some(row);

                // Enqueues delta and delta-delta values

                let Some(row) = row.take() else{
                    return;
                };

                let Some(prev_row) = self.prev_row.take() else{
                    #(
                        self.#col_delta_comp_queue_idents.push(row.#col_idents as #upgraded_col_tys);
                        self.#col_delta_delta_comp_queue_idents.push(row.#col_idents as #upgraded_col_tys);

                    )*
                        self.prev_row = Some(row);
                    return;
                };

                let Some(prev_prev_row) = self.prev_prev_row.take() else{

                    #(
                        let delta = prev_row.#col_idents as #upgraded_col_tys - row.#col_idents as #upgraded_col_tys;
                        self.#col_delta_comp_queue_idents.push(delta);
                        self.#col_delta_delta_comp_queue_idents.push(delta);
                    )*;
                    self.prev_prev_row = Some(prev_row);
                    self.prev_row = Some(row);
                    return;
                };

                #(
                    self.#col_delta_comp_queue_idents.push(prev_row.#col_idents as #upgraded_col_tys - row.#col_idents as #upgraded_col_tys);

                    self.#col_delta_delta_comp_queue_idents.push(
                        (row.#col_idents as #upgraded_col_tys - prev_row.#col_idents as #upgraded_col_tys) - (prev_row.#col_idents as #upgraded_col_tys - prev_prev_row.#col_idents as #upgraded_col_tys)
                    );

                    self.prev_prev_row = Some(prev_row);
                    self.prev_row = Some(row);

                    // Emit when the queues reaches their maximum capacity
                    if self.#col_delta_comp_queue_idents.is_full() {
                        self.#col_delta_buf_idents.as_mut().map(|outbuf| self.#col_values_emitted_delta += self.#col_delta_comp_queue_idents.emit_delta_bits( outbuf, false));
                    }
                    if self.#col_delta_delta_comp_queue_idents.is_full() {
                        self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| self.#col_values_emitted_delta_delta += self.#col_delta_delta_comp_queue_idents.emit_delta_delta_bits( outbuf, false));}


                    // Chooses the compression algorithm associated with the output buffer that is N times smaller than the other output buffer.
                    if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                        if delta_buffer.len() > COMPRESSION_SIZE_FACTOR * delta_delta_buffer.len(){
                            self.#col_delta_buf_idents = None;
                            self.#col_values_emitted_delta = 0;
                        }
                        else if delta_delta_buffer.len() > COMPRESSION_SIZE_FACTOR * delta_buffer.len(){
                            self.#col_delta_delta_buf_idents = None;
                            self.#col_values_emitted_delta_delta = 0;
                        }
                    }
                )*
            }

            fn len(&self) -> usize {
                let mut finished_bit_count = 0;
                #(
                    if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                        finished_bit_count += delta_buffer.len().min(delta_delta_buffer.len());
                    }
                    else if let Some(delta_buffer) = &self.#col_delta_buf_idents{
                        finished_bit_count += delta_buffer.len()
                    }
                    else if let Some(delta_delta_buffer) = &self.#col_delta_delta_buf_idents{
                        finished_bit_count += delta_delta_buffer.len()
                    }

                )*
                let col_count_delta = (#( self.#col_delta_comp_queue_idents.len() )+*);
                let col_count_delta_delta = (#( self.#col_delta_delta_comp_queue_idents.len() )+*);
                let col_bit_rate = #num_columns * self.bit_rate();
                let pending_bit_count = col_count_delta.min(col_count_delta_delta) * col_bit_rate;
                finished_bit_count + pending_bit_count
            }

            fn bit_rate(&self) -> usize {
                let mut finished_bit_count = 0;
                let mut total_col_values_emitted = 0;
                #(
                    if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                        finished_bit_count += delta_buffer.len().min(delta_delta_buffer.len());
                    }
                    else if let Some(delta_buffer) = &self.#col_delta_buf_idents{
                            finished_bit_count += delta_buffer.len()
                        }
                    else if let Some(delta_delta_buffer) = &self.#col_delta_delta_buf_idents{
                        finished_bit_count += delta_delta_buffer.len()
                    }
                    // Increment total_col_values_emitted by the sum of values emitted for either delta or delta-delta compression per column. One of them will be 0 for each column.
                    total_col_values_emitted += (self.#col_values_emitted_delta + self.#col_values_emitted_delta_delta);
                )*
                finished_bit_count / total_col_values_emitted / #num_columns

            }

            fn finish(mut self) -> ::tsz_compress::prelude::BitBuffer {
                // Only use one encoding mechanism
                #(
                    if let (Some(delta_buffer), Some(delta_delta_buffer)) = (&self.#col_delta_buf_idents, &self.#col_delta_delta_buf_idents) {
                        if delta_buffer.len() > delta_delta_buffer.len() {
                            self.#col_delta_buf_idents = None;
                        } else {
                            self.#col_delta_delta_buf_idents = None;
                        }
                    }
                )*

                let mut final_capacity = 0;

                #(
                    self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                        while(self.#col_delta_comp_queue_idents.len() > 0){
                            self.#col_delta_comp_queue_idents.emit_delta_bits(outbuf, true);
                        }
                        final_capacity += 4 + outbuf.len();
                    });
                    self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                        while(self.#col_delta_delta_comp_queue_idents.len() > 0){
                            self.#col_delta_delta_comp_queue_idents.emit_delta_delta_bits(outbuf, true);
                        }
                        final_capacity += 4 + outbuf.len();
                    });
                )*

                // End on the byte alignment
                if final_capacity % 8 == 4 {
                    final_capacity += 4;
                }

                // All of the bits are concatenated with a 1001 tag indicating the start of a new column
                let mut output = ::tsz_compress::prelude::BitBuffer::with_capacity(final_capacity);

                // Extend bits to final output column by column
                #(
                    self.#col_delta_buf_idents.as_mut().map(|outbuf| {
                        output.push(true);
                        output.push(false);
                        output.push(false);
                        output.push(true);
                        output.extend(outbuf);
                    });
                    self.#col_delta_delta_buf_idents.as_mut().map(|outbuf| {
                        output.push(true);
                        output.push(false);
                        output.push(false);
                        output.push(true);
                        output.extend(outbuf);
                    });
                )*

                // Pad with 1011 as garbage for byte-alignment
                if output.len() % 8 == 4 {
                    output.push(true);
                    output.push(false);
                    output.push(true);
                    output.push(true);
                }
                output
            }
        }
    };

    compressor_struct.into()
}

#[proc_macro_derive(DecompressV2)]
pub fn derive_decompressv2(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);

    // We will define a struct by this name
    let decompressor_ident = format_ident!("{}DecompressorImpl", input.ident);

    let columns = get_fields_of_struct(input);
    let (col_idents, col_tys): (Vec<_>, Vec<_>) = multiunzip(columns);

    let col_vec_idents = col_idents
        .iter()
        .map(|ident| format_ident!("{}_col_vec", ident))
        .collect_vec();

    let decode_idents = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { decode_i8 },
                    "i16" => quote! { decode_i16 },
                    "i32" => quote! { decode_i32 },
                    "i64" => quote! { decode_i64 },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let values_from_delta_ident = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { values_from_delta_i8 },
                    "i16" => quote! { values_from_delta_i16 },
                    "i32" => quote! { values_from_delta_i32 },
                    "i64" => quote! { values_from_delta_i64 },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let values_from_delta_delta_ident = col_tys
        .iter()
        .map(|ty| match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segment = path.segments.first().unwrap();
                let ident = segment.ident.clone();
                match ident.to_string().as_str() {
                    "i8" => quote! { values_from_delta_delta_i8 },
                    "i16" => quote! { values_from_delta_delta_i16 },
                    "i32" => quote! { values_from_delta_delta_i32 },
                    "i64" => quote! { values_from_delta_delta_i64 },
                    _ => panic!("Unsupported type"),
                }
            }
            _ => panic!("Unsupported type"),
        })
        .collect::<Vec<_>>();

    let delta_ident = col_idents
        .iter()
        .map(|ident| format_ident!("{}_delta_ident", ident))
        .collect_vec();

    let decompressor_struct = quote! {
        struct #decompressor_ident {
            #( #col_vec_idents: Vec<#col_tys>,)*
            #( #delta_ident: bool,)*
            bits_length: usize,
            index: Option<usize>
        }

        impl TszDecompressV2 for #decompressor_ident {
            /// Performs compression using either delta or delta-delta compression, selecting the method that yields the smallest compressed values.
            fn new() -> Self {
                #decompressor_ident {
                    #( #col_vec_idents: Vec::new(),)*
                    #( #delta_ident: true,)*
                    bits_length: 0,
                    index: Some(0),
                }
            }

            fn decompress(
                &mut self,
                bits: & tsz_compress::prelude::BitBufferSlice){

                self.index = Some(0);
                self.bits_length = bits.len();


                #(
                    if let Some(index) = self.index{
                        (self.index, self.#delta_ident) = #decode_idents(& bits, index, &mut self.#col_vec_idents).unwrap();
                    }
                )*

                #(
                    if self.#delta_ident {
                        #values_from_delta_ident(&mut self.#col_vec_idents);
                    }
                    else{
                        #values_from_delta_delta_ident(&mut self.#col_vec_idents);
                    }
                )*

                if let Some(index) = self.index{
                    if (index < self.bits_length) && !(bits[index] && !(bits[index] && bits[index + 1] && !bits[index + 2]  && bits[index + 3])) {
                        panic!("Invalid bits.");
                    }
                }
            }
        }
    };
    decompressor_struct.into()
}
