use crate::models::category::Category;
use crate::models::product::Product;
use crate::pages::category::CategoryPage;
use crate::pages::navbar::Navbar;
use crate::pages::products::ProductModal;
use crate::pages::search::SearchPage;
use leptos::*;
use leptos_router::*;

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
                <Route path="/search" view=move || view! { <SearchPage set_product=set_product/> }/>
                <Route path="/category/:id" view=move || view! { <CategoryPage set_category=set_category set_product=set_product /> } />
                <Route path="/*" view=|| view! { <h1>404 Not Found</h1> } />
                // <Route path="/cart" view=|| view! { <CartPage /> } />
            </Routes>
        </Router>
    }
}
