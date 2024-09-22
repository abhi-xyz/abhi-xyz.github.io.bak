use crate::Route;
use ::yew_router::prelude::*;
use yew::prelude::*;

#[function_component(YewWebsite)]
pub fn yew_website() -> Html {
    html!(
    <>
    <header>
    <div class="wrapper">
    <h1>{ "ABHINANDH S" }<span class="color">{ "." }</span></h1>
    <nav>
    <ul>
    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
    <li><Link<Route> to={Route::Portfolio}>{ "Portfolio" }</Link<Route>></li>
    <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
    <li><Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>></li>
    </ul>
    </nav>
    </div>
    </header>
    <hr />
    <div class="article">
    <h2>{"Creating a website using the Yew framework in Rust"}</h2>

    <p>{ "In this blog post, we'll walk through the steps to create a multi page website using the Yew framework." }</p>
    <h3>{"Prerequisites"}</h3>
    <p>{ "Before diving into the code, you'll need to set up your development environment. Ensure you have the following installed:" }</p>
    <ul>
    <li>
    { "Rust: Yew is built in Rust, so you'll need to have Rust installed. You can install Rust by following the instructions on the " }
    <a href="https://www.rust-lang.org/tools/install" target="_blank">{ "official website" }</a>
    { "." }
    </li>
    </ul>

    <h3>{ "Step 1: Setting Up the Project" }</h3>
    <p>{ "To get started, let's create a new Yew project. Open your terminal and run:" }</p>

    <pre>
    <code>{ "[dependencies]\nyew = { version = \"0.20\", features = [\"csr\"] }\nyew-router = \"0.17\"" }</code>
    </pre>
    <h2>{ "Step 2: Writing the Yew App" }</h2>
    <p>{ "Now, let's write a simple Yew app. Open the " }<code>{ "src/main.rs" }</code>{ " file and replace its contents with the following code:" }</p>
    <pre>
    <code>
    { "use yew::prelude::*;\n\nstruct App;\n\nimpl Component for App {\n    type Message = ();\n    type Properties = ();\n\n    fn create(_ctx: &Context<Self>) -> Self {\n        App\n    }\n\n    fn update(&mut self, _msg: Self::Message) -> bool {\n        false\n    }\n\n    fn view(&self, _ctx: &Context<Self>) -> Html {\n        html! {\n            <div>\n                <h1>{ \"Welcome to My Yew Website!\" }</h1>\n                <p>{ \"This is a simple website built using the Yew framework in Rust.\" }</p>\n            </div>\n        }\n    }\n}\n\nfn main() {\n    yew::Renderer::<App>::new().render();\n}" }
    </code>
    </pre>

    <h3>{ "Explanation:" }</h3>
    <ul>
    <li>{ "Component: The " }<code>{ "App" }</code>{ " struct represents a component in Yew. Components are the building blocks of Yew applications." }</li>
    <li>{ "create: This method is called when the component is first created. In this example, it returns an instance of " }<code>{ "App" }</code>{ "." }</li>
    <li>{ "view: This method defines how the component is rendered. It returns an HTML-like syntax (using the " }<code>{ "html!" }</code>{ " macro) to define the UI." }</li>
    </ul>

    <h2>{ "Step 3: Building and Running the App" }</h2>
    <p>{ "Now that the basic Yew app is set up, it's time to build and run it. Use the following command to build and serve your app:" }</p>
    <pre>
    <code>{ "trunk serve" }</code>
    </pre>
    <p>{ "This command will build your Yew app and start a development server at " }<code>{ "http://127.0.0.1:8080" }</code>{ ". Open this URL in your browser, and you should see the message \"Welcome to My Yew Website!\"" }</p>

    <h2>{ "Step 4: Adding More Pages (Routing)" }</h2>
    <p>{ "A website typically has multiple pages, so let's add routing to our Yew app. For routing, we'll use the " }<code>{ "yew-router" }</code>{ " crate. Add it to your " }<code>{ "Cargo.toml" }</code>{ " file:" }</p>
    <pre>
    <code>{ "[dependencies]\nyew = { version = \"0.20\", features = [\"csr\"] }\nyew-router = \"0.17\"" }</code>
    </pre>
    <p>{ "Now, let's modify the " }<code>{ "src/main.rs" }</code>{ " file to include routing:" }</p>
    <pre>
    <code>
    { "use yew::prelude::*;\nuse yew_router::prelude::*;\n\n#[derive(Routable, PartialEq, Clone, Debug)]\nenum Route {\n    #[at(\"/\")]\n    Home,\n    #[at(\"/about\")]\n    About,\n}\n\nfn switch(routes: &Route) -> Html {\n    match routes {\n        Route::Home => html! { <Home /> },\n        Route::About => html! { <About /> },\n    }\n}\n\nstruct Home;\nstruct About;\n\nimpl Component for Home {\n    type Message = ();\n    type Properties = ();\n\n    fn create(_ctx: &Context<Self>) -> Self {\n        Home\n    }\n\n    fn update(&mut self, _msg: Self::Message) -> bool {\n        false\n    }\n\n    fn view(&self, _ctx: &Context<Self>) -> Html {\n        html! {\n            <div>\n                <h1>{ \"Home Page\" }</h1>\n                <p>{ \"Welcome to the Home Page!\" }</p>\n                <a href=\"/about\">{ \"Go to About Page\" }</a>\n            </div>\n        }\n    }\n}\n\nimpl Component for About {\n    type Message = ();\n    type Properties = ();\n\n    fn create(_ctx: &Context<Self>) -> Self {\n        About\n    }\n\n    fn update(&mut self, _msg: Self::Message) -> bool {\n        false\n    }\n\n    fn view(&self, _ctx: &Context<Self>) -> Html {\n        html! {\n            <div>\n                <h1>{ \"About Page\" }</h1>\n                <p>{ \"This is the About Page of the Yew website.\" }</p>\n                <a href=\"/\">{ \"Go to Home Page\" }</a>\n            </div>\n        }\n    }\n}\n\nfn main() {\n    yew::Renderer::<Router<Route>>::new().render();\n}" }
    </code>
    </pre>

    </div>
    </>
    )
}
