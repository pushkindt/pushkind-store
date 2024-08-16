use leptos::*;

#[component]
pub fn CartPage() -> impl IntoView {
    // Mock cart data
    let cart_items = vec!["Product 1", "Product 2"];

    view! {
        <div>
            <h1>"Shopping Cart"</h1>
            <ul>
                {cart_items.iter().map(move |item| view! {
                    <li>{*item}</li>
                }).collect::<Vec<_>>()}
            </ul>
            <button on:click=move |_| {
                // Place order logic
            }>"Place Order"</button>
        </div>
    }
}
