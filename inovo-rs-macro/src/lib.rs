// #![feature(proc_macro_span)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{self, parse_macro_input, ItemStruct, LitStr, Token, Type};

struct InovoMsgAttr {
    args: Punctuated<LitStr, Token![,]>,
}

impl Parse for InovoMsgAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(InovoMsgAttr {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn inovo_msg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let ident = item_struct.ident.clone();

    let inovo_msg_attr = parse_macro_input!(attr as InovoMsgAttr);

    let type_name = inovo_msg_attr
        .args
        .get(1)
        .map(LitStr::value)
        .unwrap_or(ident.to_string());

    let ros_type_name = inovo_msg_attr.args[0].value() + "/" + &type_name;

    let serde_default = if item_struct.fields.is_empty() {
        quote! {}
    } else {
        quote! {
            #[serde(default)]
        }
    };

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
        #serde_default
        #item_struct

        impl roslibrust::RosMessageType for #ident {
            const ROS_TYPE_NAME: &'static str = #ros_type_name;
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn inovo_req(attr: TokenStream, item: TokenStream) -> TokenStream {
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

struct NameType {
    pub name: LitStr,
    pub msg_type: Type,
}

impl Parse for NameType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: Comma = input.parse()?;
        let msg_type = input.parse()?;
        Ok(NameType { name, msg_type })
    }
}

#[proc_macro_attribute]
pub fn inovo_topic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let ident = item_struct.ident.clone();

    let inovo_topic_attr = parse_macro_input!(attr as NameType);

    let topic_name = inovo_topic_attr.name;
    let topic_msg_type = inovo_topic_attr.msg_type;

    quote! {
        #item_struct

        impl crate::ros_bridge::Topic for #ident {
            const NAME: &'static str = #topic_name;
            type Message = #topic_msg_type;
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn inovo_service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let ident = item_struct.ident.clone();

    let inovo_topic_attr = parse_macro_input!(attr as NameType);

    let topic_name = inovo_topic_attr.name;
    let topic_msg_type = inovo_topic_attr.msg_type;

    quote! {
        #item_struct

        impl crate::ros_bridge::Service for #ident {
            const NAME: &'static str = #topic_name;
            type Req = #topic_msg_type;
        }

        unsafe impl Send for #ident {}
    }
    .into()
}
