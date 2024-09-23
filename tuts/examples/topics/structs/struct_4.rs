/* 
    Learnings:
    - ownership model
    - Borrow a struct variable inside another struct's function
    - The `&Config` as the parameter type means that it now borrows a reference instead of taking ownership

    Problem:
    - Instead of parsing the actual `config` inside the product_service & basket_service, the memory address is parsed.

    Source: https://blog.thoughtram.io/rust/2015/05/11/rusts-ownership-model-for-javascript-developers.html

    Continued in 'struct_5.rs'...
*/

struct Product;

struct Config {
    debug_mode: bool
}

struct ProductService;

struct BasketService;

impl ProductService {
    fn new (_config: &Config) -> ProductService {
        ProductService
    }
}

impl BasketService {
    fn new (_config: &Config) -> BasketService {
        BasketService
    }
}


pub fn run() {
    let config = Config{ 
        debug_mode: true
    };

    let product_service = ProductService::new(&config);
    let basket_service = BasketService::new(&config);

}
