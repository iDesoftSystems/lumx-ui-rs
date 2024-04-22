use leptos::{
    component, create_node_ref, create_signal, ev::MouseEvent, event_target_value, html::Div, view,
    Callable, Callback, For, IntoView, Resource, RwSignal, Show, SignalGet, SignalGetUntracked,
    SignalSet, Suspense, WriteSignal,
};
use leptos_use::{is_some, on_click_outside, watch_debounced};

use crate::icons::x_mark::XMark;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeAheadOption {
    pub value: String,
    pub label: String,
}

impl TypeAheadOption {
    pub fn new(value: String, label: String) -> Self {
        Self { value, label }
    }
}

#[component]
pub fn TypeAhead(
    #[prop(default = "label")] label: &'static str,
    #[prop(default = "")] placeholder: &'static str,
    #[prop(default = 800.0)] debounced_ms: f64,
    suggestions: Resource<String, Vec<TypeAheadOption>>,
    changes: WriteSignal<String>,
    value: RwSignal<Option<TypeAheadOption>>,
) -> impl IntoView {
    let type_ahead_ref = create_node_ref::<Div>();

    let (show_panel_reader, show_panel_writer) = create_signal(false);

    // handler to detect clicks outside then element and close open panels.
    let _ = on_click_outside(type_ahead_ref, move |_| {
        let is_panel_open = show_panel_reader.get_untracked();
        if is_panel_open {
            show_panel_writer.set(false);
        }
    });

    // expand suggestions panel on click
    let on_panel_click = move |_ev: MouseEvent| {
        show_panel_writer.set(true);
    };

    // select a new option as value
    let on_select_item = move |option: TypeAheadOption| {
        value.set(Some(option));

        show_panel_writer.set(false);
    };

    // clear current value
    let on_clear_item = move |_ev: MouseEvent| {
        value.set(None);
    };

    view! {
        <div node_ref=type_ahead_ref
            on:click=on_panel_click
            class="lumx-type-ahead rounded-md border box-border border-slate-200 bg-white mb-2.5 focus-within:border focus-within:border-blue-600">

            <label class="lumx-type-ahead-relative relative">

                <div class="type-ahead-control flex flex-col p-2.5">
                    <div class="type-ahead-label text-xs text-slate-900">
                        {label}
                    </div>

                    <Show
                        when=move || is_some(value).get()
                        fallback=move || view! {
                            <div class="type-ahead-placeholder text-sm text-slate-400 min-h-5">
                                {placeholder}
                            </div>
                         }>
                        <div class="type-ahead-value flex justify-between flex-row gap-y-1 min-h-5">
                            <div class="text-sm text-slate-900">
                                {move || value.get().unwrap().label}
                            </div>

                            <div on:click=on_clear_item class="type-ahead-clear cursor-pointer rounded-full hover:bg-slate-200">
                                <XMark class="w-5 h-5 text-slate-900" />
                            </div>
                        </div>
                    </Show>
                </div>

                <Show when=move || show_panel_reader.get()>
                    <TypeAheadSearchPanel
                        debounced_ms=debounced_ms
                        on_select=on_select_item
                        changes=changes
                        suggestions=suggestions />
                </Show>
            </label>
        </div>
    }
}

#[component]
fn TypeAheadSearchPanel(
    #[prop(default = 800.0)] debounced_ms: f64,
    #[prop(into)] on_select: Callback<TypeAheadOption>,
    changes: WriteSignal<String>,
    suggestions: Resource<String, Vec<TypeAheadOption>>,
) -> impl IntoView {
    let (input_changes, input_changes_writer) = create_signal("".to_string());

    let on_input = move |ev| {
        input_changes_writer.set(event_target_value(&ev));
    };

    let _ = watch_debounced(
        move || input_changes.get(),
        move |input_value, _, _| {
            changes.set(input_value.to_string());
        },
        debounced_ms,
    );

    view! {
        <div class="type-ahead-overlay-panel w-full absolute mt-1 z-10 rounded-md border box-border border-slate-200 bg-white">
            <div class="py-1 px-2 flex flex-col">
                <input class="outline-none py-1 px-2 text-sm bg-white text-slate-900 rounded-md border border-slate-200"
                    type="text"
                    on:input=on_input />
            </div>

            <ul class="type-ahead-suggestions"
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="menu-button"
                tabindex="-1">
                <Suspense>
                    <For each=move || suggestions.get().unwrap()
                        key=|i| i.value.clone()
                        let:item>
                        <TypeAheadOptionView
                            item=item
                            on_select=on_select />
                    </For>
                </Suspense>
            </ul>

        </div>
    }
}

#[component]
fn TypeAheadOptionView(
    item: TypeAheadOption,
    #[prop(into)] on_select: Callback<TypeAheadOption>,
) -> impl IntoView {
    let item_presenter = item.clone();

    let on_click = move |ev: MouseEvent| {
        ev.prevent_default();

        on_select.call(item.clone());
    };

    view! {
        <li class="type-ahead-option text-sm text-slate-900 py-2 px-2 hover:bg-slate-200 cursor-pointer"
            on:click=on_click>
            <span>{item_presenter.label}</span>
        </li>
    }
}
