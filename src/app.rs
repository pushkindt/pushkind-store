use crate::models::category::Category;
use crate::models::product::Product;
use crate::models::shopping_cart::ShoppingCart;
use crate::pages::category::CategoryPage;
use crate::pages::navbar::Navbar;
use crate::pages::products::ProductModal;
use crate::pages::search::SearchPage;
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::*;
use leptos_use::storage::use_session_storage;

#[component]
pub fn App() -> impl IntoView {
    let (get_category, set_category) = create_signal(None::<Category>);
    let (get_product, set_product) = create_signal(None::<Product>);

    let (get_cart, set_cart, reset_cart) =
        use_session_storage::<ShoppingCart, JsonSerdeCodec>("cart");

    provide_context(get_cart);

    view! {
        <Router>
            <Navbar get_category=get_category />
            <ProductModal get_product=get_product/>
            <Routes>
                <Route path="/" view=move || view! { <CategoryPage set_category=set_category set_product=set_product /> }/>
                <Route path="/search" view=move || view! { <SearchPage set_product=set_product/> }/>
                <Route path="/category/:id" view=move || view! { <CategoryPage set_category=set_category set_product=set_product /> } />
                <Route path="/*" view=|| view! { <h1>404 Not Found</h1> } />
                // <Route path="/cart" view=|| view! { <CartPage /> } />
            </Routes>
        </Router>
    }
}
