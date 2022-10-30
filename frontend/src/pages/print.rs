use super::LoadingState;
use crate::{components::Qr, fetch, pages::BASE_API};
use shared::EventInfo;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct Print {
    event: Option<EventInfo>,
    loading_state: LoadingState,
}
pub enum Msg {
    Fetched(Option<EventInfo>),
}
impl Component for Print {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let event_id = ctx.props().id.clone();
        request_fetch(event_id, ctx.link());

        Self {
            loading_state: LoadingState::Loading,
            event: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetched(res) => {
                self.loading_state = if res.is_none() {
                    LoadingState::NotFound
                } else {
                    LoadingState::Loaded
                };

                self.event = res;

                true
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="event">
                {self.view_internal(ctx)}
            </div>
        }
    }
}

//TODO: un-dup
fn request_fetch(id: String, link: &html::Scope<Print>) {
    log::info!("request_fetch");

    link.send_future(async move {
        let res = fetch::fetch_event(BASE_API, id, None).await;

        if let Ok(val) = res {
            Msg::Fetched(Some(val))
        } else {
            Msg::Fetched(None)
        }
    });
}

impl Print {
    fn view_internal(&self, ctx: &Context<Self>) -> Html {
        match self.loading_state {
            LoadingState::Loaded => self.view_event(ctx),
            LoadingState::Loading => {
                html! {
                    <div class="noevent">
                        <h2>{"loading event..."}</h2>
                    </div>
                }
            }
            LoadingState::NotFound => {
                html! {
                    <div class="noevent">
                        <h2>{"event not found"}</h2>
                    </div>
                }
            }
        }
    }

    fn view_event(&self, _ctx: &Context<Self>) -> Html {
        if let Some(e) = self.event.as_ref() {
            let share_url = if e.data.short_url.is_empty() {
                e.data.long_url.clone().unwrap_or_default()
            } else {
                e.data.short_url.clone()
            };

            html! {
                <div>
                    <div class="bg-print">
                    </div>

                    <div class="event-block">

                        <div class="event-name printable">{&e.data.name.clone()}</div>

                        <div class="event-desc printable"
                            >
                            {{&e.data.description.clone()}}
                        </div>
                    </div>

                    <div class="qrbox print">
                        <div class="qr print">
                            <Qr url={share_url} dimensions={300} />
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}