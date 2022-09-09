mod bindings;

use crate::bindings::levenshtein;
use std::ffi::CString;

pub fn levenshtein_safe(a: &str, b: &str) -> usize {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    let distance = unsafe { levenshtein(a.as_ptr(), b.as_ptr()) };
    distance
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
