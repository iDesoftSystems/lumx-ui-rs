use leptos::{component, event_target_value, view, IntoView, RwSignal, SignalSet};
use uuid::Uuid;

#[component]
pub fn InputText(
    #[prop(default = "label")] label: &'static str,
    #[prop(default = "")] placeholder: &'static str,
    value: RwSignal<String>,
) -> impl IntoView {
    let form_id = Uuid::new_v4().to_string();

    // notify the new value when the element loses focus
    let on_blur = move |ev| {
        value.set(event_target_value(&ev));
    };

    view! {
        <div class="lumx-input-text flex flex-col rounded-md border box-border border-slate-200 p-2.5 bg-white mb-2.5 focus-within:border focus-within:border-blue-600">
            <label for=form_id.clone() class="text-xs text-slate-900">{label}</label>
            <input
                class="outline-none text-sm bg-white text-slate-900 placeholder:text-sm placeholder:text-slate-400"
                type="text"
                placeholder=placeholder
                id=form_id
                on:blur=on_blur
                prop:value=value/>
        </div>
    }
}
