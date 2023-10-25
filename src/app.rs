mod list_component{
    pub(crate) mod task_team;
}
use list_component::task_team::TaskRepository;
mod addteam_to_poule;
mod add_team_to_team;
mod is_in_list_team;
mod list_random_team;
mod list_team_component;
mod poule_component;
pub mod list;



use yew_router::prelude::*;
use yew::prelude::*;
use crate::app::list_team_component::{ListComponent};
use crate::app::poule_component::{PouleComponent};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/poule")]
    Poule
}
fn switch(routes: Route) -> Html {
    // let mut name_list = TaskRepository::new();
    match routes {
        Route::Home => html! { <div>
              <h1 class="title">{ "Liste des Team" }</h1>
         <ListComponent />
            </div> },
        Route::Poule => html! {
            <PouleComponent />
        }
    }
}
#[function_component(App)]
pub fn app() -> Html {
    html! {

         <main>
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>



        </main>
    }

}

