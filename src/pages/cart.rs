use crate::env;
use crate::models::alert::{AlertMessage, AlertType};
use crate::models::cart::{CartItem, ShoppingCart};
use crate::models::product::{PriceLevel, Product};
use crate::models::user::User;
use leptos::*;
use leptos_oidc::{Authenticated, LoginLink};
use std::time::Duration;
use web_sys::window;

#[component]
pub fn CartModal() -> impl IntoView {
    let get_cart = expect_context::<Signal<ShoppingCart>>();

    let get_user = expect_context::<ReadSignal<User>>();
    let set_cart = expect_context::<WriteSignal<ShoppingCart>>();
    let set_alert = expect_context::<WriteSignal<AlertMessage>>();

    let cart_count = move || get_cart().get_item_count();

    let price_level = move || get_user().price_level;
    let discount = move || get_user().discount;

    let cart_sum = move || get_cart().get_total_price(&price_level(), discount());

    let cart_items = move || get_cart().items.values().cloned().collect::<Vec<_>>();

    let user_email = move || get_user().email.clone();
    let user_phone = move || get_user().phone.clone().unwrap_or_default();

    let access_token = expect_context::<ReadSignal<Option<String>>>();
    let on_submit = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            match get_cart().submit(&access_token()).await {
                Ok(_) => {
                    set_cart.update(|cart| {
                        cart.clear();
                        cart.comment = None;
                    });
                    set_alert.set(AlertMessage {
                        message: "Заказ оформлен. Вы будете перенаправлены в личный кабинет."
                            .to_string(),
                        alert_type: AlertType::Success,
                        visible: true,
                    });
                    set_timeout(
                        || {
                            if let Some(window) = window() {
                                window.location().set_href(env::APP_BACKEND_URL).unwrap();
                            };
                        },
                        Duration::new(2, 0),
                    );
                }
                Err(_) => {
                    set_alert.set(AlertMessage {
                        message: "Ошибка создания заказа. Попробуйте позже.".to_string(),
                        alert_type: AlertType::Danger,
                        visible: true,
                    });
                }
            }
        })
    };

    view! {
        <div class="modal fade" id="cartModal" tabindex="-1" aria-labelledby="cartModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="cartModalLabel">"Корзина"</h5>
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
                                    .map(|item| view! { <CartLineItem item=item price_level=Signal::derive(price_level) discount=Signal::derive(discount) /> })
                                    .collect_view()
                                }
                            }
                                <div class="row">
                                    <label for="shoppingCartEmail" class="col-sm-3 col-form-label">"Электронный адрес:"</label>
                                    <div class="col-sm-9">
                                        <input required name="email" readonly type="text" class="form-control-plaintext" id="shoppingCartEmail" placeholder="<не авторизован>" prop:value=user_email />
                                    </div>
                                </div>
                                <div class="row">
                                    <label for="shoppingCartPhone" class="col-sm-3 col-form-label">"Телефон:"</label>
                                    <div class="col-sm-9">
                                        <input readonly type="text" class="form-control-plaintext" id="shoppingCartPhone" placeholder="<не авторизован>" prop:value=user_phone />
                                    </div>
                                </div>
                                <div class="row">
                                    <label for="shoppingCartPriceLevel" class="col-sm-3 col-form-label">"Уровень цен:"</label>
                                    <div class="col-sm-9">
                                        <input readonly type="text" class="form-control-plaintext" id="shoppingCartPriceLevel" prop:value={move||format!("{}", price_level())} />
                                    </div>
                                </div>
                                <div class="row">
                                    <label for="shoppingCartDiscount" class="col-sm-3 col-form-label">"Cкидка (%):"</label>
                                    <div class="col-sm-9">
                                        <input readonly type="text" class="form-control-plaintext" id="shoppingCartDiscount" prop:value={move||format!("{:.2}%", discount())} />
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col">
                                        <textarea
                                            prop:value={move||get_cart().comment.clone().unwrap_or_default()}
                                            on:input=move |ev| {
                                                set_cart.update(|cart| cart.comment = Some(event_target_value(&ev)));
                                            }
                                            class="form-control" rows="3" placeholder="Комментарий к заказу" maxlength="256">
                                        </textarea>
                                    </div>
                                </div>
                                <div class="row my-1">
                                    <div class="col text-center">
                                        <Authenticated unauthenticated=move || {
                                            view! {
                                                <div class="alert alert-warning">
                                                    "Необходимо "
                                                    <LoginLink>"Войти"</LoginLink>
                                                    " для оформления заказа."
                                                </div>
                                            }
                                        }>
                                            <button
                                                on:click=on_submit
                                                disabled={move || cart_count() == 0}
                                                type="button"
                                                class="btn btn-primary"
                                                data-bs-dismiss="modal"
                                                aria-label="Close"
                                            >
                                                "Оформить заказ"
                                            </button>
                                        </Authenticated>
                                    </div>
                                </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CartLineItem(
    item: CartItem,
    #[prop(into)] price_level: Signal<PriceLevel>,
    discount: Signal<f32>,
) -> impl IntoView {
    let set_product = expect_context::<WriteSignal<Option<Product>>>();

    let product_clone = item.product.clone();

    let product_image = item.product.get_image();
    let product_price = move || product_clone.get_price(&price_level(), discount());
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
                    <div class="col text-end">
                        <span class="link-primary">"изменить"</span>
                    </div>
                </div>
                <div class="row">
                    <div class="col text-end">
                        {item_quantity}" "{product_measurement}" по "{move || format!("{:.2}", product_price() )}"₽"
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        {item_comment}
                    </div>
                </div>
            </div>
        </div>
    }
}
