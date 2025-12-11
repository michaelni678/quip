//! **Quip** adds expression interpolation to several quasi-quoting macros:
//!
//! - [`quote::quote!`] → [`quip!`]
//! - [`quote::quote_spanned!`] → [`quip_spanned!`]
//! - [`syn::parse_quote!`] → [`parse_quip!`]
//! - [`syn::parse_quote_spanned!`] → [`parse_quip_spanned!`]
//!
//! # Setup
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! quip = "0.2.0"
//! quote = "1"    # For `quip!` and `quip_spanned!`.
//! syn = "2"      # For `parse_quip!` and `parse_quip_spanned!`.
//! ```
//!
//! # Syntax
//!
//! All Quip macros use `#{...}` for expression interpolation, where `...` must
//! evaluate to a type implementing [`quote::ToTokens`]. All other aspects,
//! including repetition and hygiene, behave identically to the underlying
//! macro.
//!
//! ```
//! # {} /*
//! quip! {
//!     impl Clone for #{item.name} {
//!         fn clone(&self) -> Self {
//!             Self {
//!                 #(#{item.members}: self.#{item.members}.clone(),)*
//!             }
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! # Behind the Scenes
//!
//! Quip scans tokens and transforms each expression interpolation `#{...}` into
//! a variable interpolation `#...` by binding the expression to a temporary
//! variable. The macro then passes the transformed tokens to the underlying
//! quasi-quotation macro.
//!
//! ```
//! # {} /*
//! quip! {
//!     impl MyTrait for #{item.name} {}
//! }
//! # */
//! ```
//!
//! The code above expands to:
//!
//! ```
//! # {} /*
//! match (&item.name,) {
//!     (__interpolation0,) => {
//!         ::quote::quote! {
//!             impl MyTrait for #__interpolation0 {}
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! [`quip!`]: https://docs.rs/quip/latest/quip/macro.quip.html
//! [`quip_spanned!`]: https://docs.rs/quip/latest/quip/macro.quip_spanned.html
//! [`parse_quip!`]: https://docs.rs/quip/latest/quip/macro.parse_quip.html
//! [`parse_quip_spanned!`]: https://docs.rs/quip/latest/quip/macro.parse_quip_spanned.html
//!
//! [`quote::quote!`]: https://docs.rs/quote/latest/quote/macro.quote.html
//! [`quote::quote_spanned!`]: https://docs.rs/quote/latest/quote/macro.quote_spanned.html
//! [`syn::parse_quote!`]: https://docs.rs/syn/latest/syn/macro.parse_quote.html
//! [`syn::parse_quote_spanned!`]: https://docs.rs/syn/latest/syn/macro.parse_quote_spanned.html
//!
//! [`quote::ToTokens`]: https://docs.rs/quote/latest/quote/trait.ToTokens.html

use proc_macro::TokenStream;
use quote::quote;

mod core;

/// Adds expression interpolation to [`quote::quote!`].
///
/// # Setup
///
/// This macro requires the [`quote`] crate as an additional dependency. Add
/// this to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// quip = "0.2.0"
/// quote = "1"
/// ```
///
/// [`quote::quote!`]: https://docs.rs/quote/latest/quote/macro.quote.html
/// [`quote`]: https://crates.io/crates/quote
#[proc_macro]
pub fn quip(input: TokenStream) -> TokenStream {
    let path = quote!(::quote::quote!);
    core::expand(path, input.into()).into()
}

/// Adds expression interpolation to [`quote::quote_spanned!`].
///
/// # Setup
///
/// This macro requires the [`quote`] crate as an additional dependency. Add
/// this to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// quip = "0.2.0"
/// quote = "1"
/// ```
///
/// [`quote::quote_spanned!`]: https://docs.rs/quote/latest/quote/macro.quote_spanned.html
/// [`quote`]: https://crates.io/crates/quote
#[proc_macro]
pub fn quip_spanned(input: TokenStream) -> TokenStream {
    let path = quote!(::quote::quote_spanned!);
    core::expand(path, input.into()).into()
}

/// Adds expression interpolation to [`syn::parse_quote!`].
///
/// # Setup
///
/// This macro requires the [`syn`] crate as an additional dependency. Add this
/// to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// quip = "0.2.0"
/// syn = "2"
/// ```
///
/// [`syn::parse_quote!`]: https://docs.rs/syn/latest/syn/macro.parse_quote.html
/// [`syn`]: https://crates.io/crates/syn
#[proc_macro]
pub fn parse_quip(input: TokenStream) -> TokenStream {
    let path = quote!(::syn::parse_quote!);
    core::expand(path, input.into()).into()
}

/// Adds expression interpolation to [`syn::parse_quote_spanned!`].
///
/// # Setup
///
/// This macro requires the [`syn`] crate as an additional dependency. Add this
/// to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// quip = "0.2.0"
/// syn = "2"
/// ```
///
/// [`syn::parse_quote_spanned!`]: https://docs.rs/syn/latest/syn/macro.parse_quote_spanned.html
/// [`syn`]: https://crates.io/crates/syn
#[proc_macro]
pub fn parse_quip_spanned(input: TokenStream) -> TokenStream {
    let path = quote!(::syn::parse_quote_spanned!);
    core::expand(path, input.into()).into()
}
