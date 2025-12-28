//shooping cartt logiccs

use wasm_bindgen::prelude::*;


use serde::{Serialize, Deserialize};


#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Debug)]

pub struct CartItem {

    pub menu_item_id: u32,
    pub item_name : String,
    pub quantity: u32,
    pub unit_price: f64,
    pub subtotal: f64,






}


#[wasm_bindgen] 

impl CartItem {

    pub fn new(

        menu_item_id: u32.
        pub item_name: String,
        pub quantity u32,
        pub unit_price: f64,
        pub subtotal: f64,

    )


    #[wasm_bindgen]

    impl CartItem {

        pub fn new (

            menu_item_id: u32,
            item_name: String,
            quantity: u32,
            unit_price: f64,

        ) -> CartItem {



            let subtotal = unit_price * quantity as f64;

            CartItem {

                menu_item_id,

                item_name,
                quantity,
                unit_price,
                subtotal,

            }
        }


        //recaalculating subtotal after the qty changes yk


        pub fn recalculate_subtotal(&mut self) {

            self.subtotal = self.unit_price * self.quantity as f64;

        
    }
}




#[wasm_bindgen]

pub struct Cart {

    items: Vec<CartItem>,
}



#[wasm_bindgen]

impl Cart {

    #[wasm_bindgen(constructor)]


    pub fn new() -> Cart {

        crate::console_log!("cart created");

        Cart {

            items: Vec::new(),  
        }
    }



    //adding item to cart


    pub fn add_item 9

    &mut self,
    menu_item_id : u32,
    item_name: String,
    quantity: u32,
    unit_price: f64,
} -> Result<(), String> {

    if quantity == 0 {

        return Err("Quantity cannot be zero".to_string());


    }





    //checking if item alr exists in cart or nott

    if let Some(cart_item) = self.items.iter_mut().find(|x| x.menu_item_id == menu_item_id)   {





        cart_item.quantity += quantity;

        cart_item.recalculate_subtotal();


        crate::console_log!("Updated {} in cart", item_name);


    } else {

        self.items.push(CartItem::new(

            menu_item_id,
            item_name.clone(),
            quantity,
            unit_price,

        ));

        crate::console_log!("Added {} to cart", item_name);


    }




    ok(())
}


//removing items by indexx

pub fn remove_item(&mut self, index: usize) -> Result<String, String> {
    if index >= self.items.len()  {

        return Err(format!("Invalid index: {}", index));


    }


    let removed = self.items.remove(index);


    crate::console_log!("removed {}", removed.item_name);



    Ok(removed.item_name)


}



//updating quanitty

pub fn update_quantity(&mut slef, index: usize, new_qty: u32) -> Result<(), String> {

    if index >= self.items.len() {

        return Err(format!("Invalid index: {}", index));



    }

    if new_qty == 0 {

        self.remove_item(index)?;


    } else  {

        self.items[index].quantity = new_qty;
        
    }


}