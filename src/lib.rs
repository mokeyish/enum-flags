#![allow(clippy::needless_doctest_main)]

//!
//! EnumFlags is a csharp like enum flags implementation.
//!
//! # Example
//! ```rust
//! #![feature(arbitrary_enum_discriminant)]
//! use enum_flags::enum_flags;
//!
//! #[repr(u8)]
//! #[enum_flags]
//! #[derive(Copy, Clone, PartialEq)]
//! enum Flags{
//!     None = 0,
//!     A = 1,
//!     B = 2,
//!     C = 4
//! }
//! fn main() {
//!     let e1: Flags = Flags::A | Flags::C;
//!     let e2 = Flags::B | Flags::C;
//!
//!     assert_eq!(e1 | e2, Flags::A | Flags::B | Flags::C); // union
//!     assert_eq!(e1 & e2, Flags::C); // intersection
//!     assert_eq!(e1 ^ e2, Flags::A | Flags::B); // toggle
//!     assert_eq!(e1 & (!Flags::C), Flags::A); // deletion
//!     assert_eq!(e1 - Flags::C, Flags::A); // deletion
//!
//!     assert_eq!(format!("{:?}", e1).as_str(), "(Flags::A | Flags::C)");
//!     assert!(e1.has_a());
//!     assert!(!e1.has_b());
//!     assert!(e1.has_flag(Flags::C));
//!     assert!(e1.contains(Flags::C));
//!     assert_eq!(match Flags::A | Flags::C {
//!         Flags::None => "None",
//!         Flags::A => "A",
//!         Flags::B => "B",
//!         Flags::C => "C",
//!         Flags::__Composed__(v) if v == Flags::A | Flags::B => "A and B",
//!         Flags::__Composed__(v) if v == Flags::A | Flags::C => "A and C",
//!         _ => "Others"
//!     }, "A and C")
//! }
//! ```

extern crate proc_macro;

use {
    syn::{DeriveInput, parse_macro_input},
    quote::*,
    proc_macro2::{self, Span},
    self::proc_macro::TokenStream
};
use syn::{Attribute, AttrStyle, Data, Path};



#[proc_macro_attribute]
pub fn enum_flags(_args: TokenStream, input: TokenStream) -> TokenStream {
    impl_flags(parse_macro_input!(input as DeriveInput))
}

