pub struct GroceryItem {

    pub name: String,

    pub quantity: u8,

    pub aisle: Option<String>,

    pub price_per_unit: f32,

    total_price: f32,

}

impl GroceryItem {

    pub fn new(name: String, quantity: u8, aisle: Option<String>, price_per_unit: f32) -> Self {
        GroceryItem {
            name: name,
            quantity: quantity,
            aisle: aisle,
            price_per_unit: price_per_unit,
            total_price: price_per_unit * quantity as f32,
        }
    }

}