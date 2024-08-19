use crate::models::product::{Product, Products};
use crate::utils::Paginator;
use leptos::*;

#[component]
pub fn ProductModal(get_product: ReadSignal<Option<Product>>) -> impl IntoView {
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
                                    <div class="row">
                                        <div class="col">
                                            <textarea class="form-control productText" rows="3" placeholder="Комментарий" id="descriptionModalProductText">
                                            </textarea>
                                        </div>
                                    </div>
                                    <div class="row my-1">
                                        <div class="col">
                                            <div class="input-group">
                                                <span class="input-group-text">Количество</span>
                                                <input name="quantity" type="number" class="form-control productQuantity" min="0" step="1" aria-label="Количество" value="" />
                                                <span class="input-group-text">{product_measurement}</span>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="row justify-content-center my-1">
                                        <div class="col-auto">
                                            <button type="button" class="btn btn-primary">"Сохранить"</button>
                                        </div>
                                    </div>
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
pub fn ProductCards(
    #[prop(into)] products: Signal<Option<Products>>,
    set_product: WriteSignal<Option<Product>>,
) -> impl IntoView {
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
                                <ProductCard product=product.clone() set_product=set_product />
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
fn ProductCard(product: Product, set_product: WriteSignal<Option<Product>>) -> impl IntoView {
    let product_id = product.id;
    let product_image = product.get_image();
    let product_price = product.price;
    let product_name = product.name.clone();

    view! {
        <div class="col my-2">
            <div class="card text-center selectable overflow-hidden h-100" data-id=format!("{}", product_id) data-bs-toggle="modal" data-bs-target="#productModal" on:click=move |_| {set_product(Some(product.clone()))}>
                <img class="card-img-top" src=product_image alt="thumbnail" />
                <div class="card-body py-0">
                    <h5 class="card-title text-start">{format!("{:.2}", product_price)} "₽"</h5>
                </div>
                <div class="card-footer bg-white border-top-0 text-start">
                    {product_name}
                </div>
            </div>
        </div>
    }
}
