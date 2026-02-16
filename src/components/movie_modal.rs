use crate::model::video::Video;
use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn MovieModal(
    video: ReadSignal<Option<Video>>,
    set_video: WriteSignal<Option<Video>>,
) -> impl IntoView {
    let set_video_overlay = set_video;
    let set_video_btn = set_video;

    view! {
        <Stylesheet href="movie_modal.css"/>

        // Portal is ALWAYS in the DOM — we toggle visibility via CSS.
        // This prevents Leptos from destroying/recreating the Portal and losing event handlers.
        <leptos::portal::Portal>
            <div
                class="modal-overlay"
                style=move || {
                    let v = video.get();
                    leptos::logging::log!("MODAL: render style, video is some? {}", v.is_some());
                    if v.is_some() {
                        "z-index: 1000; display: flex !important;".to_string()
                    } else {
                        "display: none !important;".to_string()
                    }
                }
                on:click=move |_| {
                    leptos::logging::log!("MODAL: Overlay clicked -> closing");
                    set_video_overlay.set(None);
                }
            >
                // Modal content — updates reactively when video changes
                <div class="modal-content" on:click=|e| {
                    e.stop_propagation();
                }>
                    {move || {
                        video.get().map(|v| {
                            view! {
                                // Movie poster section
                                <div class="modal-poster-section">
                                    <h3>"Movie poster"</h3>
                                    <img
                                        src={v.thumbnail_url.clone()}
                                        alt={format!("Poster for {}", v.title)}
                                        class="modal-poster"
                                    />
                                </div>

                                // Description section
                                <div class="modal-description-section">
                                    <h3>"description and other data or text"</h3>
                                    <div class="modal-info-grid">
                                        <div class="modal-info-item">
                                            <strong>"Title:"</strong>
                                            <span>{v.title.clone()}</span>
                                        </div>
                                        <div class="modal-info-item">
                                            <strong>"Channel:"</strong>
                                            <span>{v.channel_name.clone()}</span>
                                        </div>
                                        <div class="modal-info-item">
                                            <strong>"Rating:"</strong>
                                            <span>{format!("{:.1}/10", v.rating)}</span>
                                        </div>
                                        <div class="modal-info-item">
                                            <strong>"Genres:"</strong>
                                            <span>{v.genres.join(", ")}</span>
                                        </div>
                                    </div>
                                    <div class="modal-description">
                                        <strong>"Description:"</strong>
                                        <p>{v.description.clone()}</p>
                                    </div>
                                </div>
                            }
                        })
                    }}
                </div>

                // Close button — always in the DOM, always has its event handler

                <button
                    type="button"
                    class="modal-close-btn"
                    style="z-index: 1005; position: absolute; top: 20px; right: 20px; cursor: pointer;"
                    on:click=move |e| {
                        leptos::logging::log!("MODAL: Close button clicked -> closing");
                        e.prevent_default();
                        // Allow propagation to overlay as a fallback! if button logic fails, overlay click will catch it.
                        set_video_btn.set(None);
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        style="pointer-events: none;"
                    >
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>
        </leptos::portal::Portal>
    }
}
