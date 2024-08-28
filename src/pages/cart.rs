use crate::env;
use crate::models::product::{PriceLevel, Product};
use crate::models::shopping_cart::{CartItem, ShoppingCart};
use crate::models::user::User;
use crate::utils::make_backend_url;
use leptos::*;
use leptos_oidc::{Authenticated, LoginLink};

#[component]
pub fn CartModal() -> impl IntoView {
    let get_cart = expect_context::<Signal<ShoppingCart>>();

    let set_cart = expect_context::<WriteSignal<ShoppingCart>>();

    let get_user = expect_context::<ReadSignal<User>>();

    let cart_count = move || get_cart().items.len();

    let cart_sum = move || {
        get_cart().items.values().fold(0.0, |acc, item| {
            acc + item.product.get_price(&get_user().price_level) * item.quantity as f32
        })
    };

    let cart_json_string = move || {
        let cart = get_cart();
        serde_json::to_string(&cart).unwrap()
    };

    let cart_items = move || get_cart().items.values().cloned().collect::<Vec<_>>();

    let price_level = move || get_user().price_level;

    let user_email = move || get_user().email.clone();

    // let on_submit = move |_| {
    //     set_cart.update(|cart| {
    //         cart.items.clear();
    //     });
    // };

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

                            { move || {
                                    cart_items().into_iter()
                                    .map(|item| view! { <CartLineItem item=item price_level=Signal::derive(price_level) /> })
                                    .collect_view()
                                }
                            }

                            <form method="POST" action=make_backend_url(env::APP_CART_URL) enctype="multipart/form-data">

                                <div class="row my-1">
                                    <label for="shoppingCartEmail" class="col-sm-3 col-form-label">"Электронный адрес:"</label>
                                    <div class="col-sm-9">
                                        <input name="email" readonly type="text" class="form-control-plaintext" id="shoppingCartEmail" placeholder="<не авторизован>" prop:value=user_email />
                                    </div>
                                </div>
                                <div class="row my-1">
                                    <label for="shoppingCartPriceLevel" class="col-sm-3 col-form-label">"Уровень цен:"</label>
                                    <div class="col-sm-9">
                                        <input readonly type="text" class="form-control-plaintext" id="shoppingCartPriceLevel" prop:value={move||price_level().to_string()} />
                                        <input name="price_level" type="hidden" prop:value={move||price_level() as u8} />
                                    </div>
                                </div>

                                <div class="row my-1">
                                    <div class="col">
                                        <textarea name="comment" class="form-control" rows="3" placeholder="Комментарий к заказу" maxlength="256">
                                        </textarea>
                                    </div>
                                </div>
                                <div class="row my-1">
                                    <div class="col">
                                        <input type="hidden" name="cart" prop:value=cart_json_string />
                                        <Authenticated unauthenticated=move || {
                                            view! {
                                                <LoginLink class="text-muted ms-3">"Авторизоваться"</LoginLink>
                                            }
                                        }>
                                            <button type="submit" class="btn btn-primary">
                                                "Заказать"
                                            </button>
                                        </Authenticated>
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
fn CartLineItem(item: CartItem, #[prop(into)] price_level: Signal<PriceLevel>) -> impl IntoView {
    let set_product = expect_context::<WriteSignal<Option<Product>>>();

    let product_clone = item.product.clone();

    let product_image = item.product.get_image();
    let product_price = move || product_clone.get_price(&price_level());
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
                        {item_quantity}" "{product_measurement}" по "{move || format!("{:.2}", product_price() )}"₽"
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
