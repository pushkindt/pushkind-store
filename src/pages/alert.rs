use crate::models::alert::AlertMessage;
use leptos::*;
use std::time::Duration;

#[component]
pub fn Alert() -> impl IntoView {
    let (alert, set_alert) = create_signal::<AlertMessage>(AlertMessage::default());
    provide_context::<WriteSignal<AlertMessage>>(set_alert);

    let base_alert_classes = "alert";

    let alert_classes = move || -> String {
        let t = alert.get();
        let background_class = format! {"{}", t.alert_type};

        let opacity_class = if t.visible {
            "d-block".to_string()
        } else {
            "d-none".to_string()
        };

        format!(
            "{} {} {}",
            base_alert_classes, background_class, opacity_class
        )
    };

    create_effect(move |_| {
        let t = alert.get();
        if t.visible {
            set_timeout(
                move || {
                    set_alert.update(|msg| {
                        msg.visible = false;
                    });
                },
                Duration::new(4, 0),
            )
        }
    });

    view! {
        <div id="alert" class={alert_classes} role="alert">
            {move || alert.get().message}
        </div>
    }
}
