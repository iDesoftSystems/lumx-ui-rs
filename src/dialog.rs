use leptos::{component, html::Dialog, view, Children, IntoView, NodeRef};

#[component]
pub fn Dialog(children: Children, dialog_ref: NodeRef<Dialog>) -> impl IntoView {
    view! {
        <dialog
            node_ref=dialog_ref
            class="lumx-dialog bg-white rounded-md">
        {children()}
        </dialog>
    }
}

#[component]
pub fn DialogContent(children: Children) -> impl IntoView {
    view! {
        <div class="px-4 pb-4 pt-5">
            {children()}
        </div>
    }
}

#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-row flex-wrap justify-end bg-gray-50 gap-x-2 gap-y-2 px-4 py-3">
            {children()}
        </div>
    }
}
