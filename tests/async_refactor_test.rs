use async_refactor::async_refactor;

#[test]
fn test_extract_async_function() {
    let code = "let x = 5;";
    let extracted = async_refactor::extract_async_function(code, "my_async_func");
    assert_eq!(extracted, "async fn my_async_func() {\nlet x = 5;\n}");
}

#[test]
fn test_convert_to_async() {
    use syn::parse_quote;

    let fn_item: syn::ItemFn = parse_quote! {
        fn my_sync_func() {
            println!("Hello, world!");
        }
    };
    let async_fn = async_refactor::convert_to_async(&fn_item);
    assert!(async_fn.sig.asyncness.is_some());
}
