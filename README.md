[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Documentation](https://docs.rs/enum-flags/badge.svg)](https://docs.rs/enum-flags)
[![Crates.io Version](https://img.shields.io/crates/v/enum-flags.svg)](https://crates.io/crates/enum-flags)
# EnumFlags

> EnumFlags is a [csharp](https://docs.microsoft.com/en-us/dotnet/api/system.flagsattribute?view=net-5.0) like enum flags implementation.

## Example

```rust
use enum_flags::EnumFlags;

#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, PartialEq)]
enum Flags{
    None = 0,
    A = 1,
    B = 2,
    C = 4
}
fn main() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    
    assert_eq!(e1 | e2, Flags::A | Flags::B | Flags::C); // union
    assert_eq!(e1 & e2, Flags::C); // intersection
    assert_eq!(e1 ^ e2, Flags::A | Flags::B); // xor
    assert_eq!(e1 & (!Flags::C), Flags::B); // deletion
    assert_eq!(e1 - Flags::C, Flags::B); // deletion

    assert_eq!("(Flags::A | Flags::C)", format!("{:?}", e1).as_str());
    assert!(e1.has_a());
    assert!(!e1.has_b());
    assert!(e1.has_flag(Flags::C));
    
}
```
