use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Divider(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] vertical: MaybeSignal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("divider", include_str!("./divider.css"));

    view! {
        <div
            class=class_list!["thaw-divider", ("thaw-divider--vertical", move || vertical.get()), class.map(| c | move || c.get())]
            // aria-orientation=move || if vertical.get() { "vertical" } else { "horizontal" }
            role="separator"
        >
            <OptionComp value=children let:children>
                <div class="thaw-divider__wrapper">
                    {children()}
                </div>
            </OptionComp>
        </div>
    }
}
