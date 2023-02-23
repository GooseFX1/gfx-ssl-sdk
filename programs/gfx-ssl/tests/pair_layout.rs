use gfx_ssl_interface::Pair;
use type_layout::TypeLayout;

#[test]
fn pair_layout() {
    println!("{:#?}", Pair::type_layout());
}