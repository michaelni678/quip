<h1 align="center">Quip</h1>
<h3 align="center">Quasi-quoting macros with expression interpolation</h3>
<div align="center">

[![docs.rs](https://img.shields.io/badge/docs.rs-quip-58a78a?style=for-the-badge&logo=Docs.rs)](https://docs.rs/quip)
&nbsp;&nbsp;&nbsp;
[![crates.io](https://img.shields.io/crates/v/quip?style=for-the-badge&logo=Rust)](https://crates.io/crates/quip)
&nbsp;&nbsp;&nbsp;
[![github](https://img.shields.io/badge/github-quip-gray?style=for-the-badge&logo=GitHub&color=669bbc)](https://github.com/michaelni678/quip)

</div>

**Quip** adds expression interpolation to several quasi-quoting macros:

- [`quote::quote!`] → [`quip!`]
- [`quote::quote_spanned!`] → [`quip_spanned!`]
- [`syn::parse_quote!`] → [`parse_quip!`]
- [`syn::parse_quote_spanned!`] → [`parse_quip_spanned!`]

# Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
quip = "0.2.0"
quote = "1"    # For `quip!` and `quip_spanned!`.
syn = "2"      # For `parse_quip!` and `parse_quip_spanned!`.
```

# Syntax

All Quip macros use `#{...}` for expression interpolation, where `...` must evaluate to a type implementing [`quote::ToTokens`]. All other aspects, including repetition and hygiene, behave identically to the underlying macro.

```rust
quip! {
    impl Clone for #{item.name} {
        fn clone(&self) -> Self {
            Self {
                #(#{item.members}: self.#{item.members}.clone(),)*
            }
        }
    }
}
```

# Behind the Scenes

Quip scans tokens and transforms each expression interpolation `#{...}` into a variable interpolation `#...` by binding the expression to a temporary variable. The macro then passes the transformed tokens to the underlying quasi-quotation macro.

```rust
quip! {
    impl MyTrait for #{item.name} {}
}
```

The code above expands to:

```rust
match (&item.name,) {
    (__interpolation0,) => {
        ::quote::quote! {
            impl MyTrait for #__interpolation0 {}
        }
    }
}
```

[`quip!`]: https://docs.rs/quip/latest/quip/macro.quip.html
[`quip_spanned!`]: https://docs.rs/quip/latest/quip/macro.quip_spanned.html
[`parse_quip!`]: https://docs.rs/quip/latest/quip/macro.parse_quip.html
[`parse_quip_spanned!`]: https://docs.rs/quip/latest/quip/macro.parse_quip_spanned.html

[`quote::quote!`]: https://docs.rs/quote/latest/quote/macro.quote.html
[`quote::quote_spanned!`]: https://docs.rs/quote/latest/quote/macro.quote_spanned.html
[`syn::parse_quote!`]: https://docs.rs/syn/latest/syn/macro.parse_quote.html
[`syn::parse_quote_spanned!`]: https://docs.rs/syn/latest/syn/macro.parse_quote_spanned.html

[`quote::ToTokens`]: https://docs.rs/quote/latest/quote/trait.ToTokens.html
