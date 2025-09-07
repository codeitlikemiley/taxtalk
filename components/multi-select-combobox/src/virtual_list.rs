use leptos::prelude::*;
use leptos::html::Div;
use crate::types::VirtualListState;

#[component]
pub fn VirtualList<T, V, F>(
    items: Signal<Vec<T>>,
    item_height: f64,
    container_height: f64,
    render_item: F,
) -> impl IntoView
where
    T: Clone + Send + Sync + PartialEq + 'static,
    V: IntoView + 'static,
    F: Fn(usize, T) -> V + 'static + Clone + Send,
{
    let container_ref = NodeRef::<Div>::new();
    let (virtual_state, set_virtual_state) = signal(VirtualListState::new(item_height, container_height));
    
    // Update virtual state when items change
    Effect::new(move |_| {
        let item_count = items.get().len();
        set_virtual_state.update(|state| {
            state.update_items(item_count);
        });
    });
    
    // Handle scroll events
    let handle_scroll = move |_| {
        if let Some(element) = container_ref.get() {
            let scroll_top = element.scroll_top() as f64;
            set_virtual_state.update(|state| {
                state.update_scroll(scroll_top);
            });
        }
    };
    
    // Handle resize
    Effect::new(move |_| {
        if let Some(element) = container_ref.get() {
            let rect = element.get_bounding_client_rect();
            let height = rect.height();
            set_virtual_state.update(|state| {
                state.update_container_height(height);
            });
        }
    });
    
    // Get visible items
    let visible_items = Memo::new(move |_| {
        let all_items = items.get();
        let state = virtual_state.get();
        
        all_items
            .into_iter()
            .enumerate()
            .skip(state.visible_start)
            .take(state.visible_end - state.visible_start)
            .collect::<Vec<_>>()
    });
    
    view! {
        <div
            node_ref=container_ref
            class="msc-virtual-list"
            style=move || format!("height: {}px; overflow-y: auto;", container_height)
            on:scroll=handle_scroll
        >
            <div
                class="msc-virtual-spacer"
                style=move || format!("height: {}px; position: relative;", virtual_state.get().total_height())
            >
                <div
                    class="msc-virtual-content"
                    style=move || format!("transform: translateY({}px);", virtual_state.get().offset_y())
                >
                    <For
                        each=move || visible_items.get()
                        key=|(idx, _)| *idx
                        children=move |(idx, item)| {
                            view! {
                                <div
                                    class="msc-virtual-item"
                                    style=move || format!("height: {}px;", item_height)
                                >
                                    {render_item(idx, item)}
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}