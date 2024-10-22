#[derive(Debug, Clone, PartialEq)]
pub struct Store {pub products:Vec<(String, f32)>}

impl Store {
    pub fn new(products:Vec<(String, f32)>) -> Store {Store {products}}

    pub fn get_price(&self, name:&String) -> Option<f32> {
        self.products.iter().find_map(|(product, price)| {if product == name {Some(*price)} else {None}})
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {pub items:Vec<f32>, pub receipt:Vec<f32>}

impl Cart {
    pub fn new() -> Cart {Cart {items: Vec::new(), receipt: Vec::new()}}

    pub fn insert_item(&mut self, store:&Store, element:String) {
        if let Some(price) = store.get_price(&element) {self.items.push(price)}
        else {println!("Product \"{}\" not available.", element)}
    }
}
