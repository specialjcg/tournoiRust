use std::fmt;
use std::rc::Rc;
use yew::prelude::*;
mod list;
use list::mainlist;
use list::Team;
use capped_input_component::CappedInputComponent;
mod capped_input_component;


#[function_component(App)]
pub fn app() -> Html {
    let test=mainlist();

    html! {
         <main>
            <h1>{ "Liste des Team" }</h1>
         <div>
               <div>
                <CappedInputComponent min_value={0} max_value={20}/>
            </div>
            </div>
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
