# 🦀 RustyBite - Food Ordering App

yo so i built this food ordering app to learn rust and wasm together

ngl its been a journey lol


## what is this

basically its a fake restaurant ordering system but ALL the logic is in rust compiled to webassembly

like the menu, cart, pricing, rush hour multipliers, order tracking - everything runs in rust in the browser




## why tho

- wanted to learn rust (ownership is still confusing sometimes)
- wanted to understand wasm (still dont fully get it but it works)
- make something cool for portfolio

--cuz yk it looks diversified that way


## tech stack



## features

### menu system
- 8 hardcoded items (burgers, pizza, drinks, sides)
- stock tracking
- category filters
- all managed by rust Vec<MenuItem>

### shopping cart
- add/remove/update items
- deduplication (if item already in cart, increase qty)
- subtotal calculations
- Result type for validation

### rush hour pricing!! (the cool part)
- **11am-2pm**: 1.5x multiplier (lunch rush)
- **2pm-5pm**: 0.9x discount (afternoon slump)
- **5pm-9pm**: 1.3x multiplier (dinner rush)
- **9pm+**: 1.0x normal pricing
- 8% tax + $2.50 service fee
- all calculated in rust

### order management
- OrderStatus enum: Pending → Confirmed → Cooking → Ready → Completed
- estimated wait time based on items + rush hour
- order simulation (status advances every 5 seconds)
- match expressions for state transitions


## how to run

### build wasm
```bash
# install wasm-pack (one time)
curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh

# build
wasm-pack build --target web --release

# this creates pkg/ folder with wasm binary
```

### serve locally
```bash
# python
python -m http.server 8000

# or node
npx serve

# visit http://localhost:8000
```




## what i learned

**rust:**
- structs and impl blocks
- ownership (still confusing with &mut self vs self)
- borrowing (used .clone() everywhere, probably inefficient)
- Option<T> and Result<T, E>
- pattern matching with match
- enums with variants
- iterators (.iter(), .map(), .filter(), .find())
- wasm-bindgen macros

**wasm:**
- compiling rust to wasm
- js  and rust bridge
- memory management
- performance (wasm is FAST)
- export functions with #[wasm_bindgen]

**mistakes i made:**
- cloning too much (should learn lifetimes)
- repeated if statements (should refactor)
- pub everywhere (should encapsulate better)
- console logs everywhere (but helped debugging)
- typos in comments (oops)


## known issues

- no persistence 
- hardcoded menu items (should load from json)
- rush hour based on client time (not server)
- no real payment processing
- excessive cloning (performance concern?)
- TODO: stock checking when adding to cart


## future improvements

- add unit tests (learning how to test rust)
- better error messages
- order history persistence
- real backend integration
- optimize cloning
- learn lifetimes properly
- add animations
- mobile responsive design


## time spent

like 8-10 hours total? rust learning curve is real

spent way too long on:
- borrowing rules
- wasm-bindgen exports
- pattern matching syntax
- understanding ownership
- debugging type errors




## conclusion

rust is hard but worth it

wasm is magic

compiler errors are actually helpful

would recommend for learning


---

built by someone learning rust for the first time

if you found this helpful or have suggestions, lmk!

no ai was used in writing this code (just learning from docs and stackoverflow lma)
