use yew::prelude::*;

#[function_component(YewRust)]
pub fn yew_rust() -> Html {
    html! {
        <div class="card">
            <h1 id="articleH1">{ "Build a Rust + WebAssembly website with Yew" }</h1>
            <h5>{ "September 7, 2024" }</h5>
            <div class="fakeimg" style="height:200px;">{ "Image" }</div>
            <p>{ "While Rust is known for its backend web development capabilities, the advent of WebAssembly (Wasm) has made it possible to build rich frontend apps in Rust." }</p>
            <p>{ "For those hankering to explore the frontend of Rust development, we’ll learn how to build a very basic website ( no difference with web app ) using the Yew web framework." }</p>
            <h5>{ "Let's walk through the basics of getting started with Yew." }</h5>
            <h2 id="articleH2">{ "Setting Up Your Environment" }</h2>
            <p>{ "Ensure that you have a Unix-like system (Linux or macOS) for these instructions. If you don't have Rust installed yet, you can use Nix to install and manage Rust, which is useful for creating isolated and reproducible development environments." }</p>
            <h4>{ "Install Nix" }</h4>
            <p>{ "First, you need to install Nix, a package manager that allows you to create isolated development environments."}</p>

        <pre class="src src-bash">

        <p>{ "Install Nix as a single-user installation" }</p>
    //    {"sh &lt;"}
    <span style="color: #a6e3a1; font-weight: bold;">{"sh "}</span>
    <span style="color: #e5c890; font-weight: bold;">{"<(curl -L https://nixos.org/nix/install)"}</span>
        <span style="color: #e78284; font-weight: bold;">{" --no-daemon" }</span>
        </pre>

        </div>
    }
}
