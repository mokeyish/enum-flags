
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