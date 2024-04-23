use std::ops::Not;

use leptos::{
    component, create_memo, ev::MouseEvent, view, Callable, Callback, IntoView, ReadSignal, Show,
    SignalGet, SignalGetUntracked,
};

use crate::icons::spin::Spin;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Primary,
    #[default]
    Secondary,
}

#[component]
pub fn Button(
    #[prop(default = "label")] label: &'static str,
    #[prop(default = None)] busy_label: Option<&'static str>,
    #[prop(default = ButtonStyle::Secondary)] style: ButtonStyle,
    #[prop(into)] pressed: Callback<MouseEvent>,
    #[prop(default = None)] busy: Option<ReadSignal<bool>>,
) -> impl IntoView {
    let is_busy_memo = create_memo(move |_| match busy {
        Some(busy_signal) => busy_signal.get(),
        None => false,
    });

    // handle click events only if not busy.
    let on_click = move |ev| {
        if is_busy_memo.get_untracked().not() {
            pressed.call(ev);
        }
    };

    // button specific properties by style
    let class_props_by_style = match style {
        ButtonStyle::Primary => String::from("bg-indigo-700 text-white hover:bg-indigo-600"),
        ButtonStyle::Secondary => String::from(
            "bg-white border border-slate-200 text-slate-900 hover:bg-slate-100 disabled:bg-slate-200",
        ),
    };

    view! {
        <button
            class="lumx-button flex justify-center items-center gap-x-1 min-w-40 rounded-md px-3 py-1.5 text-sm disabled:opacity-85 disabled:cursor-wait"
            class=class_props_by_style
            prop:disabled=move || is_busy_memo.get()
            on:click=on_click>
            <Show
                when=move || is_busy_memo.get()
                fallback=move || view! { {label} }>
                <Spin class="animate-spin h-5 w-5" />
                {busy_label.unwrap_or(label)}
            </Show>
        </button>
    }
}
