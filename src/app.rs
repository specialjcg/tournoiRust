use std::fmt;
use std::rc::Rc;
use yew::prelude::*;
mod list;
use list::mainlist;
use list::Team;
use capped_input_component::CappedInputComponent;
mod capped_input_component;
use list_component::ListComponent;
mod list_component;


#[function_component(App)]
pub fn app() -> Html {
    let test=mainlist();

    html! {
         <main>
            <h1>{ "Liste des Team" }</h1>
         <div>
               <div>
         <ListComponent/>
            </div>
            </div>

        </main>
    }

}

