/* 
    Continued from 'struct_4.rs'....

    Learnings:
    - understand lifetimes
    - Rust’s memory management relies on a concept of lifetimes to track references. 
    Most of the time you won’t even notice it because Rust lets us omit most lifetime annotations. 
    But there are cases where Rust needs lifetime annotations such as when defining structs that hold references.
    - Basically what the 'a lifetime annotation says is that the ProductService can’t live longer than the reference to the Config that it contains. 
    - Rust doesn’t infer that constrain for structs by itself so it needs us to bring clarity. The same helds true for the BasketService as it also keeps a reference to the Config.
    - The 'a is really only a name that we get to choose, we could have picked 'config but short single letter names are mostly used among the Rust community.
    - We need to use the life time annotation in the impl blocks as well as those are written for the ProductService and BasketService which introduce those lifetimes. Please note that the 'a of the ProductService is independend of the 'a of the BasketService we could have picked different names for both.
    
    Source: https://blog.thoughtram.io/rust/2015/05/11/rusts-ownership-model-for-javascript-developers.html
*/

#[derive(Debug)]
struct Product;

struct Config {
    debug_mode: bool
}

struct ProductService<'a> {
    config: &'a Config
}

struct BasketService<'a> {
    config: &'a Config
}

impl<'a> ProductService<'a> {
    fn new (_config: &Config) -> ProductService {
        ProductService {
            config: _config
        }
    }

    fn get_product(&self, id: u16) -> Product {
        if self.config.debug_mode {
            println!("retrieving product for id: {}", id);
        }

        Product
    }
}

impl<'a> BasketService<'a> {
    fn new (_config: &Config) -> BasketService {
        BasketService {
            config: _config
        }
    }

    fn add_product(&self, item: Product) {
        if self.config.debug_mode {
            println!("adding product: {:?}", item);
        }
    }
}


pub fn run() {
    let item = Product{};
    let config = Config{ 
        debug_mode: true
    };


    let product_service = ProductService::new(&config);
    let basket_service = BasketService::new(&config);

    product_service.get_product(1);
    basket_service.add_product(item);

}
