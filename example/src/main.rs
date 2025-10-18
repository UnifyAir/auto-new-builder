use auto_new_builder::auto_new_builder_derive::AutoNewBuilder;

fn main() {
    test_all_combinations();
}

fn test_all_combinations() {
    // Test 1: Named struct with mixed field types and attributes
    let test1 = TestStruct1::new(42, 100);
    assert_eq!(test1.required_field, 42);
    assert_eq!(test1.auto_value_field, 34);
    assert_eq!(test1.auto_string_field, "DEFAULT");
    assert_eq!(test1.option_field, None);
    assert_eq!(test1.required_option_field, Some(100));
    assert_eq!(test1.auto_option_field, Some(99));
    assert_eq!(test1.auto_option_string_field, Some("AUTO_OPTION".to_string()));

    // Test 1 with with_ methods
    let test1_modified = test1.with_option_field(200).with_auto_value_field(50);
    assert_eq!(test1_modified.option_field, Some(200));
    assert_eq!(test1_modified.auto_value_field, 50);

    // Test 2: Named struct with Option fields at the beginning (constraint removed)
    let test2 = TestStruct2::new(77);
    assert_eq!(test2.option_first, None);
    assert_eq!(test2.required_field, 77);
    assert_eq!(test2.option_middle, None);
    assert_eq!(test2.auto_value_field, 88);
    assert_eq!(test2.option_last, None);

    // Test 2 with with_ methods
    let test2_modified = test2.with_option_first(111).with_option_middle(222).with_option_last(333);
    assert_eq!(test2_modified.option_first, Some(111));
    assert_eq!(test2_modified.option_middle, Some(222));
    assert_eq!(test2_modified.option_last, Some(333));

    // Test 3: Named struct with all Option fields
    let test3 = TestStruct3::new();
    assert_eq!(test3.option1, None);
    assert_eq!(test3.option2, None);
    assert_eq!(test3.option3, None);

    let test3_modified = test3.with_option1(1).with_option2(2).with_option3(3);
    assert_eq!(test3_modified.option1, Some(1));
    assert_eq!(test3_modified.option2, Some(2));
    assert_eq!(test3_modified.option3, Some(3));

    // Test 4: Named struct with all required fields
    let test4 = TestStruct4::new(10, 20, 30);
    assert_eq!(test4.field1, 10);
    assert_eq!(test4.field2, 20);
    assert_eq!(test4.field3, 30);

    // Test 5: Named struct with all auto_value fields
    let test5 = TestStruct5::new();
    assert_eq!(test5.auto_int, 42);
    assert_eq!(test5.auto_string, "AUTO");
    assert_eq!(test5.auto_bool, true);
    assert_eq!(test5.auto_float, 3.14);

    // Test 6: Newtype with regular type (no with_inner method generated)
    let test6 = TestNewtype1::new(999);
    assert_eq!(test6.0, 999);

    // Test 7: Newtype with auto_value
    let test7 = TestNewtype2::new();
    assert_eq!(test7.0, 777);

    let test7_modified = test7.with_inner(666);
    assert_eq!(test7_modified.0, 666);

    // Test 8: Newtype with Option type
    let test8 = TestNewtype3::new();
    assert_eq!(test8.0, None);

    let test8_modified = test8.with_inner(555);
    assert_eq!(test8_modified.0, Some(555));

    // Test 9: Newtype with required Option type
    let test9 = TestNewtype4::new(444);
    assert_eq!(test9.0, Some(444));

    let test9_modified = test9.with_inner(333);
    assert_eq!(test9_modified.0, Some(333));

    // Test 10: Newtype with auto_value Option type
    let test10 = TestNewtype5::new();
    assert_eq!(test10.0, Some(222));

    let test10_modified = test10.with_inner(111);
    assert_eq!(test10_modified.0, Some(111));

    // Test 11: Complex mixed struct with function calls in auto_value
    let test11 = TestComplex::new(50);
    assert_eq!(test11.required_field, 50);
    assert_eq!(test11.auto_function_call, Some(get_value()));
    assert_eq!(test11.auto_option_with_function, Some(get_other_value()));
    assert_eq!(test11.regular_option, None);

    let test11_modified = test11.with_regular_option(999);
    assert_eq!(test11_modified.regular_option, Some(999));

    // Test 12: Struct with different Option syntaxes
    let test12 = TestOptionSyntax::new();
    assert_eq!(test12.std_option, None);
    assert_eq!(test12.short_option, None);
    assert_eq!(test12.auto_std_option, Some(123));

    let test12_modified = test12.with_std_option(456).with_short_option(789);
    assert_eq!(test12_modified.std_option, Some(456));
    assert_eq!(test12_modified.short_option, Some(789));
}

// Helper functions for auto_value tests
fn get_value() -> i32 { 42 }
fn get_other_value() -> i32 { 84 }

// Test structs covering all combinations

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestStruct1 {
    required_field: u32,
    #[auto_new_value="34"]
    auto_value_field: u32,
    #[auto_new_value="\"DEFAULT\".to_string()"]
    auto_string_field: String,
    option_field: Option<u32>,
    #[auto_new_required]
    required_option_field: Option<u32>,
    #[auto_new_value="Some(99)"]
    auto_option_field: Option<u32>,
    #[auto_new_value="Some(\"AUTO_OPTION\".to_string())"]
    auto_option_string_field: Option<String>,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestStruct2 {
    option_first: Option<u32>,
    #[auto_new_required]
    required_field: u32,
    option_middle: Option<u32>,
    #[auto_new_value="88"]
    auto_value_field: u32,
    option_last: Option<u32>,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestStruct3 {
    option1: Option<u32>,
    option2: Option<u32>,
    option3: Option<u32>,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestStruct4 {
    #[auto_new_required]
    field1: u32,
    #[auto_new_required]
    field2: u32,
    #[auto_new_required]
    field3: u32,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestStruct5 {
    #[auto_new_value="42"]
    auto_int: i32,
    #[auto_new_value="\"AUTO\".to_string()"]
    auto_string: String,
    #[auto_new_value="true"]
    auto_bool: bool,
    #[auto_new_value="3.14"]
    auto_float: f64,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestNewtype1(u32);

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestNewtype2(#[auto_new_value="777"] u32);

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestNewtype3(Option<u32>);

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestNewtype4(#[auto_new_required] Option<u32>);

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestNewtype5(#[auto_new_value="Some(222)"] Option<u32>);

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestComplex {
    #[auto_new_required]
    required_field: u32,
    #[auto_new_value="Some(get_value())"]
    auto_function_call: Option<i32>,
    #[auto_new_value="Some(get_other_value())"]
    auto_option_with_function: Option<i32>,
    regular_option: Option<u32>,
}

#[derive(AutoNewBuilder, Debug, PartialEq)]
pub struct TestOptionSyntax {
    std_option: ::std::option::Option<u32>,
    short_option: Option<u32>,
    #[auto_new_value="Some(123)"]
    auto_std_option: ::std::option::Option<u32>,
}