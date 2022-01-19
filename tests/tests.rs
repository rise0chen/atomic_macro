use std::sync::atomic::Ordering;

#[test]
fn test_enum() {
    #[derive(Debug, Clone, PartialEq)]
    #[atomic_macro::atomic(8)]
    enum Enum {
        A,
        B,
    }
    impl From<u8> for Enum {
        fn from(value: u8) -> Self {
            if value == 0 {
                Self::A
            } else {
                Self::B
            }
        }
    }
    impl From<Enum> for u8 {
        fn from(value: Enum) -> Self {
            if value == Enum::A {
                0
            } else {
                1
            }
        }
    }
    let s = Enum::B;
    let a = AtomicEnum::new();
    a.store(s.clone(), Ordering::Relaxed);
    let b = a.load(Ordering::Relaxed);
    assert_eq!(s, b);
}

#[test]
fn test_struct() {
    #[derive(Debug, Clone, PartialEq)]
    #[atomic_macro::atomic(64)]
    struct Struct {
        field: u64,
    }
    impl From<u64> for Struct {
        fn from(value: u64) -> Self {
            Self { field: value }
        }
    }
    impl From<Struct> for u64 {
        fn from(value: Struct) -> Self {
            value.field
        }
    }
    let s = Struct { field: 11 };
    let a = AtomicStruct::new();
    a.store(s.clone(), Ordering::Relaxed);
    let b = a.load(Ordering::Relaxed);
    assert_eq!(s, b);
}
