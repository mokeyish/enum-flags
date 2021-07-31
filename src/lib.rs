//!
//! EnumFlags is a csharp like enum flags implementation.
//!
//! # Example
//! ```rust
//! use enum_flags::EnumFlags;
//!
//! #[repr(u8)]
//! #[derive(EnumFlags, Copy, Clone, PartialEq)]
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
//! }
//! ```


extern crate proc_macro;

use {
    syn::{DeriveInput, parse_macro_input},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream
};
use syn::Data;


#[proc_macro_derive(EnumFlags)]
pub fn enum_flags(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = &ast.ident;
    let num_size = extract_repr(&ast.attrs)
        .unwrap()
        .unwrap_or(syn::Ident::new("isize", enum_name.span().clone()));
    let vis = &ast.vis;

    let result = match &ast.data {
        Data::Enum(ref s) => {
            let tmp = s.variants.iter().map(|v| & v.ident).collect::<Vec<&syn::Ident>>();
            impl_flags(enum_name, tmp, &num_size, vis)
        }
        _ => panic!("doesn't work with unions yet")
    };
    result.into()
}



fn impl_flags(enum_name: &syn::Ident, enum_items: Vec<&syn::Ident>, num: &syn::Ident, vis: &syn::Visibility) -> proc_macro2::TokenStream {
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
        impl #enum_name {
            fn test(&self) -> String {
                "123".to_string()
            }
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
                *self as #num == 0
            }
            #vis fn is_all(&self) -> bool {
                use #enum_name::*;
                let mut v = Self::from_num(0);
                #(
                    v |= #enum_items;
                )*
                *self == v
            }
            #vis fn contains(&self, flag: Self) -> bool {
                let a = *self as #num;
                let b = flag as #num;
                if a == 0 {
                    b == 0
                } else {
                    (a & b) != 0
                }
            }
            fn from_num(n: #num) -> Self {
                unsafe {
                    let bytes = std::slice::from_raw_parts((&n as *const #num) as *const u8, std::mem::size_of::<#num>());
                    std::ptr::read(bytes.as_ptr() as *const Self)
                }
            }
        }

        impl std::ops::BitOr for #enum_name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                let a = self as #num;
                let b = rhs as #num;
                let c = a | b;
                Self::from_num(c)
            }
        }

        impl std::ops::BitAnd for #enum_name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                let a = self as #num;
                let b = rhs as #num;
                let c = a & b;
                Self::from_num(c)
            }
        }

        impl std::ops::BitXor for #enum_name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                let a = self as #num;
                let b = rhs as #num;
                let c = a ^ b;
                Self::from_num(c)
            }
        }

        impl std::ops::Not for #enum_name {
            type Output = Self;
            fn not(self) -> Self::Output {
                let a = self as #num;
                Self::from_num(!a)
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
    }
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