use crate::Route;
use ::yew_router::prelude::*;
use yew::prelude::*;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    html!(
            <>
            /* ------------------------------------------------------ */
        <div class="header_navigation">
            <header class="header_desktop">
                <nav class="navigation-desktop">
                          <h1 class="desktop-navigation-heading">{ "ABHINANDH S" }<span class="color">{ "." }</span></h1>
                        <ul class="desktop-nav-list">
                           <li class="desktop-link"><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                           <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                           <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                           <li><Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>></li>
                        </ul>
                </nav>
            </header>

            // Menu button
                        <button class="navigation-menu-button" onclick={toggle_menu.clone()}>
                         //   { if *is_menu_open { "☰" } else { "☰" } }
            <img src="assets/icons/menu_25dp_catppuccin_text.svg" alt="Close Icon" />
                        </button>

            /*
            // Navigation links
                        <nav class={if *is_menu_open { "navigation-open" } else { "nav-closed" }}>
                            <ul class="navigation-mobile">
                                <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                                <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                                <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                                <li><Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>></li>
                            </ul>
                        </nav>
    */
        </div>
        /* -------------------------------------------------------- */
            </>
            )
}
#[function_component(Portfolio)]
pub fn portfolio_page() -> Html {
    html!(
        <>

    <NavigationMenu />

    <main>
        <div class="test-container1">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container2">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container3">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container1">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container2">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container3">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>

    <div class="test-container1">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container2">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>
        <div class="test-container3">
            <h2>{"Hello Woeld"}</h2>
                <p>{"aadsdsaadsiydioydsaadsdsufoifgdsufiydioydsaadsdsufoifgdsufiydioydsoyds"}</p>
        </div>

    </main>
        </>
    )
}
