extern crate enum_map;
#[macro_use]
extern crate reexport_proc_macro;
reexport_proc_macro!(enum_map_derive);

#[derive(EnumMap)]
enum Hello {
    A,
    B,
    C,
}

#[test]
fn test_if_an_item_is_reexported() {
    assert_eq!(enum_map::index_for_key(Hello::B), 1);
}
