struct Product;

struct Config {
    debug_mode: bool,
}

struct ProductService {
    config: Config,
}
struct BasketService {
    config: Config,
}

impl ProductService {
    fn new(config: Config) -> ProductService {
        ProductService { config: config }
    }
}

impl BasketService {
    fn new(config: Config) -> BasketService {
        BasketService { config: config }
    }
}

fn main() {
    let config = Config { debug_mode: true };
    let _product_service = ProductService::new(config);
    let _basket_service = BasketService::new(config);
}
