use crate::env;
use crate::models::category::Category;
use crate::models::shopping_cart::ShoppingCart;
use crate::models::tag::load_tags;
use crate::utils::make_backend_url;
use leptos::*;
use leptos_oidc::{Auth, Authenticated, LoginLink};

#[component]
pub fn Navbar() -> impl IntoView {
    let get_category =
        use_context::<ReadSignal<Option<Category>>>().expect("Get category signal not found");

    let category_id = move || match get_category() {
        None => 0,
        Some(category) => category.id,
    };

    let category_children = move || match get_category() {
        None => vec![],
        Some(category) => category.children,
    };

    let category_name = move || match get_category() {
        None => "".to_string(),
        Some(category) => category.name,
    };

    let get_cart =
        use_context::<Signal<ShoppingCart>>().expect("Get shopping cart signal not found");

    let cart_count = move || get_cart().items.len();

    let tags = create_resource(|| (), |_| async move { load_tags().await });
    let tags = move || match tags.get() {
        None => vec![],
        Some(tags) => match tags {
            None => vec![],
            Some(tags) => tags,
        },
    };

    view! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary px-5">
            <div class="container-fluid">
                <a class="navbar-brand" href=env::APP_BACKEND_URL>
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
                        <a class="text-muted ms-1" href="#" data-bs-toggle="modal" data-bs-target="#cartModal">
                            <i class="bi bi-cart fs-4"></i>
                            <span class="position-absolute translate-middle badge rounded-pill bg-danger">
                                {cart_count}
                                <span class="visually-hidden">items in cart</span>
                            </span>
                        </a>
                        <Authenticated unauthenticated=move || {
                            view! {
                                <LoginLink class="text-muted ms-3"><i class="bi bi-box-arrow-right fs-4"></i></LoginLink>
                            }
                        }>
                            <a class="text-muted ms-3" href=env::APP_BACKEND_URL>
                                <i class="bi bi-person-circle fs-4"></i>
                            </a>
                        </Authenticated>
                    </form>
                </div>
            </div>
        </nav>
        <div class="row justify-content-center">
            <div class="col-auto">
                {move || {
                    tags().into_iter().map(|tag| {
                        view! {
                            <a class="badge text-bg-secondary me-1 text-decoration-none" href={format!("/category/{}/tag/{}", category_id(), tag)}>{tag}</a>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
