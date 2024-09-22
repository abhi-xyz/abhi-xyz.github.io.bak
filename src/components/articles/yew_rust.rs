use yew::prelude::*;

#[function_component(YewRust)]
pub fn yew_rust() -> Html {
    html! {
    <div class="card">
      <h4>{ "Build a Rust + WebAssembly website with Yew" }</h4>
      <h5>{ "September 7, 2024" }</h5>
      <div class="fakeimg" style="height:200px;">{ "Image" }</div>
      <p>{ "While Rust is known for its backend web development capabilities, the advent of WebAssembly (Wasm) has made it possible to build rich frontend apps in Rust." }</p>
    <p>{ "For those hankering to explore the frontend of Rust development, we’ll learn how to build a very basic website ( no difference with web app ) using the Yew web framework." }</p>
    </div>
    }
}
