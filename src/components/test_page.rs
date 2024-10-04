use crate::home_page::{Footer, Header};
use yew::prelude::*;

use crate::components::articles::yew_rust::YewRust;

#[allow(dead_code)]
#[derive(Debug)]
pub struct WritingComponent {
    id: u16,
    heading: String,
    date: String,
    contents: Vec<String>,
}

impl WritingComponent {
    pub fn full_content(&self) -> Vec<Html> {
        // Map each string in contents to a <p> element
        let paragraphs: Vec<Html> = self
            .contents
            .iter()
            .map(|content| {
                html! { <p>{ content }</p> }
            })
            .collect();
        paragraphs
    }

    #[allow(dead_code)]
    pub fn full_content_html(&self) -> Html {
        // Create a vector of paragraphs as Html elements
        let paragraphs: Vec<Html> = self
            .contents
            .iter()
            .map(|content| {
                html! { <p>{ content }</p> }
            })
            .collect();

        // Construct the overall HTML structure
        html! {
            <div class="card">
                <h4>{ "Build a Rust + WebAssembly website with Yew" }</h4>
                <h5>{ "September 7, 2024" }</h5>
                <div class="fakeimg" style="height:200px;">{ "Image" }</div>
                { for paragraphs } // Insert all paragraphs here
            </div>
        }
    }

    // Will print the first 100 characters in contents
    pub fn summary(&self) -> String {
        let combined_content = self.contents.join(" "); // Combine the contents similarly
                                                        // Truncate to the first 100 characters
        let summary = if combined_content.len() > 100 {
            format!("{}...", &combined_content[..100])
        } else {
            combined_content
        };
        summary
    }
}

#[function_component(TestPage)]
pub fn test_page() -> Html {
    let all_articles = vec![
        WritingComponent {
            id: 1,
            date: "September 7, 2024".to_string(),
            heading: "1 Build a Rust + WebAssembly website with Yew".to_string(),
            contents: vec![
                "This page shares my best articles to read on topics like linux, rust, css, productivity and more. The central question that drives my work is, “How can we live better?” To answer that question, I like to write about science-based ways to solve practical problems.".to_string(),
                "This page shares my best articles to read on topics like linux, rust, css, productivity and more. The central question that drives my work is, “How can we live better?” To answer that question, I like to write about science-based ways to solve practical problems.".to_string()
            ],
        },
        WritingComponent {
            id: 2,
            date: "September 7, 2024".to_string(),
            heading: "2 Build a Rust + WebAssembly website with Yew".to_string(),
            contents: vec!["This page shares my best articles to read on topics like linux, rust, css, productivity and more. The central question that drives my work is, “How can we live better?” To answer that question, I like to write about science-based ways to solve practical problems.".to_string()],
        },
    ];
    html!(
        { for all_articles.iter().map(|all_articles| html!(
        <div class="card">
            <h4>{ &all_articles.heading }</h4>
            <h5>{ &all_articles.date }</h5>
            <div class="fakeimg" style="height:200px;">{ "Image" }</div>
          //  <p>{ all_articles.contents.clone() }</p>
            <p>{ all_articles.summary() }</p>
            <div>{ all_articles.full_content() }</div>
          //  <div>{ all_articles.full_content_html() }</div>
        </div>
    )) }
        )
}

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

        <YewRust />

      // test 1
      <TestPage />
      // test 1 ends here


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
