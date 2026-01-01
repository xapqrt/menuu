//order management and timing logic



use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

use crate::cart::CartItem;

//order sATUS enumm (yay thisis my first time usin enum properly!!)




#[wasm_bindgen]


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]



pub enum OrderStatus {

    Pending,
    Confirmed,

    Cooking,
    Ready,
    Completed,
    Cancelled,


}


#[wasm_bindgen]

#[derive(Clone, Debug, Serialize, Deserialize)]



pub struct Order {

    pub id: u32,
    items: Vec<CartItem>,

    pub total: f64,
    status: OrderStatus,


    pub created_at: u32,
    pub ready_at: u32,

    pub estimated_wait_minutes: u32,





}





#[wasm_bindgen]

impl Order {


        #[wasm_bindgen(constructor)]

        pub fn new(

            id: u32,
            items_js: JsValue,

            total: f64,
            current_time: u32,
            is_rush_hour: bool,


        ) -> Order {

        let items: Vec<CartItem> = serde_wasm_bindgen::from_value(items_js).unwrap();
        let item_count = items.len() as u32;

        let estimated_wait = Order::estimate_wait_time(item_count, is_rush_hour);



        let ready_at = current_time + (estimated_wait * 60); 



        crate::console_log!("Order {} created, wait time: {} mins", id, estimated_wait);


        Order {


            id,
            items,
            total,
            status: OrderStatus::Pending,
            created_at: current_time,
            ready_at,
            estimated_wait_minutes: estimated_wait,

        }
    }



    //estimating wait tim ebased on items
    //and rush hur



    pub fn estimate_wait_time(item_count: u32, is_rush: bool) -> u32 {


        let base_time = 5;

        let mut wait = base_time * item_count;



        //rush hour adds extra time cuz kitchen is obv busy


        //u see what am trying to do, am creating a coding marvel, this one of a kind fake restaurant thats gonna save the earth



        //yap yap yap



        if is_rush {

            wait = (wait as f64 * 1.5) as u32;
        }


        if wait < 10 {

            wait = 10;

        }



        if wait > 60 {

            wait = 60;
        }
        wait
    }



    pub fn advance_status(&mut self) -> String {

        let old_status = self.status.clone();



        self.status = match self.status {

            OrderStatus::Pending => OrderStatus::Confirmed,


            OrderStatus::Confirmed => OrderStatus::Cooking,

            OrderStatus::Cooking => OrderStatus::Ready,
            OrderStatus::Ready => OrderStatus::Completed,

            OrderStatus::Completed => OrderStatus::Completed,



            OrderStatus::Cancelled => OrderStatus::Cancelled,


    };




    let status_str = self.get_status_string();

    crate::console_log!("Order {} Staus: {:?} -> {:?}", self.id, old_status, self.status);


    status_str
}


//gettin status ass tring


pub fn get_status_string(&self) -> String {

    match self.status {

        OrderStatus::Pending => "Pending".to_string(),
        OrderStatus::Confirmed => "Confirmed".to_string(),
        OrderStatus::Cooking => "Cooking".to_string(),

        OrderStatus::Ready => "Ready".to_string(),

        OrderStatus::Completed => "Completed".to_string(),

        OrderStatus::Cancelled => "Cancelled".to_string(),
    }
}



//cancelinng order

pub fn cancel(&mut self) -> Result<(), String> {


    if self.status == OrderStatus::Completed {

        return  Err("Cannot cancel completed order".to_string());

    }


    self.status = OrderStatus::Cancelled;

    crate::console_log!("Order {} cancelled", self.id);


    Ok(())
}





//checking if order is complete


pub fn is_completed(&self) -> bool {

    self.status == OrderStatus::Completed
}



pub fn is_cancelled(&self) -> bool {

    self.status == OrderStatus::Cancelled
}




//getting roder summary


pub fn get_summary(&self) -> String {


    format!(

        "Order #{}: {} items, ${:.2}, Status: {}, Wait: {} mins",
        self.id,
        self.items.len(),
        self.total,
        self.get_status_string(),
        self.estimated_wait_minutes

    )
}






//getting items as json string


pub fn get_items_json(&self) -> String {

    serde_json::to_string(&self.items).unwrap_or("[]".to_string())
}
}