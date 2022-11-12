/// A module for sanitizing strings and avoid them from breaking the format 
/// of the `LnPkg` that they are in
pub mod sanitizer;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct LnPkg {
    pub content: HashMap<String, LnPkgValue>,
}

impl LnPkg {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

    /// Parses a string and returns a `Self` instance.
    pub fn from_string(pkg: &str) -> Self {
        let mut hm: HashMap<String, LnPkgValue> = HashMap::new();

        for segment in pkg.split(':') {
            // Ignore empty segments
            if segment == "" {
                continue;
            }

            // Value=key
            if let Some(index) = segment.find('=') {
                let key = segment[..index].to_string();
                let value_str = segment[index + 1..].to_string();
                let value: LnPkgValue;

                // Difference between a normal key=value and a type segment
                value = LnPkgValue::from_string(value_str);
                hm.insert(key, value);
            } else {
                // Key, null value
                hm.insert(segment.to_string(), LnPkgValue::Null);
            }
        }
        Self {
            content: hm,
        }
    }

    /// Returns an instance of `LnPkg` built with a hashmap
    pub fn from_hashmap(target: HashMap<String, LnPkgValue>) -> Self {
        let target = target
            .into_iter()
            .map(|(k,v)| {
                if let LnPkgValue::String(s) = v {
                    return (k, LnPkgValue::String(sanitizer::sanitize_string(s)));
                }
                (k,v)
            })
            .collect();
        // TODO: Continue here
        Self {
            content: target,
        }
    }
    /// Returns the formatted version of the package to a string that can be parsed back
    /// into an identical `LnPkg`
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (k, v) in &self.content {
            result += format!("{}={}:", k, v).as_str()
        }
        result
    }

    /// Returns a vector of bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    /// Checks for the existance of `keys` inside of the LakeNetPackage, if all of them exist
    /// this function will return `true`, if at least one of them doesn't exist in the `self.content`
    /// hashmap, it will return `false`
    pub fn exist(&self, keys: &[&str]) -> bool {
        for k in keys {
            if !self.content.contains_key(*k) {
                return false;
            } else {
                continue;
            }
        }
        true
    }
}


#[derive(PartialEq, Debug, Clone)]
pub enum LnPkgValue {
    String(String),
    Int(i128),
    Bool(bool),
    List(Vec<String>),
    Null,
}


impl std::fmt::Display for LnPkgValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => format!("{}", b),
                Self::Int(i) => format!("{}", i),
                Self::String(s) => sanitizer::sanitize_string(s.clone()),
                Self::List(l) => "[".to_string() + &l.join(";") + &"]".to_string(),
                _ => "".to_string(),
            }
        )
    }
}

impl LnPkgValue {
    pub fn from_string(target: String) -> LnPkgValue {
        let result: LnPkgValue;
        if let Ok(int) = target.parse::<i128>() {
            result = LnPkgValue::Int(int)
        } else if let Ok(boolean) = target.parse::<bool>() {
            result = LnPkgValue::Bool(boolean)
        } else if target == "" {
            result = LnPkgValue::Null;
        } else if let Some(list) = LnPkgValue::from_string_to_list(&target) {
            result = list;
        } else {
            // String
            result = LnPkgValue::String(sanitizer::sanitize_string(target));
        }
        result
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn from_string_to_list(target: &str) -> Option<LnPkgValue> {
        if !(target.ends_with("]") && target.starts_with("[")) {
            return None;
        } else {
            let target = &target[1..target.len() - 1];
            let segments: Vec<String> = target.split(";").map(|x| x.to_string()).collect();

            if segments.len() == 1 && segments[0] == "" {
                // Avoid passing empty lists
                return Some(LnPkgValue::List(Vec::new()));
            } else {
                return Some(LnPkgValue::List(segments));
            }
        }
    }
}
