use yew::prelude::*;
use yew_router::prelude::*;

// home, Portfolio, articles, contact
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/Portfolio")]
    Portfolio,
    #[at("/contact")]
    Contact,
    #[at("/articles")]
    ArticlesRoute,
    #[at("/articles/*")]
    Articles,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum ArticlesRoute {
    #[at("/articles/post/:id")]
    Post { id: String },
    #[at("/articles")]
    Articles,
    #[at("/articles/yew-website")]
    YewWebsite,
    #[at("/articles/yew-rust")]
    YewRust,
    #[not_found]
    #[at("/articles/404")]
    NotFound,
}
mod components;

use crate::components::articles::yew_rust::YewRust;
use crate::components::articles_page;
use crate::components::contact_page;
use crate::components::home_page;
use crate::components::portfolio_page;
use crate::components::yew_website;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home_page::HomePage /> },
        Route::Portfolio => html! { <portfolio_page::Portfolio /> },
        Route::Contact => html! { <contact_page::Contact /> },
        Route::ArticlesRoute | Route::Articles => {
            html! { <Switch<ArticlesRoute> render={switch_settings} /> }
        }
        Route::NotFound => {
            html! { <> <h1>{ "404, Page not found." }</h1><p>{ "Well, we got a problem here :(" }</p> </> }
        }
    }
}
fn switch_settings(route: ArticlesRoute) -> Html {
    match route {
        ArticlesRoute::Post { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
        ArticlesRoute::YewWebsite => html! {<yew_website::YewWebsite />},
        ArticlesRoute::YewRust => html! {<YewRust />},
        ArticlesRoute::Articles => html! {<articles_page::Articles />},
        ArticlesRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
    }
}
