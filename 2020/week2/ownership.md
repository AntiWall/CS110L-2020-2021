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
No, an attempt was made to assign to a borrowed value. ref3 borrow the ownership of s.

Example 2:
```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```
No, s out of scope, cannot return borrow
should return s instead


Example 3:
```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```
No, s2 cannot take ownership of v[0] 

Should be 
```rust
let s2: &String = &v[0];
```