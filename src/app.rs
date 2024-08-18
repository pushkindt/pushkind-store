use crate::env;
use crate::models::category::Category;
use crate::pages::category::CategoryPage;
use crate::pages::product::ProductPage;
use crate::pages::search::SearchPage;
use crate::utils::make_backend_url;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(get_category: ReadSignal<Option<Category>>) -> impl IntoView {
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
                <a class="navbar-brand" href=env::APP_BACKEND_URL>
                    <img class="logo" src=make_backend_url("/static/upload/logo1.png") alt="Logo" title="Nadin" />
                </a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false"
                    aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
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
                    <form class="d-flex w-100" role="search" action="/search">
                        <div class="input-group me-2">
                            <input name="search" class="form-control" type="search" placeholder="Search" aria-label="Search" />
                            <button class="btn btn-outline-secondary" type="submit"><i class="bi bi-search"></i></button>
                        </div>
                        <a class="text-muted ms-1" href=make_backend_url(env::APP_CART_URL)>
                            <i class="bi bi-cart fs-4"></i>
                        </a>
                    </form>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (get_category, set_category) = create_signal(None::<Category>);

    view! {
        <Router>
            <Navbar get_category=get_category />
            <Routes>
                <Route path="/" view=move || view! { <CategoryPage set_category=set_category /> }/>
                <Route path="/search" view=move || view! { <SearchPage set_category=set_category /> }/>
                <Route path="/category/:id" view=move || view! { <CategoryPage set_category=set_category /> } />
                <Route path="/product/:id" view=|| view! { <ProductPage /> } />
                <Route path="/*" view=|| view! { <h1>404 Not Found</h1> } />
                // <Route path="/cart" view=|| view! { <CartPage /> } />
            </Routes>
        </Router>
    }
}
