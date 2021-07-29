# EnumFlags

> EnumFlags is a [csharp](https://docs.microsoft.com/en-us/dotnet/api/system.flagsattribute?view=net-5.0) like enum flags implementation.

## Example

```rust
use enum_flags::EnumFlags;

#[repr(u8)] // if not specific,`u64` is the default
#[derive(EnumFlags, Copy, Clone, PartialEq)]
enum Test {
    None = 0,
    A = 1,
    B = 2, // unspecified variants pick unused bits automatically
    C = 4,
}

let a_b: Test = Test::A | Test::B;

assert_eq!("(Test::A | Test::B)", format!("{:?}", a_b).as_str());

assert!(a_b.has_a());
assert!(a_b.has_flag(Test::B));

```
