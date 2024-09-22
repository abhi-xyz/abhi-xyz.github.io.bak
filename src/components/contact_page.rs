use crate::home_page::{Footer, Header};
use crate::Route;
use ::yew_router::prelude::*;
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact_page() -> Html {
    html!(
        <>
            <Header />
            <Footer />
        </>
    )
}
