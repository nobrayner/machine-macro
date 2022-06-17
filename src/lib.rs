use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, bracketed, parse_macro_input};
use syn::{token, LitStr};

#[proc_macro]
pub fn machine(input: TokenStream) -> TokenStream {
    let json = parse_macro_input!(input as RootObject);

    eprintln!("{:#?}", json);

    TokenStream::new()
}

#[derive(Debug)]
struct RootObject {
    fields: Punctuated<Field, token::Comma>,
}
impl Parse for RootObject {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RootObject {
            fields: input.parse_terminated(Field::parse)?,
        })
    }
}

#[derive(Debug)]
enum Field {
    String(FieldData<LitStr>),
    Object(FieldData<Object>),
    Array(FieldData<Array>),
}
impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;
        let colon_token: token::Colon = input.parse()?;

        let la = input.lookahead1();
        if la.peek(LitStr) {
            Ok(Field::String(FieldData {
                name,
                colon_token,
                value: input.parse()?,
            }))
        } else if la.peek(token::Brace) {
            Ok(Field::Object(FieldData {
                name,
                colon_token,
                value: input.parse()?,
            }))
        } else if la.peek(token::Bracket) {
            Ok(Field::Array(FieldData {
                name,
                colon_token,
                value: input.parse()?,
            }))
        } else {
            Err(la.error())
        }
    }
}

#[derive(Debug)]
struct FieldData<T> {
    name: LitStr,
    colon_token: token::Colon,
    value: T,
}

#[derive(Debug)]
struct Object {
    brace_token: token::Brace,
    fields: Punctuated<Field, token::Comma>,
}
impl Parse for Object {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Object {
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse)?,
        })
    }
}

#[derive(Debug)]
struct Array {
    bracket_token: token::Bracket,
    values: Punctuated<Value, token::Comma>,
}
impl Parse for Array {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Array {
            bracket_token: bracketed!(content in input),
            values: content.parse_terminated(Value::parse)?,
        })
    }
}

#[derive(Debug)]
enum Value {
    String(LitStr),
    Object(Object),
}
impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let la = input.lookahead1();

        if la.peek(LitStr) {
            Ok(Value::String(input.parse()?))
        } else if la.peek(token::Brace) {
            Ok(Value::Object(input.parse()?))
        } else {
            Err(la.error())
        }
    }
}
