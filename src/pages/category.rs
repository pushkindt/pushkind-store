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
    page: Option<u32>,
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
    let page =
        move || query.with(|query| query.as_ref().map(|query| query.page).unwrap_or(Some(1)));

    let (get_sort_by, set_sort_by) = create_signal("name_asc".to_string());

    let products_query_params = move || (cat_id(), page(), tag(), get_sort_by());
    let products = create_resource(products_query_params, |value| async move {
        Products::load(value.0, value.1, &value.2, &Some(value.3)).await
    });
    let products = move || match products.get() {
        None => Some(Products::default()),
        Some(products) => products,
    };

    let set_category = expect_context::<WriteSignal<Option<Category>>>();
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
            <div class="row my-1">
                <div class="col">
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
                </div>
                {
                    move || {
                        if cat_id() != 0 {
                            view! {
                                <div class="col col-md-2 text-end">
                                    <select name="sort_by" class="form-select" aria-label="Сортировка"
                                        on:change=move |ev| {
                                            let new_value = event_target_value(&ev);
                                            set_sort_by(new_value);
                                        }
                                        prop:value=move || get_sort_by.get().to_string()
                                    >
                                        <option value="name_asc">"↑ Название"</option>
                                        <option value="name_desc">"↓ Название"</option>
                                        <option value="price_asc">"↑ Цена"</option>
                                        <option value="price_desc">"↓ Цена"</option>
                                    </select>
                                </div>
                            }.into_view()
                        } else {
                            view! {  }.into_view()
                        }
                    }
                }
            </div>
            <ProductCards products=products />
        </div>
    }
}
