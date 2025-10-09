// #![feature(proc_macro_span)]

extern crate proc_macro;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{self, parse_macro_input, ItemStruct, LitStr, Token, Type};

struct InovoMsgAttr {
    fields: Punctuated<LitStr, Token![,]>,
}

impl Parse for InovoMsgAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(InovoMsgAttr {
            fields: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn inovo_msg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let ident = item_struct.ident.clone();

    let inovo_msg_attr = parse_macro_input!(attr as InovoMsgAttr);

    let type_name = inovo_msg_attr
        .fields
        .get(1)
        .map(LitStr::value)
        .unwrap_or(ident.to_string());

    let ros_type_name = inovo_msg_attr.fields[0].value() + "/" + &type_name;

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
        #item_struct

        impl roslibrust::RosMessageType for #ident {
            const ROS_TYPE_NAME: &'static str = #ros_type_name;
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn inovo_srv(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let ident = item_struct.ident.clone();

    let response_type = parse_macro_input!(attr as Type);

    quote! {
        #item_struct

        impl roslibrust::RosServiceType for #ident {
            type Request = #ident;
            type Response = #response_type;
            const ROS_SERVICE_NAME: &'static str = "";
            const MD5SUM: &'static str = "";
        }
    }
    .into()
}
