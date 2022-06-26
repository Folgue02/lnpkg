pub const SANITIZED: &[&str] = &["%0", "%1", "%2", "%3", "%4"];
pub const DESANITIZED: &[&str] = &[":", "=", "[", "]", ";"];
pub const ESCAPE_CHARACTER: &str = "\x7f";

/// Replaces characters that might break the `LnPkg` format
/// with safe characters from the `SANITIZED` slice.
pub fn sanitize_string(mut target: String) -> String {
    target = target.replace("%", ESCAPE_CHARACTER);
    for i in 0..DESANITIZED.len() {
        if target.contains(DESANITIZED[i]) {
            target = target.replace(DESANITIZED[i], SANITIZED[i]);
        }
    }
    target
}

/// Replaces the safe characters with the normal characters. This
/// action shouldn't be done on a string that's going to be inside of a `LnPkg`.
/// It uses the `DESANITIZED` slice.
pub fn desanitize_string(mut target: String) -> String {
    for i in 0..SANITIZED.len() {
        if target.contains(SANITIZED[i]) {
            target = target.replace(SANITIZED[i], DESANITIZED[i]);
        }
    }
    target = target.replace(ESCAPE_CHARACTER, "%");
    target 
}
