//learning rust for the first time so lets see how it all goess



use wasm_bindgen::prelude::*;



//wga an thinking is that i'd js create seperate files so they are easy to organise



mod menu;
mod cart;
mod pricing;
mod order;




//js


pub use menu::*;
pub use cart::*;

pub use pricing::*;
pub use order::*;


#[wasm_bindgen(start)]


pub fn init() {


    //uh js gunnga panic hook so it shows errors in browser console



    #[cfg(feature = "console_error_panic_hook")]

    console_error_panic_hook::set_once();



    console.log("rustybite initialized");



}



#[wasm_bindgen]

extern "c" {

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {

    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

//yup first commit, it was pretty hard took me so many trips to the documentation lmaoo