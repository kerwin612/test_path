# test_path
**Test-Path**

# Usage

**dependency**:
```bash
cargo add test_path
```

**import**
```rust
use test_path::is_valid;
```

**call**
```rust
assert_eq!(is_valid("1"), true);
assert_eq!(is_valid("C:/测试"), true);
assert_eq!(is_valid("C:/test"), true);
assert_eq!(is_valid("X:/x/y/z"), true);
assert_eq!(is_valid(r"C:/te|st"), false);
```
