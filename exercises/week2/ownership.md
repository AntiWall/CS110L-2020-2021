Example 1:

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```



Example 2:
s out of scope, cannot return borrow
should return s instead
```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```



Example 3:
s2 cannot take ownership of v[0]
can be 
let s2: &String = &v[0];
```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```
