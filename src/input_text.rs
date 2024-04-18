use leptos::{component, view, IntoView};
use uuid::Uuid;

#[component]
pub fn InputText(
    #[prop(default = "label")] label: &'static str,
    #[prop(default = "")] placeholder: &'static str,
) -> impl IntoView {
    let form_id = Uuid::new_v4().to_string();

    view! {
        <div class="lumx-input-text flex flex-col rounded-md border box-border border-slate-200 p-2.5 bg-white mb-2.5 focus-within:border focus-within:border-blue-600">
            <label for=form_id.clone() class="text-xs text-slate-900">{label}</label>
            <input class="outline-none text-sm bg-white text-slate-900" placeholder=placeholder id=form_id type="text" value=""/>
        </div>
    }
}
