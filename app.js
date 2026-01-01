// js for da bridge to wasm

//ngl i still dont fully understand how wasm lloading works lol

import init, {

    Menu,
    Cart,
    Pricing,
    Order,
    OrderStatus
} from './pkg/rustybite.js';


let menu;
let cart;
let pricing;
let orders = [];
let currentHour = new Date().getHours();
let orderIdCounter = 1;


async function initApp() {

    try {

        await init();
        console.log('wasm loaded succesfully yayy')


        menu = new Menu();
        cart = new Cart();

        pricing = new Pricing();

        console.log('menu, cart and pricing initialized');

        renderMenu();
        renderCategories();
        updateRushHourIndicator();

        updateCart();



        setInterval(() => {

            currentHour = new Date().getHours();
            updateRushHourIndicator();
            updateCart();
        }, 60000);
    } catch (err) {

        console.error('failed to load wasm:', err);
        document.body.innerHTML = '<h1>failed to load app :(</h1><p>' + err + '</p>';


    }
}




//render menu itemss

function renderMenu(category = 'all') {

    const menuGrid = document.getElementById('menu-items');

    menuGrid.innerHTML = '';


    const items = menu.get_all_items();

    console.log('rendering menu items:', items.length);

    items.forEach(item => {


        if (category !== 'all' && item.category !== category ) {

            return;
        }

        const itemDiv = document.createElement('div');
        itemDiv.className = 'menu-item';

        const isAvailable = item.stock > 0;
        const stockText = isAvailable ? `stock: ${item.stock}` : 'out of stock';
        const multiplier = pricing.get_multiplier(currentHour);
        const finalPrice = item.price * multiplier;
        const priceText = multiplier !== 1.0 ? `$${finalPrice.toFixed(2)} <small>(${multiplier}x)</small>` : `$${item.price.toFixed(2)}`;

        itemDiv.innerHTML = `
        
        <h3>${item.name}</h3>
        
        <p>${item.description}</p>
        <div class = "price">${priceText}</div>
        <div class = "stock">${stockText}</div>
        
        <button onclick = "addToCart(${item.id})" ${!isAvailable ? 'disabled' : ''}>
        
        add to cart
        
        </button>
        
        `;


        menuGrid.appendChild(itemDiv);

    });


}




//rendering category filters

function renderCategories() {

    const filters = document.querySelector('.category-filters');
    const categories = menu.get_categories();


    //add all button
    const allBtn = document.createElement('button');
    allBtn.className = 'filter-btn active';
    allBtn.textContent = 'all';
    allBtn.onclick = () => {
        document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
        allBtn.classList.add('active');
        renderMenu('all');
    };
    filters.appendChild(allBtn);


    categories.forEach(cat => {

        const btn = document.createElement('button');
        btn.className = 'filter-btn';
        btn.textContent = cat;
        btn.dataset.category = cat;
        btn.onclick = () => {


            document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            
            renderMenu(cat);
        };

        filters.appendChild(btn);

    });
}



window.addToCart = function(itemId) {

    console.log('adding item to cart:', itemId);

    const item = menu.get_item_by_id_json(itemId);

    if (!item) {
        console.error('item not found');
        return;
    }


    try {
        cart.add_item(item.id, item.name, 1, item.price);
        console.log('item added to cart');
        updateCart();

    } catch(err) {
        console.error('error adding to cart:', err);
        alert('failed to add item: ' + err);
    }
};



//update cart display

