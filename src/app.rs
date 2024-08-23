use crate::models::product::Product;
use crate::models::shopping_cart::ShoppingCart;
use crate::pages::cart::CartModal;
use crate::pages::category::CategoryPage;
use crate::pages::navbar::Navbar;
use crate::pages::products::ProductModal;
use crate::pages::search::SearchPage;
use crate::utils::make_backend_url;
use crate::{env, models::category::Category};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_oidc::{Auth, AuthParameters, Challenge};
use leptos_router::*;
use leptos_use::storage::use_session_storage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AppWithRouter/>
        </Router>
    }
}

#[component]
pub fn AppWithRouter() -> impl IntoView {
    let (get_category, set_category) = create_signal(None::<Category>);
    let (get_product, set_product) = create_signal(None::<Product>);
    let (get_cart, set_cart, _) = use_session_storage::<ShoppingCart, JsonSerdeCodec>("cart");

    provide_context(get_category);
    provide_context(set_category);
    provide_context(get_product);
    provide_context(set_product);
    provide_context(get_cart);
    provide_context(set_cart);

    let auth_parameters = AuthParameters {
        issuer: make_backend_url(env::APP_SIGNIN_URL),
        client_id: env::APP_SIGNIN_CLIENT.to_string(),
        redirect_uri: env::APP_SIGNIN_REDIRECT_URL.to_string(),
        post_logout_redirect_uri: env::APP_SIGNIN_REDIRECT_URL.to_string(),
        challenge: Challenge::S256,
        scope: Some("openid%20profile%20email".to_string()),
        audience: None,
        prompt: None,
    };
    let auth = Auth::init(auth_parameters);

    provide_context(auth);

    view! {
        <Router>
            <Navbar />
            <ProductModal />
            <CartModal />
            <Routes>
                <Route path="/" view=move || view! { <CategoryPage /> }/>
                <Route path="/search" view=move || view! { <SearchPage /> }/>
                <Route path="/category/:id" view=move || view! { <CategoryPage /> } />
                <Route path="/category/:id/tag/:tag" view=move || view! { <CategoryPage /> } />
                <Route path="/*" view=|| view! { <h1>404 Not Found</h1> } />
                // <Route path="/cart" view=|| view! { <CartPage /> } />
            </Routes>
        </Router>
    }
}
