use web_sys::{HtmlInputElement};
use yew::{prelude::*};
use crate::app::list::{add_team_to_team, remove_team, Team};

#[function_component(ListComponent)]
pub fn list_component() -> Html {
    let mut name_list = use_state(Vec::new);
    let input_ref: NodeRef = NodeRef::default();

    let on_click = {
        let name_list = name_list.clone();
        let cur_input = input_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut names:Vec<Team> = (*name_list).clone();
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            let newteam: String = name_input_element.value().to_string();


            let result  = add_team_to_team(newteam.as_str(), names.clone());
            match result {
                Ok(value) => names=value,
                Err(error) => eprintln!("Error: {}", error),
            }
            // let new_name:Team = name_input_element.value();
            // name_input_element.set_value("");
            // names.push(new_name);
            name_list.set(names);

        })
    };

    html!{
        <div>

            <div class="form__group field">
                <input class="form__field" ref={input_ref} type="text" placeholder="input a name"/>
                <label for="name" class="form__label">{"Name"}</label>
                <button class="btn-hover color-1" onclick={on_click}>{"Add to list"}</button>
            </div>
        <table class="styled-table">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Poule"}</th>
                    </tr>
                </thead>
                <tbody>
                    { for (*name_list).iter().map(render_team_row) }
                </tbody>
            </table>

        </div>
    }
}
fn render_team_row(team: &mut Team) -> Html {

    let on_click = {
        Callback::from(move |_e: MouseEvent| {


            // let new_name:Team = name_input_element.value();
            // name_input_element.set_value("");
            // names.push(new_name);
            team.team= "".parse().unwrap();
        })
    };
    html! {
        <tr class="active-row">
            <td>{ &team.team }</td>
            <td>{ &team.poule }</td>
         <button class="btn-hover color-5" onclick={on_click}>{"remove from list"}</button>
        </tr>
    }
}