to run this use should use the command 'trunk serve'



FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
VOLUME /code
WORKDIR /code
EXPOSE 8081
CMD trunk serve




use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    token: String, // Assuming you have a token to determine if the user is logged in
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            token: "your_token_here".to_string(), // Initialize with an empty token or fetch from somewhere
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let filter_options = vec!["Option 1", "Option 2", "Option 3"]; // Example options
        let sort_options = vec!["Option A", "Option B", "Option C"]; // Example options

        html! {
            <section class="container">
                { self.render_filter_sort() }
                { self.render_tasks() }
                // Add buttons conditionally based on token presence
                { if !self.token.is_empty() {
                    html! {
                        <div class="buttons">
                            <button onclick=self.link.callback(|_| Msg::GoToPostPage)>{"Post"}</button>
                            <button onclick=self.link.callback(|_| Msg::GoToPutPage)>{"Put"}</button>
                            <button onclick=self.link.callback(|_| Msg::GoToDeletePage)>{"Delete"}</button>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </section>
        }
    }
}

impl Model {
    fn render_filter_sort(&self) -> Html {
        let filter_options = vec!["Option 1", "Option 2", "Option 3"]; // Example options
        let sort_options = vec!["Option A", "Option B", "Option C"]; // Example options

        html! {
            <div>
                <div class="filter">
                    <BBSelect
                        data_test="filter"
                        id="filter"
                        label="Filter Tasks"
                        options={filter_options.clone()}
                        onchange={self.link.callback(|_| Msg::FilterOnChange)}
                    />
                </div>
                <div class="sort">
                    <BBSelect
                        data_test="sort"
                        id="sort"
                        label="Sort Tasks"
                        options={sort_options.clone()}
                        onchange={self.link.callback(|_| Msg::SortOnChange)}
                    />
                </div>
            </div>
        }
    }

    fn render_tasks(&self) -> Html {
        // Function to render tasks
        html! {
            // Render tasks here
        }
    }
}

// Define your Msg enum and associated functions for routing
enum Msg {
    GoToPostPage,
    GoToPutPage,
    GoToDeletePage,
    FilterOnChange,
    SortOnChange,
}
