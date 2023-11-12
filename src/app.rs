use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod home;
mod models;

use home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let meta_description = "Hi there! I'm Rehatbir, and welcome to my humble abode on the internet :D I am a high school student who likes to code, play guitar, and loves learning.";

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Stylesheet href="/stylers.css"/>
        <Title text="CodeBoi"/>

        <Meta name="og:title" content="CodeBoi" />
        <Meta name="og:image" content="/images/CB.png" />
        <Meta name="og:url" content="https://codeboi.dev" />
        <Meta name="theme-color" content="#09e85e" />
        <Meta
            name="og:description"
            content={meta_description}
        />

        <Meta
            name="description"
            content={meta_description}
        />


        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <div class="main-container">
                <main>
                    <Routes>
                        <Route path="" view=|| view! { <HomePage/> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
