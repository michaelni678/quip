use quip::quip;

#[test]
fn requires_lifetime_extension() {
    quip! {
        #{[&String::new()][0]}
    };
}
