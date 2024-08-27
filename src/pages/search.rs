use crate::models::product::Products;
use crate::pages::products::ProductCards;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct SearchPageQuery {
    q: String,
    page: Option<u32>,
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let query = use_query::<SearchPageQuery>();

    let page =
        move || query.with(|query| query.as_ref().map(|query| query.page).unwrap_or(Some(1)));
    let search = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.q.clone())
                .unwrap_or("".to_string())
        })
    };

    let search_page = move || (search(), page());

    let products = create_resource(search_page, |value| async move {
        Products::search(&value.0, value.1.unwrap_or(1)).await
    });

    let products = move || match products.get() {
        None => Some(Products::default()),
        Some(products) => products,
    };

    view! {
        <div class="container">
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="/">Поиск</a></li>
                    <li class="breadcrumb-item active" aria-current="page">{search}</li>
                </ol>
            </nav>
            <ProductCards products=products />
        </div>
    }
}
