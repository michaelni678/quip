use met::{TokenStreamExt, tokens};
use quip::quip;

// This test verifies Quip expands to an identical token stream if no expression
// interpolations are found.
#[test]
fn no_expression_interpolations() {
    let output = quip! {
        impl Shape for Triangle {
            const SIDES: usize = 3;
        }
    };

    let expected = tokens! {
        impl Shape for Triangle {
            const SIDES: usize = 3;
        }
    };

    assert!(output.equals(&expected));
}

// This test verifies Quip works properly when there is only one expression
// interpolation.
#[test]
fn single_expression_interpolation() {
    let shape = tokens!(Triangle);

    let output = quip! {
        impl Shape for #{shape} {
            const SIDES: usize = 3;
        }
    };

    let expected = tokens! {
        impl Shape for Triangle {
            const SIDES: usize = 3;
        }
    };

    assert!(output.equals(&expected));
}

// This test verifies Quip works properly when there are multiple expression
// interpolations.
#[test]
fn multiple_expression_interpolations() {
    let shape = tokens!(Triangle);
    let sides = tokens!(3);

    let output = quip! {
        impl Shape for #{shape} {
            const SIDES: usize = #{sides};
        }
    };

    let expected = tokens! {
        impl Shape for Triangle {
            const SIDES: usize = 3;
        }
    };

    assert!(output.equals(&expected));
}

// This test verifies temporary values aren't dropped prior to being
// interpolated.
//
// The code below fails to compile.
//
// ```rust
// let output = {
//     let x = &[&String::new()][0];
//     quote!(#x)
// };
// ```
//
// Quip uses `match`, which naturally extends the lifetime of `x` to the end of
// the statement. The code below successfully compiles.
//
// ```rust
// let output = match (&[&String::new()][0],) {
//     (x,) => {
//          quote!(#x)
//      }
// };
// ```
#[test]
fn expression_interpolation_contains_temporary_value() {
    let output = quip! { #{[&String::new()][0]} };
    let expected = tokens!("");

    assert!(output.equals(&expected));
}
