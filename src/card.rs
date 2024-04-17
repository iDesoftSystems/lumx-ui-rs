use leptos::{component, view, Children, IntoView};

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card-outer rounded-md border border-slate-200 bg-white py-4 sm:py-6 px-4 sm:px-6">
        {children()}
        </div>
    }
}
