use crate::models::category::Category;
use crate::models::product::Products;
use crate::pages::products::ProductCards;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct CategoryPageParams {
    id: u32,
    tag: Option<String>,
}

#[derive(Params, PartialEq)]
struct CategoryPageQuery {
    page: u32,
}

#[component]
pub fn CategoryPage() -> impl IntoView {
    let params = use_params::<CategoryPageParams>();
    let query = use_query::<CategoryPageQuery>();
    let cat_id =
        move || params.with(|params| params.as_ref().map(move |params| params.id).unwrap_or(0));
    let tag = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(move |params| params.tag.clone())
                .unwrap_or(None)
        })
    };
    let page = move || query.with(|query| query.as_ref().map(|query| query.page).unwrap_or(1));
    let products_query_params = move || (cat_id(), page(), tag());
    let products = create_resource(products_query_params, |value| async move {
        Products::load(value.0, value.1, value.2).await
    });
    let products = move || match products.get() {
        None => Some(Products::default()),
        Some(products) => products,
    };

    let set_category =
        use_context::<WriteSignal<Option<Category>>>().expect("Set category signal not found");
    let category = create_resource(cat_id, |value| async move { Category::load(value).await });

    let category = move || match category.get() {
        None => None,
        Some(category) => match category {
            None => None,
            Some(category) => {
                set_category(Some(category.clone()));
                Some(category)
            }
        },
    };

    let category_name = move || match category() {
        None => None,
        Some(category) => match category.name.len() {
            0 => None,
            _ => Some(category.name.clone()),
        },
    };

    view! {
        <div class="container">
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="/">"Главная"</a></li>
                    {
                        move || {
                            match category_name() {
                                None => view! {  }.into_view(),
                                Some(category_name) => view! { <li class="breadcrumb-item active" aria-current="page">"Категория: "<a href=format!("/category/{}", cat_id())>{category_name}</a></li> }.into_view(),
                            }
                        }
                    }
                    {
                        move || {
                            match tag() {
                                None => view! {  }.into_view(),
                                Some(tag_name) => view! { <li class="breadcrumb-item active" aria-current="page">"Тег: "{tag_name}</li> }.into_view(),
                            }
                        }
                    }
                </ol>
            </nav>
            <ProductCards products=products />
        </div>
    }
}
