use std::borrow::Borrow;

use rand::Rng;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{prelude::*};

use crate::app::list::Team;
use crate::app::list_random_team::TEAMS;
use crate::app::TaskRepository;
use crate::app::Route;
use yew_router::prelude::Link;

#[function_component(ListComponent)]
pub fn list_component() -> Html {
    let mut name_list = use_state(|| TaskRepository::new());
    let client = Client::new();
    let mut name = (*name_list).clone();
    let name_list_clone = name_list.clone();

    spawn_local(async move {

        // Create a request
        let request = client
            .get("http://localhost:8000/teams")
            .header("Content-Type", "application/json")
            .send()
            .await;

        match request {
            Ok(response) => {
                if response.status().is_success() {
                    // Read the response body as bytes.
                    let body_bytes = response.bytes().await.unwrap();
                    // Convert the bytes into a string.
                    let body_string = String::from_utf8_lossy(&body_bytes);

                    // Deserialize the JSON string into your struct.
                    let parsed_data: Result<Vec<Team>, _> = serde_json::from_str(&body_string.to_string());
                    name= TaskRepository::new();
                    match parsed_data {
                        Ok(data) => {
                            // Successfully parsed data
                            // 'data' contains the Vec<Team>

                            data.iter().for_each(|team| {
                                let newteam: String = team.team.clone();
                                name.add_task(newteam.to_string());
                            });
                        }
                        Err(e) => {
                            log::info!("Error parsing JSON: {}", e);
                        }
                    }
                } else {
                    log::info!("Received an unsuccessful response: {:?}", response.status());
                }
            }
            Err(error) => log::info!("Error: {}", error),
        }

        name_list_clone.set(name);
    });
    // log::info!("Parsed JSON data: {:?}", name_list.get_tasks());
    // log::info!("Parsed JSON data: {:?}", name.get_tasks());
    let input_ref: NodeRef = NodeRef::default();
    let on_click_remove = |team_name: String| {
        let name_list = name_list.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut names: TaskRepository = (*name_list).clone();
            names.remove_task(team_name.clone());
            let json_data = serde_json::to_string(&names.get_tasks()).unwrap();
            let client = Client::new();

            spawn_local(async move {
                log::info!("Some info {}",json_data);
                // Create a request
                let request = client
                    .post("http://localhost:8000/users")
                    .header("Content-Type", "application/json")
                    .body(json_data)
                    .send()
                    .await;
            });
            name_list.set(names);
        })
    };


    let on_click_random = {
        let name_list = name_list.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut names: TaskRepository = (*name_list).clone();
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..196);
            names.add_task(TEAMS[index].to_string());


            let json_data = serde_json::to_string(&names.get_tasks()).unwrap();
            let client = Client::new();

            spawn_local(async move {
                let client = Client::new();
                log::info!("Some info {}",json_data);
                // Create a request
                let request = client
                    .post("http://localhost:8000/users")
                    .header("Content-Type", "application/json")
                    .body(json_data)
                    .send()
                    .await;
            });

            // You can use name_list_clone here as needed
            name_list.set(names);
        })
    };
    let on_click = {
        let name_list = name_list.clone();
        let cur_input = input_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut names: TaskRepository = (*name_list).clone();
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            let newteam: String = name_input_element.value().to_string();
            if newteam.len() > 0 {
                //
                names.add_task(newteam.to_string());
                let json_data = serde_json::to_string(&names.get_tasks()).unwrap();
                let client = Client::new();

                spawn_local(async move {
                    let client = Client::new();
                    log::info!("Some info {}",json_data);
                    // Create a request
                    let request = client
                        .post("http://localhost:8000/users")
                        .header("Content-Type", "application/json")
                        .body(json_data)
                        .send()
                        .await;
                });
                name_list.set(names);

                // (*name_list).save_to_json();

            }
        })
    };
    let name_clone = name_list.clone();

    html! {

        <div>

            <div class="form__container">
                <input class="form__field" ref={input_ref} type="text" placeholder="input a name"/>
                <label for="name" class="form__label">{"Name"}</label>
            <button class="btn-hover color-1 buttonsRandomTeam" onclick={on_click_random}>{"random name"}</button>
                <button class="btn-hover color-1 buttonsaddTeam" onclick={on_click}>{"Add to list"}</button>
                <button class="btn-hover color-1 buttonsPouleTeam">
        <Link<Route> to={Route::Poule}>{ "click here to go Poule" }</Link<Route>>
        </button>

        </div>
 <div class="flex-container">
                {

                    for name_clone.get_tasks().iter().map(|team| {
 let on_click_remove = on_click_remove(team.team.clone());
                    html! {
                        <button class="btn-hover color-2 flex-items" onclick={on_click_remove}>
                            { &team.team }
                        </button>
                    }
                })}

            </div>


        </div>
    }
}
