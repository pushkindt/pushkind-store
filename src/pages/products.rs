use crate::models::cart::{CartItem, ShoppingCart};
use crate::models::product::{PriceLevel, Product, Products};
use crate::models::user::User;
use crate::utils::Paginator;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ProductModal() -> impl IntoView {
    let get_product = expect_context::<ReadSignal<Option<Product>>>();

    let get_cart = expect_context::<Signal<ShoppingCart>>();

    let set_cart = expect_context::<WriteSignal<ShoppingCart>>();

    let get_user = expect_context::<ReadSignal<User>>();

    let product_name = move || match get_product() {
        None => "".to_string(),
        Some(product) => product.name,
    };
    let product_image = move || match get_product() {
        None => "".to_string(),
        Some(product) => product.get_image(),
    };
    let product_description = move || match get_product() {
        None => "".to_string(),
        Some(product) => product.description.unwrap_or_default(),
    };
    let product_measurement = move || match get_product() {
        None => "".to_string(),
        Some(product) => product.measurement.unwrap_or_default(),
    };
    let product_price = move || match get_product() {
        None => "0.00".to_string(),
        Some(product) => format!(
            "{:.2}",
            product.get_price(&get_user().price_level, get_user().discount)
        ),
    };
    let product_sku = move || match get_product() {
        None => "".to_string(),
        Some(product) => product.sku.clone(),
    };
    let product_quantity = move || match get_product() {
        None => "".to_string(),
        Some(product) => match get_cart().items.get(&product.id) {
            Some(item) => item.quantity.to_string(),
            None => "".to_string(),
        },
    };
    let product_comment = move || match get_product() {
        None => "".to_string(),
        Some(product) => match get_cart().items.get(&product.id) {
            Some(item) => item.comment.clone().unwrap_or_default(),
            None => "".to_string(),
        },
    };
    let product_options = move || match get_product() {
        None => None,
        Some(product) => match product.options {
            None => None,
            Some(options) => Some(options),
        },
    };

    let quantity_element: NodeRef<html::Input> = create_node_ref();
    let comment_element: NodeRef<html::Textarea> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        if let Some(product) = get_product() {
            let quantity = quantity_element()
                .expect("<input> should be mounted")
                .value();
            let comment = comment_element()
                .expect("<textarea> should be mounted")
                .value();
            set_cart.update(|cart| {
                // cart.add_item(product, quantity.parse().unwrap(), comment);
                let quantity: u32 = quantity.parse().unwrap_or(0);
                if quantity == 0 {
                    cart.items.remove(&product.id);
                    return;
                }
                let item = CartItem {
                    product: product.clone(),
                    quantity,
                    comment: Some(comment),
                };
                cart.items.insert(product.id, item);
            });
        }
    };

    view! {
        <div class="modal fade" id="productModal" tabindex="-1" aria-labelledby="productModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="productModalLabel">{product_name}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row">
                                <div class="col-sm">
                                    <div id="carouselImages" class="carousel slide" data-bs-theme="dark">
                                        <div class="carousel-inner">
                                            {
                                                move || view! {
                                                    <div class="carousel-item active">
                                                        <img src=product_image() class="d-block w-100" alt="Product Image" />
                                                    </div>
                                                }
                                            }

                                            {
                                                move || match get_product() {
                                                    None => view! {}.into_view(),
                                                    Some(product) => match product.images {
                                                        None => view! {}.into_view(),
                                                        Some(images) => images
                                                            .iter()
                                                            .map(|image| {
                                                                view! {
                                                                    <div class="carousel-item">
                                                                        <img src=image class="d-block w-100" alt="Media Image" />
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view(),
                                                    },
                                                }
                                            }
                                        </div>
                                        <button class="carousel-control-prev" type="button" data-bs-target="#carouselImages" data-bs-slide="prev">
                                            <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                                            <span class="visually-hidden">Previous</span>
                                        </button>
                                        <button class="carousel-control-next" type="button" data-bs-target="#carouselImages" data-bs-slide="next">
                                            <span class="carousel-control-next-icon" aria-hidden="true"></span>
                                            <span class="visually-hidden">Next</span>
                                        </button>
                                    </div>
                                    {
                                        move || match product_options() {
                                            None => view! {}.into_view(),
                                            Some(options) => view! {
                                                {
                                                    options.iter().map(|(option, values)| {
                                                        view! {
                                                            <div class="row mb-2">
                                                                <div class="col fw-bold">
                                                                    {option}
                                                                </div>
                                                                <div class="col">
                                                                    {
                                                                        match values.len() {
                                                                            1 => view! { {values[0].clone()} }.into_view(),
                                                                            _ => view! {
                                                                                <select class="form-select productOption" name=option>
                                                                                {
                                                                                    values.iter().map(|value| {
                                                                                    view! {

                                                                                            <option value=value>
                                                                                                {value}
                                                                                            </option>

                                                                                    }
                                                                                    }).collect_view()
                                                                                }
                                                                                </select>
                                                                            }.into_view(),
                                                                        }
                                                                    }
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()
                                                }
                                            }.into_view(),
                                        }
                                    }
                                </div>
                                <div class="col-sm">
                                    <form on:submit=on_submit>
                                        <div class="row my-1">
                                            <div class="col fw-bold">
                                                "Артикул: "
                                            </div>
                                            <div class="col">
                                                {product_sku}
                                            </div>
                                        </div>
                                        <div class="row my-1">
                                            <div class="col fw-bold">
                                                "Цена: "
                                            </div>
                                            <div class="col">
                                                {product_price}"₽"
                                            </div>
                                        </div>
                                        <div class="row my-1">
                                            <div class="col">
                                                <textarea prop:value=product_comment node_ref=comment_element name="comment" class="form-control" rows="3" placeholder="Комментарий" maxlength="256">
                                                </textarea>
                                            </div>
                                        </div>
                                        <div class="row my-1">
                                            <div class="col">
                                                <div class="input-group">
                                                    <span class="input-group-text">Количество</span>
                                                    <input node_ref=quantity_element name="quantity" type="number" class="form-control productQuantity" min="0" step="1" aria-label="Количество" prop:value=product_quantity />
                                                    <span class="input-group-text">{product_measurement}</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="row justify-content-center my-1">
                                            <div class="col-auto">
                                                <button data-bs-dismiss="modal" type="submit" class="btn btn-primary">"Сохранить"</button>
                                            </div>
                                        </div>
                                        <div class="row">
                                            <strong>"Описание:"</strong>
                                            <p>{product_description}</p>
                                        </div>
                                    </form>
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
fn ProductPagination(#[prop(into)] products: Signal<Option<Products>>) -> impl IntoView {
    let mut query = use_query_map().get_untracked();
    query.remove("page");
    let mut query_string = query.to_query_string();
    if query_string.is_empty() {
        query_string.insert(0, '?');
    } else {
        query_string.push('&');
    }

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
                                    <a class="page-link" href=format!("{}page={}", query_string, page)>{page.to_string()}</a>
                                </li>
                            }.into_view(),
                            Some(page) => view! {
                                <li class="page-item">
                                    <a class="page-link" href=format!("{}page={}", query_string, page)>{page.to_string()}</a>
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

    let get_user = expect_context::<ReadSignal<User>>();
    let price_level = move || get_user().price_level;
    let discount = move || get_user().discount;

    view! {

        {
            move || match products.get() {
                None => view! { <div class="row"><div class="col"><div class="spinner-border" role="status"><span class="visually-hidden">Загрузка...</span></div></div></div> }.into_view(),
                Some(products) => match products.total {
                    0 => view! { <div class="row"><div class="col"><div class="alert alert-primary">"Ничего не найдено"</div></div></div> }.into_view(),
                    _ => view! {
                        <div class="row row-cols-1 row-cols-lg-4 row-cols-md-4 row-cols-sm-1">
                        <For each=products_products key=move |product| product.id children=move |product| view! {
                            <ProductCard product=product.clone() price_level=Signal::derive(price_level) discount=Signal::derive(discount) />
                        } />
                        </div>
                    }.into_view(),
                },
            }
        }

        <ProductPagination products=products />
    }
}

#[component]
fn ProductCard(
    product: Product,
    #[prop(into)] price_level: Signal<PriceLevel>,
    #[prop(into)] discount: Signal<f32>,
) -> impl IntoView {
    let set_product = expect_context::<WriteSignal<Option<Product>>>();
    let product_clone = product.clone();

    let product_image = product.get_image();
    let product_price = move || product_clone.get_price(&price_level(), discount());
    let product_name = product.name.clone();

    view! {
        <div class="col my-2">
            <div class="card text-center selectable overflow-hidden h-100" data-bs-toggle="modal" data-bs-target="#productModal" on:click=move |_| {set_product(Some(product.clone()))}>
                <img class="card-img-top" src=product_image alt="thumbnail" />
                <div class="card-body py-0">
                </div>
                <div class="card-footer bg-white border-top-0 text-start">
                    {product_name}
                    <h5 class="card-title text-start">{move || format!("{:.2}", product_price() )}"₽"</h5>
                </div>
            </div>
        </div>
    }
}
