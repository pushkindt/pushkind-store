use crate::env;
use crate::models::cart::ShoppingCart;
use crate::models::category::Category;
use crate::models::tag::load_tags;
use crate::utils::make_backend_url;
use leptos::*;
use leptos_oidc::{Authenticated, LoginLink, LogoutLink};

#[component]
pub fn Navbar() -> impl IntoView {
    let get_category = expect_context::<ReadSignal<Option<Category>>>();

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

    let get_cart = expect_context::<Signal<ShoppingCart>>();

    let cart_count = move || get_cart().items.len();

    let access_token = expect_context::<ReadSignal<Option<String>>>();

    let tags = create_resource(access_token, move |access_token| async move {
        load_tags(&access_token).await
    });
    let tags = move || match tags.get() {
        None => vec![],
        Some(tags) => tags.unwrap_or_default(),
    };

    view! {
        <div class="container">
            <nav class="navbar navbar-expand-sm bg-body-tertiary">
                <div class="container-fluid">
                    <a class="navbar-brand me-0" href=env::APP_BACKEND_URL>
                        <img class="logo" src=make_backend_url(env::APP_LOGO_URL) alt="Logo" title="Nadin" />
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
                                    <ul class="navbar-nav me-auto">
                                        <li class="nav-item dropdown">
                                            <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown"
                                                aria-expanded="false">
                                                "Категории"
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
                                    <ul class="navbar-nav me-auto">
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
                        </form>
                    </div>
                    <a class="text-muted nav-link" href="#" data-bs-toggle="modal" data-bs-target="#cartModal">
                        <i class="bi bi-cart fs-4"></i>
                        <span class="position-absolute translate-middle badge rounded-pill bg-danger">
                            {move || match cart_count () {
                                count if count > 9 => "9+".to_string(),
                                count => format!("{}", count),
                            }}
                            <span class="visually-hidden">items in cart</span>
                        </span>
                    </a>
                    <a class="nav-link text-muted ms-3" href="#" data-bs-toggle="modal" data-bs-target="#contactsModal" title="Контакты">
                        <i class="bi bi-telephone fs-4"></i>
                    </a>
                    <Authenticated unauthenticated=move || {
                        view! {
                            <LoginLink class="nav-link text-muted ms-3">"Войти"</LoginLink>
                        }
                    }>
                        <div class="dropdown-center">
                            <button class="btn btn-link nav-link align-items-center ms-3 text-muted dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
                                <i class="bi bi-person-circle fs-4"></i>
                            </button>
                            <ul class="dropdown-menu dropdown-menu-end">
                                <li>
                                    <a class="dropdown-item icon-link" href=make_backend_url(env::APP_PROFILE_URL)>
                                        <i class="bi bi-person mb-2"></i>
                                        "Профиль"
                                    </a>
                                </li>
                                <li>
                                    <LogoutLink class="dropdown-item icon-link">
                                        <i class="bi bi-box-arrow-right mb-2"></i>
                                        "Выйти"
                                    </LogoutLink>
                                </li>
                            </ul>
                        </div>
                    </Authenticated>
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
        </div>
    }
}
