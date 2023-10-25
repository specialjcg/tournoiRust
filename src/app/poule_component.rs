use yew::{prelude::*};
use crate::app::list_component::task_team::TaskRepository;

#[function_component(PouleComponent)]
pub fn poule_component() -> Html {
    let  task_repository = TaskRepository::new();
    // let task_repository = &props.task_repository;
    let name_list = use_state(|| task_repository.clone());

    html! {
        <div>
 <h1 class="title">{ "create Poules " } </h1>
        <div class="flex-container">
                { for (*name_list).get_tasks().iter().map(|team| {
                    html! {
                        <button class="btn-hover color-2 flex-items" >
                            { &team.team }
                        </button>
                    }
                })}

            </div>
        </div>




    }
}
// #[derive(Properties, Clone, PartialEq)]
// pub struct UserPropsPoule {
//     pub task_repository: TaskRepository, // Replace String with your User type
// }
