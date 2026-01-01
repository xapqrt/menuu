//menuuu


use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


//menuitem am thinking will be representing one item on the menu at a time


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItem {

    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock:u32,
    pub category: String,

}




//impl block guna adds methods to MenuItem

impl MenuItem {

    pub fn new (

        id: u32,
        name: String,
        description: String,
        price: f64,
        stock:u32,
        category: String,

    ) -> MenuItem {

        MenuItem {
            id,
            name,
            description,
            price,
            stock,
            category,
        }
    }

    pub fn  is_available(&self) -> bool {

        self.stock > 0
    }



  


    //reduce stock for when items are added to cart

    pub fn reduce_stock(&mut self, qty: u32) -> Result < (), String> {

        if self.stock < qty {

            return  Err(format!("Not enough stock.Available: {}", self.stock));
        }

        self.stock -= qty;
        Ok(())

    }


    //restore stock for when item is removed from the cart


    pub fn restore_stock(&mut self, qty: u32) {

        self.stock += qty;
    }


    //get price but this will surely be modified by the rush hour logic i planned


    //this was actually for fudge-fudge-fudge, but since it was for a new lang i thought why not js do it in dummies ll


    pub fn get_price(&self) -> f64 {

        self.price
    }
}




//menu struct to hold all da menu items


#[wasm_bindgen]
pub struct Menu {
    items: Vec<MenuItem>,

}


#[wasm_bindgen]

impl Menu {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Menu {

        let mut items =Vec::new();



        //burgerss!

        items.push(MenuItem::new(
            1,
            "Classic Burger".to_string(),


            "Beef patty, lettuce, tomato".to_string(),
            8.99,
            20,
            "Burgers".to_string(),

        ));

        items.push(MenuItem::new(
            2,
            "Cheese Burger".to_string(),
            "Beef patty, cheddar, lettuce".to_string(),
            9.99,
            20,
            "Burgers".to_string(),
        ));

        //pizzass
        items.push(MenuItem::new(
            3,
            "Margherita Pizza".to_string(),
            "Fresh mozzarella, basil, tomato sauce".to_string(),
            12.99,
            15,
            "Pizzas".to_string(),
        ));

        items.push(MenuItem::new(

            4,
            "Pepperoni".to_string(),
            "Mozzarella, pepperoni, sauce".to_string(),
            13.99,
            15,
            "Pizzas".to_string(),
        ));
        
        



        //drinks, cuz we need sm drinks


        items.push(MenuItem::new(

            5,
            "Coke".to_string(),
            "Cold Coca-Cola".to_string(),
            2.99,
            50,
            "Drinks".to_string(),
        ));


        items.push(MenuItem::new(
            6,
            "Sprite".to_string(),
            "Lemon-lime soda".to_string(),
            2.99,
            50,
            "Drinks".to_string(),
        ));

        items.push(MenuItem::new(

            7,"Fries".to_string(),
            "Crispy golden fries".to_string(),
            3.99,
            30,
            "Sides".to_string(),
        ));



        items.push(MenuItem::new(

            8,
            "Onion Rings".to_string(),
            "Crispy onion rings".to_string(),
            4.99,
            20,
            "Sides".to_string(),

        ));


        crate::console_log!("Menu created with {} items", items.len());

        Menu {items}
    }



        pub fn get_all_items(&self) -> JsValue{

            serde_wasm_bindgen::to_value(&self.items).unwrap()
            //will this make it slow idk, guess well find out


        }



        fn get_item_by_id(&self, id: u32) -> Option<MenuItem>{

            self.items.iter().find(|item| item.id == id).cloned()




        }



        //categoryy

        pub fn get_item_by_category(&self, category: &str) -> JsValue {

            let items: Vec<&MenuItem> = self.items
            .iter()
            .filter(|item| item.category == category)
            .collect();
            serde_wasm_bindgen::to_value(&items).unwrap()



        }



        pub fn get_item_by_id_json(&self, id: u32) -> JsValue{

           match self.items
            .iter()
            .find(|item| item.id == id) {
                Some(item) => serde_wasm_bindgen::to_value(&item).unwrap(),
                None => JsValue::NULL
            }
        }


        //getting unique categoriess


        pub fn get_categories(&self) -> Vec<String> {

            let mut categories = Vec::new();

            for item in &self.items {

                if !categories.contains(&item.category){

                    categories.push(item.category.clone());
                }
            }

            categories

        }



        //to  check if items are still askndskdsdssskza

        pub fn is_item_available(&self, id: u32) -> bool {

            self.get_item_by_id(id)
            .map(|item| item.is_available())

            .unwrap_or(false)
        }

        pub fn reduce_item_stock(&mut self, id: u32, qty: u32) -> bool {
            for item in &mut self.items {
                if item.id == id {
                    return item.reduce_stock(qty).is_ok();
                }
            }
            false
        }
    }