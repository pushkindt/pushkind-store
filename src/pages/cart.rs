use crate::env;
use crate::models::product::Product;
use crate::models::shopping_cart::{CartItem, ShoppingCart};
use crate::utils::make_backend_url;
use leptos::*;

#[component]
pub fn CartModal() -> impl IntoView {
    let get_cart =
        use_context::<Signal<ShoppingCart>>().expect("Get shopping cart signal not found");

    let cart_count = move || get_cart().items.len();

    let cart_sum = move || {
        get_cart().items.values().fold(0.0, |acc, item| {
            acc + item.product.price * item.quantity as f32
        })
    };

    let cart_json_string = move || {
        let cart = get_cart();
        serde_json::to_string(&cart).unwrap()
    };

    let cart_items = move || get_cart().items.values().cloned().collect::<Vec<_>>();

    view! {
        <div class="modal fade" id="cartModal" tabindex="-1" aria-labelledby="productModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="productModalLabel">"Корзина"</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row my-1">
                                <div class="col">
                                    "Позиций: "{cart_count}" на сумму "{move || format!("{:.2}", cart_sum())}"₽"
                                </div>
                            </div>
                            <For each=cart_items key=|item| item.product.id*item.quantity children=move |item| view! {
                                <CartLineItem item=item.clone() />
                            } />
                            <form method="POST" action=make_backend_url(env::APP_CART_URL)>
                                <div class="row my-1">
                                    <div class="col">
                                        <textarea name="comment" class="form-control" rows="3" placeholder="Комментарий">
                                        </textarea>
                                    </div>
                                </div>
                                <div class="row my-1">
                                    <div class="col">
                                        <input type="hidden" name="cart" value=cart_json_string />
                                        <button type="submit" class="btn btn-primary">
                                            "Заказать"
                                        </button>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CartLineItem(item: CartItem) -> impl IntoView {
    let set_product =
        use_context::<WriteSignal<Option<Product>>>().expect("Set product signal not found");

    let product_image = item.product.get_image();
    let product_price = item.product.price;
    let product_name = item.product.name.clone();
    let product_sku = item.product.sku.clone();
    let product_measurement = item.product.measurement.clone().unwrap_or_default();
    let item_quantity = item.quantity;
    let item_comment = item.comment.clone();

    view! {
        <div class="row my-1 selectable" data-bs-toggle="modal" data-bs-target="#productModal" on:click=move |_| {set_product(Some(item.product.clone()))}>
            <div class="col-auto">
                <img height="64" width="64" src=product_image />
            </div>
            <div class="col-sm-6">
                <h5 class="my-0">{product_name}</h5>
                <div class="row">
                    <div class="col">
                        "Артикул: "{product_sku}
                    </div>
                </div>
            </div>
            <div class="col-sm">
                <div class="row">
                    <div class="col">
                        {item_quantity}" "{product_measurement}" по "{format!("{:.2}", product_price)}"₽"
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <span class="fw-bold">"Комментарий: "</span>{item_comment}
                    </div>
                </div>
            </div>
        </div>
    }
}
