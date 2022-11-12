use lnpkg;
use std::collections::HashMap;
#[test]
fn test_sanitizing_hashmap_string_basic() {
    let target = "[][]".to_string();
    let target_clone = target.clone();
    let mut hm = HashMap::new();
    hm.insert("msg".to_string(), lnpkg::LnPkgValue::String(target));

    let pkg = lnpkg::LnPkg::from_hashmap(hm);

    assert_ne!(pkg.content["msg"], lnpkg::LnPkgValue::String(target_clone));

}
