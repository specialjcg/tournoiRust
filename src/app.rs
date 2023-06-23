use std::fmt;
use std::rc::Rc;
use yew::prelude::*;
mod list;
use list::mainlist;
use list::Team;



#[function_component(App)]
pub fn app() -> Html {
    let test=mainlist();
    let name = use_state(|| String::new());
    let oninput = Callback::from({
        let name = name.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            //web_sys::console::log_1(&target.value().into()); // <- can console the value.
            name.set(target.value());
        }
    });
    html! {
         <main>
            <h1>{ "Liste des Team" }</h1>
        <input {oninput}  />
            <table class="styled-table">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Poule"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for test.iter().map(render_team_row) }
                </tbody>
            </table>
        </main>
    }

}
fn render_team_row(team: &Team) -> Html {
    html! {
        <tr class="active-row">
            <td>{ &team.team }</td>
            <td>{ &team.poule }</td>
        </tr>
    }
}
