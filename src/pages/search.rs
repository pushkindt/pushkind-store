use crate::models::category::Category;
use crate::models::product::Products;
use crate::pages::products::ProductCards;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct CategoryPageParams {
    id: i32,
}
#[derive(Params, PartialEq)]
struct CategoryPageQuery {
    page: i32,
}

#[component]
pub fn SearchPage(set_category: WriteSignal<Option<Category>>) -> impl IntoView {
    let params = use_params::<CategoryPageParams>();
    let query = use_query::<CategoryPageQuery>();

    let id =
        move || params.with(|params| params.as_ref().map(move |params| params.id).unwrap_or(0));

    let category = create_resource(id, |value| async move { Category::load(value).await });
    let page = move || query.with(|query| query.as_ref().map(|query| query.page).unwrap_or(1));

    let id_page = move || (id(), page());

    let products = create_resource(id_page, |value| async move {
        Products::load(value.0, value.1).await
    });

    let products = move || match products.get() {
        None => Some(Products::default()),
        Some(products) => products,
    };

    let category_name = move || match category.get() {
        None => "".to_string(),
        Some(category) => {
            set_category.set(category.clone());
            match category {
                None => "".to_string(),
                Some(category) => category.name,
            }
        }
    };

    view! {
        <div class="container">
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="/">Поиск</a></li>
                    <li class="breadcrumb-item active" aria-current="page">{category_name}</li>
                </ol>
            </nav>
            <ProductCards products=products />
        </div>
    }
}
