use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, Component};
use std::env::{temp_dir};

pub fn is_valid(path: &str) -> bool {
    let p = Path::new(path);
    let mut t = temp_dir();

    if p.has_root() {
        for c in p.components() {
            match c {
                Component::Prefix(_p) => {
                    continue;
                }
                Component::RootDir => {
                    continue;
                }
                _ => {
                    t = t.join(c);
                }
            }
        }
    } else {
        t = t.join(p);
    }

    match create_dir_all(&t) {
        Err(_e) => return false,
        Ok(()) => {
            match remove_dir_all(&t) {
                Err(_e) => return false,
                Ok(()) => return true,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("1"), true);
        assert_eq!(is_valid("C:/测试"), true);
        assert_eq!(is_valid("C:/test"), true);
        assert_eq!(is_valid("X:/x/y/z"), true);
        assert_eq!(is_valid(r"C:/te|st"), false);
    }
}
