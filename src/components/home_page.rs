use crate::Route;
use ::yew_router::prelude::*;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
        <Header />

                    <div id="primary-content">
                        <div class="wrapper">
                            <article>
                                <h3>{ "About me" }</h3>
                                <p>{ "I’ve been writing about some things and other some thing since 2024. I'm a technology enthusiast with a passion for Rust programming and Linux systems. Here, you'll find a variety of content, including tech musings, project updates, and random ideas and discoveries. This space is where I share the less formal, yet intriguing aspects of my work and interests." }</p>
                            </article>
                        </div>
                    </div>


        <div id="secondary-content">
            <div class="wrapper">
                    <div class="overlay">
                        <h4>{ "Articles on Rust" }</h4>
                        <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt nam." }</p>
                        <a href="#" class="more-link">{ "Go to index" }</a>
                    </div>
                <div class="clear"></div>
            </div>
        </div>
            <div id="cta">
                <div class="wrapper">
                    <h3>{ "View My Work" }</h3>
                    <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod leo a nibh dignissim tincidunt. Nam ultricies odio ac neque suscipit volutpat. Ut dictum adipiscing felis sed malesuada. Integer porta sem nec nibh hendrerit imperdiet." }</p>
                    <a href="#" class="button-2">{ "Get Started" }</a>
                </div>
            </div>
    <Footer />
            </>
        }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <header>
                <div class="wrapper">
                    <h1>{ "ABHINANDH S" }<span class="color">{ "." }</span></h1>
                    <nav>
                        <ul>
                            <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                            <li><Link<Route> to={Route::ArticlesRoute}>{ "Articles" }</Link<Route>></li>
                        </ul>
                    </nav>
                </div>
            </header>
        <div>
            <nav id="mobile-nav">
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                    <li><Link<Route> to={Route::ArticlesRoute}>{ "Articles" }</Link<Route>></li>
                </ul>
            </nav>
        </div>
    </>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="wrapper">
                <div id="footer-info">
                    <p>{ "Except where otherwise noted, content on this site is licensed under a " }<a href="https://creativecommons.org/publicdomain/zero/1.0/" target="_blank">{ "Creative Commons Zero (CC0) license"}</a></p> // Except where otherwise noted, content on this site is licensed under a <a href="https://creativecommons.org/publicdomain/zero/1.0/">Creative Commons Zero (CC0) license</a>.
                    <a href="https://creativecommons.org/publicdomain/zero/1.0/" target="_blank">
                    <img src="https://licensebuttons.net/p/zero/1.0/88x31.png" alt="CC0" />
                    </a>
                    <p><a href="#">{ "Terms of Service & Privacy" }</a></p>
                </div>
            <div id="footer-links">
                <ul>
                    <li><h5>{ "Links" }</h5></li>
                    <li><a href="https://x.com/Ungraduate_Abhi" target="_blank">{ "X / Twitter" }</a></li>
                    <li><a href="https://github.com/abhi-xyz" target="_blank">{ "Github" }</a></li>
                    <li><a href="https://git.sr.ht/~abhinandh/" target="_blank">{ "Sourcehut" }</a></li>
                </ul>
            </div>
        <div class="clear"></div>
        </div>
    </footer>
    }
}
