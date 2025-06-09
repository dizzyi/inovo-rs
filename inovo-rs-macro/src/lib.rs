extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, ItemStruct, LitStr};

#[proc_macro_attribute]
pub fn inovo_msg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let s = parse_macro_input!(item as ItemStruct);

    let i = s.ident.clone();

    let a = parse_macro_input!(attr as LitStr);

    quote! {
        #s

        impl RosMessageType for #i {
            const ROS_TYPE_NAME: &'static str = #a;
        }
        // impl RosMessageType for InovoMessage<#i> {
        //     const ROS_TYPE_NAME: &'static str = #a;
        // }
    }
    .into()
}
