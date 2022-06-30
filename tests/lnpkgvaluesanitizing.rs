use lnpkg;
use std::collections::HashMap;
#[test]
fn basic_with_escapes_2() {
    let mut target = "This works 102% sure: []=;".to_string();
    let target_clone = target.clone();

    // Sanitize and desanitize
    target = lnpkg::sanitizer::sanitize_string(target);
    target = lnpkg::sanitizer::desanitize_string(target);

    assert_eq!(target, target_clone);
}

#[test]
fn sanitizing_string_in_lnpkg() {
    let target = "This works 102% sure: []=;".to_string();
    let sanitize_target = lnpkg::sanitizer::sanitize_string(target.clone());
    assert_eq!(lnpkg::sanitizer::sanitize_string(target), sanitize_target);
}

