# space age 2

使用过程宏实现 imple trait

```rust
#[planet(rate=3600)]
struct Earth;
```


```rust

#[proc_macro_attribute]
pub fn planet(attr: TokenStream, item: TokenStream) -> TokenStream {

    let ast = syn::parse(item).unwrap();
    impl_planet_macro(&ast, attr.to_string().parse::<f64>().unwrap_or(1.0).to_owned())
}
```
