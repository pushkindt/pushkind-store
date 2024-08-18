use crate::env;
use crate::{models::product::Product, utils::make_backend_url};
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ProductPageParams {
    id: i32,
}
#[component]
pub fn ProductPage() -> impl IntoView {
    let params = use_params::<ProductPageParams>();
    let id =
        move || params.with(|params| params.as_ref().map(move |params| params.id).unwrap_or(0));

    let product = create_resource(id, |value| async move { Product::load(value).await });

    let product = move || product.get();

    view! {
        <div class="container">
        {
            move || match product() {
                None => view! { <div class="col"><div class="spinner-border" role="status"><span class="visually-hidden">Загрузка...</span></div></div> }.into_view(),
                Some(product) => match product {
                    Some(product) => view! {
                        <h5>{product.name}</h5>
                        <div class="row">
                            <div class="col">
                                <img src=make_backend_url(&product.image.unwrap_or(env::APP_DEFAULT_PRODUCT_IMAGE.to_string())) alt="" />
                            </div>
                            <div class="col">
                                <p>{product.description.unwrap_or_default()}</p>
                                <p>"Цена: "{format!("{:.2}",product.price)}</p>
                                <form>
                                    <div class="input-group mb-3">
                                        <span class="input-group-text">"Количество"</span>
                                        <input type="number" class="form-control" min="0" step="1" aria-label="Количество" value="" />
                                        <span class="input-group-text">{product.measurement.unwrap_or_default()}</span>
                                    </div>
                                    <div class="mb-3">
                                        <button type="submit" class="btn btn-primary">"В корзину"</button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    }.into_view(),
                    None => view! { <Redirect path="/"/> }.into_view(),
                },
            }
        }
        </div>
    }
}