fn impl_flags(mut ast: DeriveInput) -> TokenStream {

    let enum_name = &ast.ident;

    let num = if let Some(t) = extract_repr(&ast.attrs).unwrap() {
        t
    } else {
        ast.attrs.push(Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            path: Path::from(syn::Ident::new("repr", Span::call_site())),
            tokens: syn::parse2(quote! { (usize) }).unwrap(),
        });
        syn::Ident::new("usize", enum_name.span().clone())
    };


    let vis = &ast.vis;

    if let Data::Enum(ref mut data_enum) = &mut ast.data {
        data_enum.variants.push(syn::parse2(quote! {__Composed__(#num)}).unwrap());

    } else {
        panic!("`EnumFlags` has to be used with enums");
    }

    let result = match &ast.data {
        Data::Enum(ref data_enum) => {
            let enum_items = data_enum.variants.iter()
                .filter(|f| f.ident.to_string().ne("__Composed__"))
                .map(|v| & v.ident)
                .collect::<Vec<&syn::Ident>>();


            let has_enum_items = enum_items.iter()
                .map(|x| {
                    let mut n = to_snake_case(&x.to_string());
                    n.insert_str(0, "has_");
                    syn::Ident::new(n.as_str(), enum_name.span().clone())
                }).collect::<Vec<syn::Ident>>();
            let enum_names = enum_items.iter()
                .map(|x| {
                    let mut n = enum_name.to_string();
                    n.push_str("::");
                    n.push_str(&x.to_string());
                    n
                }).collect::<Vec<String>>();


            quote! {

                #ast

                impl #enum_name {
                    #(
                        #vis fn #has_enum_items(&self)-> bool {
                            use #enum_name::*;
                            self.contains(#enum_items)
                        }
                    )*
                    #vis fn has_flag(&self, flag: Self) -> bool {
                        self.contains(flag)
                    }
                    #vis fn is_empty(&self) -> bool {
                        #num::from(self) == 0
                    }
                    #vis fn is_all(&self) -> bool {
                        use #enum_name::*;
                        let mut v = Self::from(0);
                        #(
                            v |= #enum_items;
                        )*
                        *self == v
                    }
                    #vis fn contains(&self, flag: Self) -> bool {
                        let a: #num = self.into();
                        let b: #num = flag.into();
                        if a == 0 {
                            b == 0
                        } else {
                            (a & b) != 0
                        }
                    }

                    fn from_num(n: #num) -> Self {
                        n.into()
                    }

                    fn as_num(&self) -> #num {
                        self.into()
                    }
                }

                impl From<#num> for #enum_name {
                    fn from(n: #num) -> Self {
                        if n != 1 && n % 2 == 1 {
                            Self::__Composed__(n)
                        } else {
                            unsafe {
                                let bytes = std::slice::from_raw_parts((&n as *const #num) as *const u8, std::mem::size_of::<#num>());
                                std::ptr::read(bytes.as_ptr() as *const Self)
                            }
                        }
                    }
                }

                impl From<#enum_name> for #num {
                    #[inline]
                    fn from(s: #enum_name) -> Self {
                        use #enum_name::__Composed__;
                        match s {
                            __Composed__(n) => n,
                            _ => unsafe { *(&s as *const #enum_name as *const #num) }
                        }
                    }
                }

                impl From<&#enum_name> for #num {
                    fn from(s: &#enum_name) -> Self {
                        (*s).into()
                    }
                }

                impl std::ops::BitOr for #enum_name {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a | b;
                        Self::from(c)
                    }
                }

                impl std::ops::BitAnd for #enum_name {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a & b;
                        Self::from(c)
                    }
                }

                impl std::ops::BitXor for #enum_name {
                    type Output = Self;
                    fn bitxor(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a ^ b;
                        Self::from(c)
                    }
                }

                impl std::ops::Not for #enum_name {
                    type Output = Self;
                    fn not(self) -> Self::Output {
                        let a: #num = self.into();
                        Self::from(!a)
                    }
                }

                impl std::ops::Sub for #enum_name {
                    type Output = Self;

                    fn sub(self, rhs: Self) -> Self::Output {
                        self & (!rhs)
                    }
                }

                impl std::ops::BitOrAssign for #enum_name {
                    fn bitor_assign(&mut self, rhs: Self) {
                        *self = *self | rhs;
                    }
                }

                impl std::ops::BitAndAssign for #enum_name {
                    fn bitand_assign(&mut self, rhs: Self) {
                        *self = *self & rhs;
                    }
                }

                impl std::ops::BitXorAssign for #enum_name {
                    fn bitxor_assign(&mut self, rhs: Self) {
                        *self = *self ^ rhs;
                    }
                }

                impl std::ops::SubAssign for #enum_name {
                    fn sub_assign(&mut self, rhs: Self) {
                        *self = *self - rhs
                    }
                }

                impl std::fmt::Debug for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut v = Vec::new();
                        #(
                            if self.#has_enum_items() {
                                v.push(#enum_names)
                            }
                        )*
                        write!(f, "({})", v.join(" | "))
                    }
                }

                impl std::cmp::PartialEq<#num> for #enum_name {
                    fn eq(&self, other: &#num) -> bool {
                        #num::from(self) == *other
                    }
                }

                impl std::cmp::PartialEq<#enum_name> for #num {
                    fn eq(&self, other: &#enum_name) -> bool {
                        *self == #num::from(other)
                    }
                }

            }
        }
        _ => panic!("`EnumFlags` has to be used with enums")
    };

    result.into()
}


fn extract_repr(attrs: &[syn::Attribute]) -> Result<Option<syn::Ident>, syn::Error> {
    use syn::{Meta, NestedMeta};
    attrs
        .iter()
        .find_map(|attr| match attr.parse_meta() {
            Err(why) => Some(Err(syn::Error::new_spanned(
                attr,
                format!("Couldn't parse attribute: {}", why),
            ))),
            Ok(Meta::List(ref meta)) if meta.path.is_ident("repr") => {
                meta.nested.iter().find_map(|mi| match mi {
                    NestedMeta::Meta(Meta::Path(path)) => path.get_ident().cloned().map(Ok),
                    _ => None,
                })
            }
            Ok(_) => None,
        })
        .transpose()
}

fn to_snake_case(str: &str) -> String {
    let mut s = String::with_capacity(str.len());
    for (i, char) in str.char_indices() {
        if char.is_uppercase() && char.is_ascii_alphabetic() {
            if i > 0 {
                s.push('_');
            }
            s.push(char.to_ascii_lowercase());
        } else {
            s.push(char)
        }
    }
    s
}
