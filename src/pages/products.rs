use crate::env;
use crate::models::product::{Product, Products};
use crate::utils::{make_backend_url, Paginator};
use leptos::*;

#[component]
fn ProductPagination(#[prop(into)] products: Signal<Option<Products>>) -> impl IntoView {
    let paginator = move || match products.get() {
        None => Paginator::new(1, 1),
        Some(products) => Paginator::new(products.page as usize, products.pages as usize),
    };

    let current_page = move || match products.get() {
        None => 1,
        Some(products) => products.page,
    };

    view! {
        <nav>
            <ul class="pagination justify-content-center flex-wrap">
                {
                    move || paginator().iter_pages(2, 2, 4, 2).map(|page|
                        match page {
                            None => view! { <li class="page-item disabled"><a class="page-link" href="#">...</a></li> }.into_view(),
                            Some(page) if page == current_page() as usize => view! {
                                <li class="page-item active">
                                    <a class="page-link" href=format!("?page={}", page)>{page.to_string()}</a>
                                </li>
                            }.into_view(),
                            Some(page) => view! {
                                <li class="page-item">
                                    <a class="page-link" href=format!("?page={}", page)>{page.to_string()}</a>
                                </li>
                            }.into_view(),
                        }
                    ).collect::<Vec<_>>()
                }
            </ul>
        </nav>
    }
}

#[component]
pub fn ProductCards(#[prop(into)] products: Signal<Option<Products>>) -> impl IntoView {
    let products_products = move || match products.get() {
        None => vec![],
        Some(products) => products.products,
    };

    view! {
        <ProductPagination products=products />
        <div class="row row-cols-1 row-cols-lg-6 row-cols-md-4 row-cols-sm-2">
            {
                move || match products.get() {
                    None => view! { <div class="col"><div class="spinner-border" role="status"><span class="visually-hidden">Загрузка...</span></div></div> }.into_view(),
                    Some(products) => match products.total {
                        0 => view! { <div class="col"><div class="spinner-border" role="status"><span class="visually-hidden">Загрузка...</span></div>.</div> }.into_view(),
                        _ => view! {
                            <For each=products_products key=|product| product.id children=move |product| view! {
                                <ProductCard product=product.clone() />
                            } />
                        }.into_view(),
                    },
                }
            }
        </div>
        <ProductPagination products=products />
    }
}

#[component]
fn ProductCard(product: Product) -> impl IntoView {
    // let navigate = leptos_router::use_navigate();
    view! {
        <div class="col my-2">
            <div class="card text-center selectable overflow-hidden h-100" data-id=format!("{}", product.id) data-bs-toggle="modal" data-bs-target="#productModal"> //on:click=move |_| navigate(&format!("/product/{}", product.id), Default::default())>
                <img class="card-img-top" src=make_backend_url(&product.image.unwrap_or(env::APP_DEFAULT_PRODUCT_IMAGE.to_string())) alt="thumbnail" />
                <div class="card-body py-0">
                    <h5 class="card-title text-start">{format!("{:.2}", product.price)} "₽"</h5>
                </div>
                <div class="card-footer bg-white border-top-0 text-start">
                    {product.name}
                </div>
            </div>
        </div>
    }
}
