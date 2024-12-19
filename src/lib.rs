#![allow(clippy::needless_doctest_main)]

//!
//! EnumFlags is a csharp like enum flags implementation.
//!
//! The generated code is `no_std` compatible.
//!
//! # Example
//! ```rust
//! #![feature(arbitrary_enum_discriminant)]
//! use enum_flags::enum_flags;
//!
//! #[repr(u8)]  // default: #[repr(usize)]
//! #[enum_flags]
//! #[derive(Copy, Clone, PartialEq)]   // can be omitted
//! enum Flags{
//!     None = 0,
//!     A = 1,
//!     B, // 2
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

use syn::{AttrStyle, Attribute, Data, Expr, ExprLit, Ident, Lit, LitInt, Meta, NestedMeta, Path};
use {
    self::proc_macro::TokenStream,
    proc_macro2::{self, Span},
    quote::*,
    syn::{parse_macro_input, DeriveInput},
};

#[proc_macro_attribute]
pub fn enum_flags(_args: TokenStream, input: TokenStream) -> TokenStream {
    impl_flags(parse_macro_input!(input as DeriveInput))
}

fn impl_flags(mut ast: DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;

    let num = if let Some(repr) = extract_repr(&ast.attrs) {
        repr
    } else {
        ast.attrs.push(Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            path: Path::from(syn::Ident::new("repr", Span::call_site())),
            tokens: syn::parse2(quote! { (usize) }).unwrap(),
        });
        syn::Ident::new("usize", Span::call_site())
    };

    let vis = &ast.vis;

    if let Data::Enum(ref mut data_enum) = &mut ast.data {
        let mut i = 0;

        for variant in &mut data_enum.variants {
            if let Some((_, ref expr)) = variant.discriminant {
                i = if let Expr::Lit(ExprLit {
                    lit: Lit::Int(ref lit_int),
                    ..
                }) = expr
                {
                    lit_int
                        .to_string()
                        .parse::<u128>()
                        .expect("Invalid literal")
                        + 1
                } else {
                    panic!("Unsupported discriminant type, only integer are supported.")
                }
            } else {
                // println!("{}:{}", variant.ident, i);
                variant.discriminant = Some((
                    syn::token::Eq(Span::call_site()),
                    Expr::Lit(ExprLit {
                        lit: Lit::Int(LitInt::new(i.to_string().as_str(), Span::call_site())),
                        attrs: vec![],
                    }),
                ));
                i += 1;
            }
        }

        data_enum
            .variants
            .push(syn::parse2(quote! {__Composed__(#num)}).unwrap());
    } else {
        panic!("`EnumFlags` has to be used with enums");
    }



    // try to derive Copy,Clone,PartialEq automatically
    {
        let dervies = extract_derives(&ast.attrs);

        let dervies = ["Copy", "Clone", "PartialEq"]
            .iter()
            .filter(|x| dervies.iter().all(|d| d.ne(x)))
            .map(|x| Ident::new(x, Span::call_site()))
            .collect::<Vec<_>>();

        if dervies.len() > 0 {
            ast.attrs.push(Attribute {
                pound_token: Default::default(),
                style: AttrStyle::Outer,
                bracket_token: Default::default(),
                path: Path::from(syn::Ident::new("derive", Span::call_site())),
                tokens: syn::parse2(quote! { (#(#dervies),* )}).unwrap(),
            });
        }
    }

    let result = match &ast.data {
        Data::Enum(ref data_enum) => {
            let (enum_items, enum_values): (Vec<&syn::Ident>, Vec<&syn::Expr>) = data_enum
                .variants
                .iter()
                .filter(|f| f.ident.ne("__Composed__"))
                .map(|v| (&v.ident, &v.discriminant.as_ref().expect("").1))
                .unzip();

            let has_enum_items = enum_items
                .iter()
                .map(|x| {
                    let mut n = to_snake_case(&x.to_string());
                    n.insert_str(0, "has_");
                    Ident::new(n.as_str(), enum_name.span().clone())
                })
                .collect::<Vec<syn::Ident>>();

            let enum_names = enum_items
                .iter()
                .map(|x| {
                    let mut n = enum_name.to_string();
                    n.push_str("::");
                    n.push_str(&x.to_string());
                    n
                })
                .collect::<Vec<String>>();

            quote! {

                #ast

                impl #enum_name {
                    #(
                        #[inline]
                        #vis fn #has_enum_items(&self)-> bool {
                            self.contains(#enum_name::#enum_items)
                        }
                    )*

                    /// Returns `true` if all of the flags in `other` are contained within `self`.
                    #[inline]
                    #vis fn has_flag(&self, other: Self) -> bool {
                        self.contains(other)
                    }

                    /// Returns `true` if no flags are currently stored.
                    #[inline]
                    #vis fn is_empty(&self) -> bool {
                        #num::from(self) == 0
                    }

                    /// Returns `true` if all flags are currently set.
                    #[inline]
                    #vis fn is_all(&self) -> bool {
                        use #enum_name::*;
                        let mut v = Self::from(0);
                        #(
                            v |= #enum_items;
                        )*
                        *self == v
                    }

                    /// Returns `true` if all of the flags in `other` are contained within `self`.
                    #[inline]
                    #vis fn contains(&self, other: Self) -> bool {
                        let a: #num = self.into();
                        let b: #num = other.into();
                        if a == 0 {
                            b == 0
                        } else {
                            (a & b) != 0
                        }
                    }

                    #[inline]
                    #vis fn clear(&mut self) {
                        *self = Self::from(0);
                    }

                    /// Inserts the specified flags in-place.
                    #[inline]
                    #vis fn insert(&mut self, other: Self) {
                        *self |= other;
                    }

                    /// Removes the specified flags in-place.
                    #[inline]
                    #vis fn remove(&mut self, other: Self) {
                        *self &= !other;
                    }

                    /// Inserts or removes the specified flags depending on the passed value.
                    #[inline]
                    #vis fn set(&mut self, other: Self, value: bool) {
                        if value {
                            self.insert(other);
                        } else {
                            self.remove(other);
                        }
                    }

                    /// Toggles the specified flags in-place.
                    #[inline]
                    #vis fn toggle(&mut self, other: Self) {
                        *self ^= other;
                    }

                    /// Returns the intersection between the flags in `self` and
                    #[inline]
                    #vis fn intersection(&self, other: Self) -> Self {
                        *self & other
                    }

                    /// Returns the union of between the flags in `self` and `other`.
                    #[inline]
                    #vis fn union(&self, other: Self) -> Self {
                        *self | other
                    }

                    /// Returns the difference between the flags in `self` and `other`.
                    #[inline]
                    #vis fn difference(&self, other: Self) -> Self {
                        *self & !other
                    }

                    /// Returns the [symmetric difference][sym-diff] between the flags
                    /// in `self` and `other`.
                    #[inline]
                    #vis fn symmetric_difference(&self, other: Self) -> Self {
                        *self ^ other
                    }

                    #[inline]
                    #vis fn from_num(n: #num) -> Self {
                        n.into()
                    }

                    #[inline]
                    #vis fn as_num(&self) -> #num {
                        self.into()
                    }
                }

                impl From<#num> for #enum_name {
                    #[inline]
                    fn from(n: #num) -> Self {
                        use #enum_name::*;
                        match n {
                            #(
                                #enum_values => #enum_items,
                            )*
                            _ => __Composed__(n)
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
                    #[inline]
                    fn from(s: &#enum_name) -> Self {
                        (*s).into()
                    }
                }

                impl core::ops::BitOr for #enum_name {
                    type Output = Self;
                    #[inline]
                    fn bitor(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a | b;
                        Self::from(c)
                    }
                }

                impl core::ops::BitAnd for #enum_name {
                    type Output = Self;
                    #[inline]
                    fn bitand(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a & b;
                        Self::from(c)
                    }
                }

                impl core::ops::BitXor for #enum_name {
                    type Output = Self;
                    #[inline]
                    fn bitxor(self, rhs: Self) -> Self::Output {
                        let a: #num = self.into();
                        let b: #num = rhs.into();
                        let c = a ^ b;
                        Self::from(c)
                    }
                }

                impl core::ops::Not for #enum_name {
                    type Output = Self;

                    #[inline]
                    fn not(self) -> Self::Output {
                        let a: #num = self.into();
                        Self::from(!a)
                    }
                }

                impl core::ops::Sub for #enum_name {
                    type Output = Self;

                    #[inline]
                    fn sub(self, rhs: Self) -> Self::Output {
                        self & (!rhs)
                    }
                }

                impl core::ops::BitOrAssign for #enum_name {
                    #[inline]
                    fn bitor_assign(&mut self, rhs: Self) {
                        *self = *self | rhs;
                    }
                }

                impl core::ops::BitAndAssign for #enum_name {
                    #[inline]
                    fn bitand_assign(&mut self, rhs: Self) {
                        *self = *self & rhs;
                    }
                }

                impl core::ops::BitXorAssign for #enum_name {
                    #[inline]
                    fn bitxor_assign(&mut self, rhs: Self) {
                        *self = *self ^ rhs;
                    }
                }

                impl core::ops::SubAssign for #enum_name {
                    #[inline]
                    fn sub_assign(&mut self, rhs: Self) {
                        *self = *self - rhs
                    }
                }

                impl core::fmt::Debug for #enum_name {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        let mut first = true;
                        write!(f, "(")?;
                        #(
                            if self.#has_enum_items() {
                                if first {
                                    first = false;
                                }else {
                                    write!(f, " | ")?;
                                }
                                write!(f, "{}", #enum_names)?;
                            }
                        )*
                        write!(f, ")")
                    }
                }

                impl core::cmp::PartialEq<#num> for #enum_name {
                    #[inline]
                    fn eq(&self, other: &#num) -> bool {
                        #num::from(self) == *other
                    }
                }

                impl core::cmp::PartialEq<#enum_name> for #num {
                    #[inline]
                    fn eq(&self, other: &#enum_name) -> bool {
                        *self == #num::from(other)
                    }
                }

            }
        }
        _ => panic!("`EnumFlags` has to be used with enums"),
    };

    result.into()
}

fn extract_repr(attrs: &[Attribute]) -> Option<Ident> {
    attrs
        .iter()
        .find_map(|attr| match attr.parse_meta() {
            Err(why) => panic!("{:?}", syn::Error::new_spanned(
                attr,
                format!("Couldn't parse attribute: {}", why),
            )),
            Ok(Meta::List(ref meta)) if meta.path.is_ident("repr") => {
                meta.nested.iter().find_map(|mi| match mi {
                    NestedMeta::Meta(Meta::Path(path)) => path.get_ident().cloned(),
                    _ => None,
                })
            }
            Ok(_) => None,
        })
}

fn extract_derives(attrs: &[Attribute]) -> Vec<Ident> {
    attrs
        .iter()
        .flat_map(|attr| attr.parse_meta())
        .flat_map(|ref meta| match meta {
            Meta::List(ref meta) if meta.path.is_ident("derive") => {
                meta.nested.iter().filter_map(|mi| match mi {
                    NestedMeta::Meta(Meta::Path(path)) => path.get_ident().cloned(),
                    _ => None,
                })
                .collect::<Vec<_>>()
            }
            _ => Default::default(),
        })
        .collect::<Vec<_>>()
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
