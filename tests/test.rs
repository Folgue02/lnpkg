use lnpkg::{LnPkg, LnPkgType, LnPkgValue as lpv};
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
        LnPkg::from_hashmap(target, LnPkgType::Unknown)
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
        LnPkg::from_hashmap(target, LnPkgType::Unknown)
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
        LnPkg::from_hashmap(target, LnPkgType::Unknown)
    );
}
#[test]
fn test_lnpkg_type() {
    let sample = "type=msg:msg=Hello starshine".to_string();
    let mut hm_output = HashMap::new();
    hm_output.insert(
        String::from("msg"),
        lpv::String("Hello starshine".to_string()),
    );
    let pkg_type = LnPkgType::Message;
    assert_eq!(LnPkg::from_string(&sample).content, hm_output);
    assert_eq!(LnPkg::from_string(&sample).pkg_type, pkg_type);

    let sample = "content=Im null".to_string();
    let mut hm_output = HashMap::new();
    hm_output.insert(String::from("content"), lpv::String("Im null".to_string()));
    let pkg_type = LnPkgType::Unknown;
    assert_eq!(LnPkg::from_string(&sample).content, hm_output);
    assert_eq!(LnPkg::from_string(&sample).pkg_type, pkg_type);

    let sample = "type=dontknow:content=Im null".to_string();
    let mut hm_output = HashMap::new();
    hm_output.insert(String::from("content"), lpv::String("Im null".to_string()));
    let pkg_type = LnPkgType::Unknown;
    assert_eq!(LnPkg::from_string(&sample).content, hm_output);
    assert_eq!(LnPkg::from_string(&sample).pkg_type, pkg_type);
}

#[test]
fn string_testing() {
    let sample = "type=msg:msg=Hello!:".to_string();
    let mut hm_output = HashMap::new();

    hm_output.insert("msg".to_string(), lpv::String("Hello!".to_string()));
    let output = LnPkg::from_hashmap(hm_output, LnPkgType::Message);

    assert_eq!(output.to_string(), sample);

    let sample = "type=msg:msg=Hello!:".to_string();
    let mut hm_output = HashMap::new();

    hm_output.insert("type".to_string(), lpv::String("msg".to_string()));
    hm_output.insert("msg".to_string(), lpv::String("Hello!".to_string()));
    let output = LnPkg::from_hashmap(hm_output, LnPkgType::Message);

    assert_eq!(output.to_string(), sample);
}

#[test]
fn test_from_string() {
    let sample = LnPkg::from_string(&"type=msg:msg=Hello".to_string());
    let mut output_hm = HashMap::new();
    output_hm.insert("msg".to_string(), lpv::String("Hello".to_string()));

    assert_eq!(sample.content, output_hm);
    assert_eq!(sample.pkg_type, LnPkgType::Message);
}

#[test]
#[ignore = "Cannot use for testing, sicne the order of the keys can change everytime it runs"]
fn test_to_string() {
    let mut hm = HashMap::new();
    hm.insert("client".to_string(), lpv::Int(23));
    hm.insert("msg".to_string(), lpv::String("Hey".to_string()));
    assert_eq!(
        LnPkg::from_hashmap(hm, LnPkgType::Message).to_string(),
        "type=msg:client=23:msg=Hey:"
    );
}

#[test]
fn command_with_no_arguments() {
    // TODO: Write tests for commands with no arguments
}

