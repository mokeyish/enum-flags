#![feature(arbitrary_enum_discriminant)]

use enum_flags::enum_flags;


#[test]
fn test_bitor(){

    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let s: State = State::A | State::B;

    s.has_a();

    assert_eq!("(State::A | State::B)", format!("{:?}", s).as_str())
}

#[test]
fn test_bitor_assign(){

    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut s: State = State::A;

    s |= State::B;

    assert_eq!("(State::A | State::B)", format!("{:?}", s).as_str())
}

#[test]
fn test_has_flag(){

    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let s: State = State::A | State::B;

    assert!(s.has_flag(State::B));
    assert!(s.has_b());
    assert!(!s.has_flag(State::C));
    assert!(!s.has_flag(State::None));
}

#[test]
fn test_has_flag2(){


    #[enum_flags]
    #[derive(Clone, PartialEq)]
    #[derive(Copy)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let s: State = State::None;
    assert!(s.has_flag(State::None));
    assert!(s.has_none());
}

#[test]
fn test_bitand(){

    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let s: State = State::A | State::B;
    let s = s & State::C;
    assert_eq!(State::None, s);
}

#[test]
fn test_bitand_assign(){

    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut s: State = State::A | State::B;
    s &= State::C;
    assert_eq!(State::None, s);
}

#[test]
fn test_union(){
    #[repr(u8)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B.union(Flags::C);
    assert_eq!(e1 | e2, Flags::A | Flags::B | Flags::C) // union
}

#[test]
fn test_intersection(){
    #[repr(u8)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!(e1 & e2, Flags::C); // intersection
    assert_eq!(e1.intersection(e2), Flags::C) // intersection
}

#[test]
fn test_xor(){
    #[repr(u8)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!(e1 ^ e2, Flags::A | Flags::B);
}


#[test]
fn test_deletion(){
    #[repr(u8)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }


    let mut e1 = Flags::A | Flags::C;
    assert_eq!(e1 & (!Flags::C), Flags::A.as_num());
    assert_eq!(e1 - Flags::C , Flags::A);
    e1.remove(Flags::C);
    assert_eq!(e1, Flags::A);
}


#[test]
fn test_empty(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::None;
    assert!(e1.is_empty());
    assert!(!Flags::A.is_empty());
    assert!(!(Flags::A | Flags::B).is_empty());
}


#[test]
fn test_is_all(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::None;
    assert!(!e1.is_all());

    let e1 = Flags::A;
    assert!(!e1.is_all());

    let e1 = Flags::A | Flags::C;
    assert!(!e1.is_all());

    let e1 = Flags::A | Flags::C | Flags::B;
    assert!(e1.is_all());

    let e1 = Flags::A | Flags::C | Flags::B | Flags::None;
    assert!(e1.is_all());
}

#[test]
fn test_contains(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A;
    assert!(e1.contains(Flags::A));

    let e1 = Flags::A | Flags::C;
    assert!(e1.contains(Flags::A));

    let e1 = Flags::A | Flags::C;
    assert!(!e1.contains(Flags::None));



    let e1 = Flags::None;
    assert!(e1.contains(Flags::None));
}


#[test]
fn test_clear(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut e1 = Flags::A | Flags::C;
    e1.clear();
    assert_eq!(e1, Flags::None);
}

#[test]
fn test_insert(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut e1 = Flags::A;
    e1.insert(Flags::B);
    assert_eq!("(Flags::A | Flags::B)", format!("{:?}", e1));


    let mut e1 = Flags::None;
    e1.insert(Flags::B);
    assert_eq!("(Flags::B)", format!("{:?}", e1));
}


#[test]
fn test_remove(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut e1 = Flags::A;
    e1.insert(Flags::B);
    assert_eq!("(Flags::A | Flags::B)", format!("{:?}", e1));
    e1.remove(Flags::A);
    assert_eq!("(Flags::B)", format!("{:?}", e1));
}


#[test]
fn test_toggle(){
    #[repr(u32)]
    #[enum_flags]
    #[derive(Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let mut e1 = Flags::A;
    e1.insert(Flags::B);
    assert_eq!("(Flags::A | Flags::B)", format!("{:?}", e1));
    e1.toggle(Flags::B);
    assert_eq!("(Flags::A)", format!("{:?}", e1));
    e1.toggle(Flags::B);
    assert_eq!("(Flags::A | Flags::B)", format!("{:?}", e1));
}

#[test]
fn test_omit_derives(){
    #[enum_flags]
    enum Flags{
        None = 0,
        A,
        B = 2,
        C = 4
    }

    let mut e1 = Flags::A;
    e1.insert(Flags::B);
    assert_eq!("(Flags::A | Flags::B)", format!("{:?}", e1));
}