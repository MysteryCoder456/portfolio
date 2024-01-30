use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod box_collision;
mod home;
mod projects;

use box_collision::BoxCollision;
use home::HomePage;
use projects::Projects;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let meta_description = "Hi there! I'm Rehatbir, and welcome to my humble abode on the internet :D I am a high school student who likes to code, play guitar, and loves learning.";

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        <Meta name="og:title" content="CodeBoi"/>
        <Meta name="og:image" content="/images/CB.png"/>
        <Meta name="og:url" content="https://codeboi.dev"/>
        <Meta name="theme-color" content="#09e85e"/>
        <Meta name="og:description" content=meta_description/>
        <Meta name="description" content=meta_description/>

        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Regular.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Bold.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Italic.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-BoldItalic.ttf"/>

        <Script async_="true" src="https://eu.umami.is/script.js" attrs=vec![("data-website-id", Attribute::String("ff25ff64-cebc-44a4-ac55-7a8bba3671ee".into()))]></Script>

        <div class="main-container">
            <main>
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }.into_view()
                }>
                    <Routes>
                        <Route path="" view=|| view! { <HomePage/> }/>
                        <Route path="/projects" view=|| view! { <Projects/> }/>
                        <Route path="/box" view=|| view! { <BoxCollision/> }/>
                    </Routes>
                </Router>
            </main>
        </div>

        <hr class="end-hr"/>
        <div class="end-info muted">
            <p>Made by CodeBoi with Rust + Leptos + lots of love</p>
            <a href="https://github.com/MysteryCoder456/codeboi.dev" class="muted" target="_blank">
                View Source on GitHub
            </a>
        </div>
    }
}
