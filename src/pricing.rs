//pricign and rush oour lofics

use wasm_bindgen::prelude::*;

use crate::cart::Cart;



//what am thinking is that rush hour multiplier rules


//uh 11 to 2pm am thinking maybe 1.5x cuz luch rush

//(2 -5)pm : 0.9x cuz idk does anybody even eat at thhat time

//5 to 9 p, is 1.3x cuz dinner rushh
//and finally 9pm+ it will be 1x


#[wasm_bindgen]

pub struct Pricing {

    tax_rate: f64,
    service_fee: f64,
}



#[wasm_bindgen] 
impl Pricing {

    #[wasm_bindgen(constructor)]

    pub fn new() -> Pricing {

        Pricing {

            tax_rate: 0.08, //js the 8% holy gadamn tax
            service_fee: 2.50,
        }
    }



    //checking if its rush or not
    
    pub fn is_rush_hour(&self, hour: u32) -> bool {



        if hour >= 11 && hour < 14 {

            return true;

        }


        if hour >= 17 && hour < 21 {

            return true;
        }



        false
    }



    pub fn get_multiplier(&self, hour: u32) -> f64 {

        if hour >= 11 && hour < 14 {

            return 1.5;

        }


        if hour >= 14 && hour < 17 {

            return 0.9;
        }

        if hour >=17 && hour < 21 {

            return 1.3;
        }


        1.0
    }




    //calculatge item price with thr rush hour logic

    pub fn calculate_item_price(&self, base_price: f64, hour: u32, qty: u32) -> f64 {

            let multiplier = self.get_multiplier(hour);


            let price = base_price * multiplier * qty as f64;



            crate::console_log!("price calc: base={}, mult={}, qty={}, final={}", base_price, multiplier, qty, price);





            price


    }




    //cacalculate subtotal from cart

    pub fn calculate_subtotal(&self, cart: &Cart, hour: u32) -> f64 {



        let items_js = cart.get_items();
        let items: Vec<crate::cart::CartItem> = serde_wasm_bindgen::from_value(items_js).unwrap();
        let mut total = 0.0;



        for item in items {

            let item_total = self.calculate_item_price(item.unit_price, hour, item.quantity);





            total += item_total;


        }


        crate::console_log!("subtotal calculated: {}", total);

        total

}



//applying taxes and fees, i feel like the irs no

pub fn apply_tax_and_fees(&self, subtotal: f64)  -> f64 {

    let tax = subtotal * self.tax_rate;


    let total = subtotal + tax + self.service_fee;



    crate::console_log!("tax: {}, service_fee: {}, final: {}", tax, self.service_fee, total);

    total


}



//calculating final total with everything god this is so week


pub fn calculate_total(&self, cart: &Cart, hour: u32) -> Result<f64, String> {



    if cart.is_empty()  {

        return Err("Cart is empty".to_string());
    }




    let subtotal = self.calculate_subtotal(cart, hour);

    let total = self.apply_tax_and_fees(subtotal);

    Ok(total)
}



pub fn get_rush_hour_message(&self, hour: u32) -> String {


    if hour >= 11 && hour < 14 {

        return "LUNCH RUSH! Prices are 1.5x higher".to_string();


    }


    if hour >=14 && hour < 17 {

        return "Afternoon discount! 10% off!!!".to_string();


    }



    if hour  >= 17 && hour < 21 {


        return "Dinner rushh!! Prices are 1.3x higher".to_string();




    }



    "Normal pricing".to_string()

}
}



//ok i think the logic is mostly and i emphasize on the word mostly, but guess we'll never know (ye reference)


