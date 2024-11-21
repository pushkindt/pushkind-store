use crate::env;
use crate::utils::make_backend_url;
use leptos::*;

#[component]
pub fn ContactsModal() -> impl IntoView {
    view! {
        <div class="modal fade" id="contactsModal" tabindex="-1" aria-labelledby="contactsModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content">
                    <div class="modal-header">
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body bg-light">
                    <iframe class="w-100 mt-3" style="height: 80vh;" src=make_backend_url(&format!("{}{}", env::APP_CONTACTS_URL, env::APP_SIGNIN_CLIENT))></iframe>
                    </div>
                </div>
            </div>
        </div>
    }
}
