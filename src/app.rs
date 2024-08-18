use crate::env;
use crate::models::category::Category;
use crate::models::product::Product;
use crate::pages::category::CategoryPage;
use crate::pages::product::ProductPage;
use crate::pages::search::SearchPage;
use crate::utils::make_backend_url;
use leptos::*;
use leptos_router::*;

#[component]
fn Navbar(get_category: ReadSignal<Option<Category>>) -> impl IntoView {
    let category_children = move || match get_category() {
        None => vec![],
        Some(category) => category.children,
    };

    let category_name = move || match get_category() {
        None => "".to_string(),
        Some(category) => category.name,
    };

    view! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary px-5">
            <div class="container-fluid">
                <a class="navbar-brand" href="#" data-bs-toggle="modal" data-bs-target="#productModal">//href=env::APP_BACKEND_URL>
                    <img class="logo" src=make_backend_url("/static/upload/logo1.png") alt="Logo" title="Nadin" />
                </a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false"
                    aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    {
                        move || match get_category() {
                            Some(_) => view! {
                                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                    <li class="nav-item dropdown">
                                        <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown"
                                            aria-expanded="false">
                                            Категории
                                        </a>
                                        <ul class="dropdown-menu">
                                            <li><a class="dropdown-item disabled" aria-disabled="true">{category_name}</a></li>
                                            <li><hr class="dropdown-divider" /></li>
                                            <For each=category_children key=|child| child.0 children=move |child| view! {
                                                <li><a class="dropdown-item" href=format!("/category/{}", child.0)>{child.1.clone()}</a></li>
                                            } />
                                        </ul>
                                    </li>
                                </ul>
                            }.into_view(),
                            None => view! {
                                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                    <li class="nav-item">
                                        <a class="nav-link" aria-current="page" href="/">Главная</a>
                                    </li>
                                </ul>
                            }.into_view()
                        }
                    }
                    <form class="d-flex w-100" role="search" action="/search">
                        <div class="input-group me-2">
                            <input name="q" class="form-control" type="search" placeholder="Поиск" aria-label="Search" />
                            <button class="btn btn-outline-secondary" type="submit"><i class="bi bi-search"></i></button>
                        </div>
                        <a class="text-muted ms-1" href=make_backend_url(env::APP_CART_URL)>
                            <i class="bi bi-cart fs-4"></i>
                            <span class="position-absolute bottom-0 start-10 translate-middle badge rounded-pill bg-danger">
                                0
                                <span class="visually-hidden">items in cart</span>
                            </span>
                        </a>
                    </form>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn ProductModal(get_product: ReadSignal<Option<Product>>) -> impl IntoView {
    view! {
        <div class="modal fade" id="productModal" tabindex="-1" aria-labelledby="productModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="productModalLabel">"Andersen-Барх кофе в зерне в какао-обсып'Ван. кор. кар' 1кг"</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row">
                                <div class="col-sm">
                                    <div id="carouselImages" class="carousel slide">
                                        <div class="carousel-inner">
                                            <div class="carousel-item active">
                                                <img src="https://reports.pushkind.com//static/upload/vendor1/1%D0%9A%D0%9C%D0%94%D0%9E%D0%9100-000001-01.jpg" class="d-block w-100" alt="Product Image" />
                                            </div>
                                            <div class="carousel-item">
                                                <img src="https://reports.pushkind.com//static/upload/vendor1/1%D0%9A%D0%9C%D0%94%D0%9E%D0%9100-000001-01.jpg" class="d-block w-100" alt="Product Image" />
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
                                            <textarea class="form-control productText"
                                                rows="3"
                                                placeholder="Комментарий"
                                                id="descriptionModalProductText">
                                            </textarea>
                                        </div>
                                    </div>
                                    <div class="row my-1">
                                        <div class="col">
                                            <div class="input-group">
                                                <span class="input-group-text">Количество</span>
                                                <input type="number" class="form-control productQuantity" min="0" step="1" aria-label="Количество" value="" />
                                                <span class="input-group-text">"шт"</span>
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
                            <pre>"qwer"</pre>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (get_category, set_category) = create_signal(None::<Category>);

    let (get_product, set_product) = create_signal(None::<Product>);

    view! {
        <Router>
            <Navbar get_category=get_category />
            <ProductModal get_product=get_product/>
            <Routes>
                <Route path="/" view=move || view! { <CategoryPage set_category=set_category set_product=set_product /> }/>
                <Route path="/search" view=move || view! { <SearchPage /> }/>
                <Route path="/category/:id" view=move || view! { <CategoryPage set_category=set_category set_product=set_product /> } />
                <Route path="/*" view=|| view! { <h1>404 Not Found</h1> } />
                // <Route path="/cart" view=|| view! { <CartPage /> } />
            </Routes>
        </Router>
    }
}
