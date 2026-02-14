use leptos::prelude::*;
use leptos_meta::Stylesheet;
use crate::model::video::Video;

#[component]
pub fn MovieModal<F>(
    video: ReadSignal<Option<Video>>,
    on_close: F,
) -> impl IntoView 
where F: Fn() + Clone + 'static + Send + Sync
{
    // Fix deprecation warning: use StoredValue::new
    let on_close = StoredValue::new(on_close);

    view! {
        <Stylesheet href="movie_modal.css"/>
        
        {move || match video.get() {
            Some(v) => {
                leptos::logging::log!("RENDERING MODAL for: {}", v.title);
                view! {
                    <leptos::portal::Portal>
                        <div class="modal-overlay" style="z-index: 99999;" on:click=move |_| on_close.with_value(|f| f())>
                            <div class="modal-content" on:click=|e| e.stop_propagation()>
                                // Close button
                                <button class="modal-close-btn" on:click=move |_| on_close.with_value(|f| f())>
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                                
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
                            </div>
                        </div>
                    </leptos::portal::Portal>
                }.into_any()
            },
            None => view! { <div></div> }.into_any()
        }}
    }
}
