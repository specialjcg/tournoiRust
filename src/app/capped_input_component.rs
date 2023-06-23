use yew::{Component, html, Html};
pub struct CappedInputComponent {}
impl Component for CappedInputComponent {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self  {}
    }
    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                {"Capped input component"}
            </div>
        }
    }
}