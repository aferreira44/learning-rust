class Product {}

class Config {
  constructor(debugMode) {
    this.debugMode = debugMode;
  }
}

class ProductService {
  constructor(config) {
    this._config = config;
  }

  getProduct(id) {
    if (this._config.debugMode) {
      console.log("retrieving product for id" + id);
    }

    return new Product();
  }
}

class BasketService {
  constructor(config) {
    this._config = config;
  }

  addProduct(product) {
    if (this._config.debugMode) {
      console.log("adding product %O", product);
    }
  }
}

let config = new Config(true);

let productService = new ProductService(config);
let basketService = new BasketService(config);

var product = productService.getProduct(1);
basketService.addProduct(product);
