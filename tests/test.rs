use lnpkg::{LnPkg, LnPkgValue as lpv};
use std::collections::HashMap;
#[test]
fn test_lnpkgvalues() {
    let sample = vec![
        "".to_string(),       // Null
        "false".to_string(),  // Bool(false)
        "true".to_string(),   // Bool(true)
        "hello!".to_string(), // String("hello!")
        "123".to_string(),    // Int(123)
    ];
    let target = vec![
        lpv::Null,
        lpv::Bool(false),
        lpv::Bool(true),
        lpv::String(String::from("hello!")),
        lpv::Int(123),
    ];

    for i in 0..sample.len() {
        assert_eq!(lpv::from_string(sample[i].clone()), target[i])
    }
}

#[test]
fn test_lnpkg_parsing() {
    let sample = String::from("key:key2:key3");
    let mut target = HashMap::new();
    target.insert(String::from("key"), lpv::Null);
    target.insert(String::from("key2"), lpv::Null);
    target.insert(String::from("key3"), lpv::Null);
    assert_eq!(
        LnPkg::from_string(&sample),
        LnPkg::from_hashmap(target)
    );

    let sample = String::from("key=This is a string:key2=false:key3=32");
    let mut target = HashMap::new();
    target.insert(
        String::from("key"),
        lpv::String(String::from("This is a string")),
    );
    target.insert(String::from("key2"), lpv::Bool(false));
    target.insert(String::from("key3"), lpv::Int(32));
    assert_eq!(
        LnPkg::from_string(&sample),
        LnPkg::from_hashmap(target)
    );

    let sample = String::from("key=This is a string:key2=false:key3=32::null key");
    let mut target = HashMap::new();
    target.insert(
        String::from("key"),
        lpv::String(String::from("This is a string")),
    );
    target.insert(String::from("key2"), lpv::Bool(false));
    target.insert(String::from("key3"), lpv::Int(32));
    target.insert(String::from("null key"), lpv::Null);
    assert_eq!(
        LnPkg::from_string(&sample),
        LnPkg::from_hashmap(target)
    );
}
#[test]
fn string_testing() {
    let sample = "msg=Hello!:".to_string();
    let mut hm_output = HashMap::new();

    hm_output.insert("msg".to_string(), lpv::String("Hello!".to_string()));
    let output = LnPkg::from_hashmap(hm_output);

    assert_eq!(output.to_string(), sample);

    let sample = "msg=Hello!:".to_string();
    let mut hm_output = HashMap::new();

    hm_output.insert("msg".to_string(), lpv::String("Hello!".to_string()));
    let output = LnPkg::from_hashmap(hm_output);

    assert_eq!(output.to_string(), sample);
}

#[test]
fn test_from_string() {
    let sample = LnPkg::from_string(&"msg=Hello".to_string());
    let mut output_hm = HashMap::new();
    output_hm.insert("msg".to_string(), lpv::String("Hello".to_string()));

    assert_eq!(sample.content, output_hm);
}

