use views::App;

mod components;
mod views;



fn main() {
    yew::Renderer::<App>::new().render();
}