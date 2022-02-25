[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Documentation](https://docs.rs/enum-flags/badge.svg)](https://docs.rs/enum-flags)
[![Crates.io Version](https://img.shields.io/crates/v/enum-flags.svg)](https://crates.io/crates/enum-flags)
# EnumFlags

> EnumFlags is a [csharp](https://docs.microsoft.com/en-us/dotnet/api/system.flagsattribute?view=net-5.0) like enum flags implementation.


## Example

Since use the feature `arbitrary_enum_discriminant`, **nightly channel** is required.

```rust
#![feature(arbitrary_enum_discriminant)]
use enum_flags::enum_flags;

#[repr(u8)]  // default: #[repr(usize)]
#[enum_flags]
#[derive(Copy, Clone, PartialEq)] // can be omitted
enum Flags{
    None = 0,
    A = 1,
    B, // 2
    C = 4
}
fn main() {
    let e1: Flags = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;

    assert_eq!(e1 | e2, Flags::A | Flags::B | Flags::C); // union
    assert_eq!(e1 & e2, Flags::C); // intersection
    assert_eq!(e1 ^ e2, Flags::A | Flags::B); // toggle
    assert_eq!(e1 & (!Flags::C), Flags::A); // deletion
    assert_eq!(e1 - Flags::C, Flags::A); // deletion

    assert_eq!(format!("{:?}", e1).as_str(), "(Flags::A | Flags::C)");
    assert!(e1.has_a());
    assert!(!e1.has_b());
    assert!(e1.has_flag(Flags::C));
    assert!(e1.contains(Flags::C));
    assert_eq!(match Flags::A | Flags::C {
        Flags::None => "None",
        Flags::A => "A",
        Flags::B => "B",
        Flags::C => "C",
        Flags::__Composed__(v) if v == Flags::A | Flags::B => "A and B",
        Flags::__Composed__(v) if v == Flags::A | Flags::C => "A and C",
        _ => "Others"
    }, "A and C")
}
```



## Breaking Changes

- before version 0.18
  ```rust
  #[derive(Copy, EnumFlags, Clone, PartialEq)]
  enum Flags {
      None = 0,
      A = 1,
      B = 2,
      C = 4
  }
  ```
- version 0.30 later
  ```rust
  #[enum_flags] 
  #[derive(Copy, Clone, PartialEq)]
  enum Flags {
      None = 0,
      A = 1,
      B = 2,
      C = 4
  }
  ```