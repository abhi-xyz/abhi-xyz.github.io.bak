use crate::home_page::{Footer, Header};
use yew::prelude::*;

#[function_component(Articles)]
pub fn articles_page() -> Html {
    html!(
        <>
        <Header />

    <div class="row">
      <div class="leftcolumn">

        <div class="intro-card">
          <h4>{ "Articles" }</h4>
          <p>{ "This page shares my best articles to read on topics like linux, rust, css, productivity and more. The central question that drives my work is, “How can we live better?” To answer that question, I like to write about science-based ways to solve practical problems." }</p>
        <p>{ "You’ll find interesting articles to read on topics like how to build things in rust as well as personal recommendations like my list of the best books to read and articles and blog posts. Ready to dive in? You can use the categories below to browse my best articles or go to full index" }</p>
        </div>

        <div class="card">
          <h4>{ "Build a Rust + WebAssembly website with Yew" }</h4>
          <h5>{ "September 7, 2024" }</h5>
          <div class="fakeimg" style="height:200px;">{ "Image" }</div>
          <p>{ "While Rust is known for its backend web development capabilities, the advent of WebAssembly (Wasm) has made it possible to build rich frontend apps in Rust." }</p>
        <p>{ "For those hankering to explore the frontend of Rust development, we’ll learn how to build a very basic website ( no difference with web app ) using the Yew web framework." }</p>
        </div>

        <div class="card">
          <h4>{" TITLE HEADING" }</h4>
          <h5>{ "Title description, Sep 2, 2017" }</h5>
          <div class="fakeimg" style="height:200px;">{ "Image" }</div>
          <p>{ "Some text.." }</p>
        </div>

        <div class="card">
          <h4>{"more" }</h4> //TODO: add button here
        </div>

      </div>

      <div class="rightcolumn">
       <div class="card">
          <h4>{ "Filter" }</h4>
          <p>{ "Work in progress.." }</p>
        </div>

        <div class="card">
          <h4>{ "Sort by tags" }</h4>
          <p>{ "Work in progress.." }</p>
        </div>
        <div class="card">
          <h5>{ "Popular Post" }</h5>
        <p>{ "Work in progress..." }</p>
        <p>{ "Work in progress..." }</p>
        <p>{ "Work in progress..." }</p>
        <p>{ "Work in progress..." }</p>
        </div>
      </div>
    </div>

        <Footer />
        </>
        )
}
