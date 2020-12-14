use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ExprBlock, Ident, LitInt, Token,
};

fn create_day_ident(day: usize, capitalize: bool) -> Ident {
    format_ident!(
        "{}ay{}",
        if capitalize { 'D' } else { 'd' },
        format!("{:02}", day)
    )
}

struct DayRange {
    start: LitInt,
    end: LitInt,
    function_ident: Ident,
}

impl Parse for DayRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start = input.parse()?;
        input.parse::<Token![,]>()?;
        let end = input.parse()?;
        input.parse::<Token![,]>()?;
        let function_ident = input.parse()?;

        Ok(Self {
            start,
            end,
            function_ident,
        })
    }
}

#[proc_macro]
pub fn build_days(input: TokenStream) -> TokenStream {
    let DayRange {
        start,
        end,
        function_ident,
    } = parse_macro_input!(input as DayRange);

    let start_n: usize = start.base10_parse().unwrap();
    let end_n: usize = end.base10_parse().unwrap();
    let struct_idents = (start_n..=end_n)
        .map(|i| create_day_ident(i, true))
        .collect::<Vec<Ident>>();

    let imports = (start_n..=end_n).map(|i| {
        let crate_ident = create_day_ident(i, false);
        let struct_ident = &struct_idents[i - start_n];

        quote! {
            mod #crate_ident;
            use #crate_ident::#struct_ident;
        }
    });

    let match_cases = (start_n..=end_n).map(|i| {
        let struct_ident = &struct_idents[i - start_n];

        quote! {
            #i => #struct_ident::#function_ident,
        }
    });

    let q = quote! {
        #(#imports)*

        pub fn get_runner<R: ::std::io::BufRead>(day: usize) -> fn(R) {
            match day {
                #(#match_cases)*
                _ => todo!("This day is not implemented yet"),
            }
        }
    };

    q.into()
}

struct DayImpl {
    day: LitInt,
    buf_read_ident_mut: Option<syn::token::Mut>,
    buf_read_ident: Ident,
    impl_block: ExprBlock,
}

impl Parse for DayImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let day = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<Token![|]>()?;
        let buf_read_ident_mut = input.parse::<Token![mut]>().ok();
        let buf_read_ident = input.parse()?;
        input.parse::<Token![|]>()?;
        let impl_block = input.parse()?;

        Ok(Self {
            day,
            buf_read_ident_mut,
            buf_read_ident,
            impl_block,
        })
    }
}

#[proc_macro]
pub fn impl_day(input: TokenStream) -> TokenStream {
    let DayImpl {
        day,
        buf_read_ident_mut,
        buf_read_ident,
        impl_block,
    } = parse_macro_input!(input as DayImpl);
    let struct_ident = create_day_ident(day.base10_parse::<usize>().unwrap(), true);

    let q = quote! {
        pub struct #struct_ident;

        impl ::aoc_common::AocDay for #struct_ident {
            fn run<R: ::std::io::BufRead>(#buf_read_ident_mut #buf_read_ident: R) #impl_block
        }
    };

    q.into()
}
