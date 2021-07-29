
use enum_flags::EnumFlags;


#[test]
fn test_bitor(){

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
    enum State {
        None= 0,
        A = 1,
        B = 2,
        C = 4
    }

    let s: State = State::A | State::B;

    assert_eq!("(State::A | State::B)", format!("{:?}", s).as_str())
}

#[test]
fn test_bitor_assign(){

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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

    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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
    #[derive(EnumFlags, Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!(e1 | e2, Flags::A | Flags::B | Flags::C) // union
}

#[test]
fn test_intersection(){
    #[repr(u8)]
    #[derive(EnumFlags, Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }

    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!(e1 & e2, Flags::C) // intersection
}

#[test]
fn test_xor(){
    #[repr(u8)]
    #[derive(EnumFlags, Copy, Clone, PartialEq)]
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
    #[derive(EnumFlags, Copy, Clone, PartialEq)]
    enum Flags{
        None = 0,
        A = 1,
        B = 2,
        C = 4
    }


    let e1 = Flags::A | Flags::C;
    assert_eq!(e1 & (!Flags::C), Flags::A);
    assert_eq!(e1 - Flags::C , Flags::A);
}