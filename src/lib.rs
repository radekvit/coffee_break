#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use syn::{parse::Parse, Error};

/// Take a break when compiling.
///
/// # Parameters
/// The parameter is one of the following:
/// - `1 second` or up to `1319 seconds`.
/// - `1 minute` or up to `251 minutes`.
///
/// # Example
/// ```
/// use coffee_break::coffee_break;
///
/// // Take a break while running `cargo build`
/// coffee_break!(60 seconds);
/// ```
///
#[proc_macro]
pub fn coffee_break(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as CoffeeBreak);

    let is_checking = if cfg!(feature = "check-friendly") {
        std::env::args_os().any(|arg| {
            let arg = arg.to_string_lossy();
            arg.starts_with("--emit=") && arg.contains("metadata") && !arg.contains("link")
        })
    } else {
        false
    };

    // We know if we're using `rust-analyzer` instead of `rustc` in the first argument.
    // However proc-macro checking needs at least one build of the macros to work so you'll
    // take a small break during the setup
    let is_rust_analyzer = if cfg!(feature = "ra-friendly") {
        std::env::args_os()
            .next()
            .unwrap()
            .to_string_lossy()
            .ends_with("rust-analyzer-proc-macro-srv")
    } else {
        false
    };

    if !(is_checking || is_rust_analyzer) {
        std::thread::sleep(std::time::Duration::from_secs(input.seconds));
    }
    
    TokenStream::new()
}

struct CoffeeBreak {
    seconds: u64,
}

impl Parse for CoffeeBreak {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let seconds = input.parse::<syn::LitInt>()?.base10_parse::<u16>()?;
        let ident = input.parse::<syn::Ident>()?;
        let seconds = parse_time(ident, seconds.into())?;
        Ok(CoffeeBreak { seconds })
    }
}

fn parse_time(ident: syn::Ident, time: u64) -> syn::Result<u64> {
    match ident.to_string().as_str() {
        "second" if time == 1 => Ok(time),
        "minute" if time == 1 => Ok(time * 60),
        "minutes" if time <= 251 => Ok(time * 60),
        "minutes" => 
                Err(Error::new(ident.span(), "We're not letting you take a break for longer than 4 hours and 11 minutes. That's a bit much.")),
        "seconds" if time <= 1319 => Ok(time),
        "seconds" => Err(Error::new(ident.span(),"If you want a longer break than 1319 seconds, give us a time in minutes please."))
          ,
        _ if time == 1 => Err(syn::Error::new(
            ident.span(),
            format!(
                "You need to take a break for `{time} second` or `{time} minute`, not `{time} {ident}`!"
            ),
        )),
        _  => Err(syn::Error::new(
            ident.span(),
            format!(
                "You need to take a break for `{time} seconds` or `{time} minutes`, not `{time} {ident}`!"
            ),
        )),
    }
}
