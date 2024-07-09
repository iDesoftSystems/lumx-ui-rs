use leptos::{component, view, Children, IntoView};

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="lumx-card rounded-md border border-slate-200 bg-white">
        {children()}
        </div>
    }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
    view! {
        <div class="lumx-card-content py-4 sm:py-6 px-4 sm:px-6">
        {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
    view! {
        <div class="lumx-card-footer rounded-b-md flex flex-row flex-wrap justify-end gap-x-2 gap-y-2 px-4 py-3 bg-gray-50">
        {children()}
        </div>
    }
}