function updateCart() {

    const cartItems = document.getElementById('cart-items');
    const cartTotal = document.getElementById('total');
    const checkoutBtn = document.getElementById('checkout-btn');


    cartItems.innerHTML = '';


    const items = cart.get_items();

    console.log('cart has', items.length, 'items');


    if (items.length === 0) {

        cartItems.innerHTML = '<p class="empty-cart">cart is empty</p>';
        cartTotal.textContent = '$0.00';
        checkoutBtn.disabled = true;
        return;
    }

    checkoutBtn.disabled = false;


    items.forEach((item, index) => {

        const itemDiv = document.createElement('div');
        itemDiv.className = 'cart-item';

        itemDiv.innerHTML = `
        
        <div class="cart-item-info">
        
        <strong>${item.item_name}</strong>
        <div class = "cart-item-price">$${item.unit_price.toFixed(2)} x ${item.quantity}</div>
        
        </div>
        
        <div class = "cart-item-controls">
        
        <button onclick = "updateQty(${index}, ${item.quantity - 1})">-</button>
        <span>${item.quantity}</span>
        <button onclick = "updateQty(${index}, ${item.quantity + 1})">+</button>
        <button onclick="removeItem(${index})" class = "remove-btn">remove</button>
        
        </div>
        
        `;

        cartItems.appendChild(itemDiv);
    });


    //calculate total with rush hour pricing

    try {
        const total = pricing.calculate_total(cart, currentHour);
        const subtotal = pricing.calculate_subtotal(cart, currentHour);
        const tax = subtotal * 0.08;
        const fee = 2.50;

        document.getElementById('subtotal').textContent = `$${subtotal.toFixed(2)}`;
        document.getElementById('tax').textContent = `$${tax.toFixed(2)}`;
        document.getElementById('service-fee').textContent = `$${fee.toFixed(2)}`;
        cartTotal.textContent = `$${total.toFixed(2)}`;

        console.log('pricing breakdown:', {subtotal, tax, fee, total});

    } catch (err) {
        console.error('error calculating total:', err);
        cartTotal.textContent = '$0.00';
    }
}



window.updateQty = function(index, newQty) {

    console.log('updating qty at index', index, 'to', newQty);

    try {
        cart.update_quantity(index, newQty);
        updateCart();
    } catch (err) {
        console.error('error updating qty:', err);
    }
};


window.removeItem = function(index) {

    console.log('removing item at index', index);

    try {
        cart.remove_item(index);
        updateCart();
    } catch (err) {
        console.error('error removing item:', err);
    }
};




//update rush hour indicator

function updateRushHourIndicator() {

    const indicator = document.getElementById('rush-hour-indicator');

    const message = pricing.get_rush_hour_message(currentHour);
    const isRush = pricing.is_rush_hour(currentHour);


    indicator.textContent = message;
    indicator.className = isRush ? 'rush-active' : 'rush-normal';


    console.log('rush hour update:', message);
}




//place order

window.placeOrder = function() {

    console.log('placing order...');


    if (cart.is_empty()) {
        alert('cart is empty!');
        return;
    }


    try {

        const total = pricing.calculate_total(cart, currentHour);
        const items = cart.get_items();
        const currentTime = Math.floor(Date.now() / 1000);
        const isRush = pricing.is_rush_hour(currentHour);


        const order = new Order(
            orderIdCounter++,
            items,
            total,
            currentTime,
            isRush
        );


        orders.push(order);


        console.log('order created:', order.get_summary());

        alert('order placed! estimated wait: ' + order.estimated_wait_minutes + ' mins');


        //clear cart

        cart.clear();
        updateCart();


        //simulate order progression

        simulateOrderProgress(order);


    } catch (err) {
        console.error('error placing order:', err);
        alert('failed to place order: ' + err);
    }
};



//simulate order status changes

function simulateOrderProgress(order) {

    console.log('simulating order progress for order', order.id);


    const interval = setInterval(() => {

        if (order.is_completed() || order.is_cancelled()) {
            clearInterval(interval);
            console.log('order simulation complete');
            return;
        }


        const newStatus = order.advance_status();
        console.log('order status updated:', newStatus);


        //show notification

        if (newStatus === 'Ready') {
            alert('order #' + order.id + ' is ready!');
        }


    }, 5000);  //advance every 5 seconds for demo
}




//init app on load

window.addEventListener('DOMContentLoaded', initApp);