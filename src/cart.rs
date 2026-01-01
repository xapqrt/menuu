//shooping cartt logiccs

use wasm_bindgen::prelude::*;


use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, Debug)]

pub struct CartItem {

    pub menu_item_id: u32,
    pub item_name : String,
    pub quantity: u32,
    pub unit_price: f64,
    pub subtotal: f64,

}


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


    pub fn add_item(

    &mut self,
    menu_item_id : u32,
    item_name: String,
    quantity: u32,
    unit_price: f64,
) {

    if quantity == 0 {

        crate::console_log!("Quantity cannot be zero");
        return;

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

pub fn update_quantity(&mut self, index: usize, new_qty: u32) -> Result<(), String> {

    if index >= self.items.len() {

        return Err(format!("Invalid index: {}", index));



    }

    if new_qty == 0 {

        self.remove_item(index)?;


    } else  {

        self.items[index].quantity = new_qty;
        self.items[index].recalculate_subtotal();


        crate::console_log!(

            "updated {} qty to {}",
            self.items[index].item_name, new_qty
        );
    }

    Ok(())


}





//getting all itemss

pub fn get_items(&self) -> JsValue {

    serde_wasm_bindgen::to_value(&self.items).unwrap()
}



//huff, i still see one error idk what tf is it



// getting cart item count


pub fn item_count(&self) -> usize {

    self.items.len()
}




//getting total items and that is js the sum of quantities


pub fn total_items(&self) -> u32 {

    self.items.iter().map(|x| x.quantity).sum()

}




//duhh this is so boring istg, idk why copilot autocomplete is not working like duh


pub fn subtotal(&self) -> f64 {

    self.items.iter().map(|x| x.subtotal).sum()     

}




//uh clearing  cart


pub fn clear(&mut self) {

    self.items.clear();





    //crate consoleeee

    crate::console_log!("cart cleared!");
}




pub fn is_empty(&self) -> bool {

    self.items.is_empty()

}
}

//finally this thing is done idek how am  gonna do wasm i swear

