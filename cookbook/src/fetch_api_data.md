---
title: Fetch API data
---

In order to fetch any JSON data, one needs to deserialize the JSON data of custom type into a struct. This can be done using the [`serde_json`](https://crates.io/crates/serde_json/) crate.

Initially, I am trying to fetch the response body from the API that looks like this:
![](../../img/api_response.png)

```rust
// get location input of valet via Location API
#[tokio::main]
async fn get_valet() -> Result<User> {
    let name = get_input(&format!("Valet 🚚💁 name please?"));

    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("VALET_LOCATION_URL").expect("URL var not found");

    let body = reqwest::get(url).await?.json::<_>().await?;

    println!("{:?}", body);
    Ok(User {
        name,
        lat: 0.0,
        lng: 0.0,
    })
}
```

---

Now, I need to create a struct - `ValetLocation` to deserialize the JSON data into it.

And then I have to add the `serde` crate in order to use `Deserialize` trait for the JSON data that I am going to fetch.

```toml
[dependencies]
# cargo add serde --features=derive
serde = { version = "1.0", features = ["derive"] }
```

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ValetLocation {
    id: u32,
    uid: String,
    city: String,
    street_name: String,
    street_address: String,
    secondary_address: String,
    building_number: String,
    mail_box: String,
    community: String,
    zip_code: String,
    zip: String,
    postcode: String,
    time_zone: String,
    street_suffix: String,
    city_suffix: String,
    city_prefix: String,
    state: String,
    state_abbr: String,
    country: String,
    country_code: String,
    latitude: f64,
    longitude: f64,
    full_address: String,
}

// get location input of valet via Location API
#[tokio::main]
async fn get_valet() -> Result<User> {
    let name = get_input(&format!("Valet 🚚💁 name please?"));

    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("VALET_LOCATION_URL").expect("URL var not found");

    let body = reqwest::get(url).await?.json::<ValetLocation>().await?;

    println!("{:?}", body);
    Ok(User {
        name,
        lat: 0.0,
        lng: 0.0,
    })
}
```

---

With the above code, we get the following console output:

**Output**:

```sh
ValetLocation { id: 4928, uid: "7fef49a8-809e-48d8-b155-c072a34e29ba", city: "Yundtside", street_name: "Stanton Fields", street_address: "55989 Effertz Brooks", secondary_address: "Suite 683", building_number: "77339", mail_box: "PO Box 81", community: "Park Acres", zip_code: "76601-7271", zip: "13600-8160", postcode: "37818", time_zone: "America/Godthab", street_suffix: "Rapid", city_suffix: "port", city_prefix: "West", state: "Colorado", state_abbr: "AL", country: "Sierra Leone", country_code: "HT", latitude: 16.96054548560228, longitude: -80.78326583644798, full_address: "9449 Towne Glen, Irishberg, CT 47471-6266" }
```

---

In order to further filter the data (`latitude`, `longitude`), I would be modifying the struct like this as I don't need all the data from the API response.

```rust
#[derive(Deserialize, Debug)]
struct ValetLocation {
    latitude: f64,
    longitude: f64,
}


// get location input of valet via Location API
#[tokio::main]
async fn get_valet() -> Result<User> {
    let name = get_input(&format!("Valet 🚚💁 name please?"));

    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("VALET_LOCATION_URL").expect("URL var not found");

    let body = reqwest::get(url).await?.json::<ValetLocation>().await?;

    println!("{:?}", body);
    Ok(User {
        name,
        lat: 0.0,
        lng: 0.0,
    })
}
```

**Console output**:

```sh
ValetLocation { latitude: -28.38022252300653, longitude: -30.736327485312614 }
```
