# Struct

- There are different types of struct

  - normal struct: with parameters
  - unit struct: without parameters

- Structs can be inherited from other structs via declaring the 1 struct type as type of another struct's parameter.

```rs
struct Purchase {
  bill: f64,
  tax: f64,
  shop_name: String,
  gst_no: String,
}

struct House {
  address: String,
  area: f64,
  price: f64,
  purchase: Purchase,
}
```
