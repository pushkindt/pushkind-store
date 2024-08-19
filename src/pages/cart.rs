use crate::env;
use crate::utils::make_backend_url;
use leptos::*;

#[component]
pub fn CartModal() -> impl IntoView {
    view! {
        <div class="modal fade" id="cartModal" tabindex="-1" aria-labelledby="productModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="productModalLabel">"Корзина"</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row">
                                <div class="col">
                                    <div class="alert alert-danger">TODO!</div>
                                </div>
                                <div class="col">
                                    <form method="POST" action=make_backend_url(env::APP_CART_URL)>
                                        <button type="submit" class="btn btn-primary">
                                            "Заказать"
                                        </button>
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
