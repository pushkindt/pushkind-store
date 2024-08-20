use crate::models::product::{Product, Products};
use crate::models::shopping_cart::{CartItem, ShoppingCart};
use crate::utils::Paginator;
use leptos::*;

#[component]
pub fn ProductModal() -> impl IntoView {
    let get_product =
        use_context::<ReadSignal<Option<Product>>>().expect("Get product signal not found");

    let get_cart =
        use_context::<Signal<ShoppingCart>>().expect("Get shopping cart signal not found");

    let set_cart =
        use_context::<WriteSignal<ShoppingCart>>().expect("Set shopping cart signal not found");

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
        Some(product) => format!("{:.2}", product.price),
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
                    quantity: quantity,
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
                                    <div id="carouselImages" class="carousel slide">
                                        <div class="carousel-inner">
                                            <div class="carousel-item active">
                                                <img src=product_image class="d-block w-100" alt="Product Image" />
                                            </div>
                                            <div class="carousel-item">
                                                <img src=product_image class="d-block w-100" alt="Product Image" />
                                            </div>
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
                                </div>
                                <div class="col-sm">
                                    <form on:submit=on_submit>
                                        <div class="row my-1">
                                            <div class="col-3 fw-bold">
                                                "Артикул: "
                                            </div>
                                            <div class="col">
                                                {product_sku}
                                            </div>
                                        </div>
                                        <div class="row my-1">
                                            <div class="col-3 fw-bold">
                                                "Цена: "
                                            </div>
                                            <div class="col">
                                                {product_price}"₽"
                                            </div>
                                        </div>
                                        <div class="row my-1">
                                            <div class="col">
                                                <textarea prop:value=product_comment node_ref=comment_element name="comment" class="form-control" rows="3" placeholder="Комментарий">
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
                                    </form>
                                </div>
                            </div>
                        </div>
                        <div class="row">
                            <strong>"Описание:"</strong>
                            <pre>{product_description}</pre>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

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
    let set_product =
        use_context::<WriteSignal<Option<Product>>>().expect("Set product signal not found");

    let product_image = product.get_image();
    let product_price = product.price;
    let product_name = product.name.clone();

    view! {
        <div class="col my-2">
            <div class="card text-center selectable overflow-hidden h-100" data-bs-toggle="modal" data-bs-target="#productModal" on:click=move |_| {set_product(Some(product.clone()))}>
                <img class="card-img-top" src=product_image alt="thumbnail" />
                <div class="card-body py-0">
                    <h5 class="card-title text-start">{format!("{:.2}", product_price)}"₽"</h5>
                </div>
                <div class="card-footer bg-white border-top-0 text-start">
                    {product_name}
                </div>
            </div>
        </div>
    }
}
