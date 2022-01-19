# atomic_macro

## Usage

```rust
#[atomic_macro::atomic(32)]
struct SomeStruct {
    field_one: u8,
    field_two: u16,
}
impl From<u32> for SomeStruct {
    fn from(value: u32) -> Self {
        ...
    }
}
impl From<SomeStruct> for u32 {
    fn from(value: SomeStruct) -> Self {
        ...
    }
}
```

## Generates

```rust
enum AtomicSomeStruct(AtomicU32);
impl AtomicSomeStruct {
    fn load(&self, order: Ordering) -> SomeStruct;
    fn store(&self, val: SomeStruct, order: Ordering);
    ...
}
```
