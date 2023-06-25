

fn add_team_to_poule(team : &str, teams: Vec<Team>,poule:i32) -> Vec<Team> {
    let mut teamvec:Vec<Team> = teams;
    let index = teamvec.iter().position(|x| *x.team == team.to_string()).unwrap();
    teamvec[index].poule=poule;
    return teamvec;

}

pub(crate)  fn add_team_to_team(team: &str, teams: Vec<Team>) -> Result<Vec<Team>, String> {
    let newteam: String = team.to_string();

    if is_in_list_team(newteam.as_str(), teams.clone()){
        return Err(String::from("nom dèjà utiliser"))
    };
    let mut teamsvec = teams;
    teamsvec.push(Team { team: newteam.to_string(), poule: 0 });
    return Ok(teamsvec);
}





fn is_in_list_team(teamtest: &str, teams: Vec<Team>) -> bool {
    for teambool in teams {
        if teamtest == teambool.team.to_string() {return true}}

    return false;
}




pub(crate) fn remove_team(team : &str, teams: Vec<Team>) -> Vec<Team>  {
    let mut teamvec: Vec<Team> = teams.clone();
    let team_poule:Team=Team{team: team.to_string(), poule: 0 };
    let index = teamvec.iter().position(|x| *x.team == team_poule.team).unwrap();
    teamvec.remove(index);
    return teamvec;

}

fn change_name(team : &str, teamchange : &str, teams:Vec<Team>) -> Vec<Team>  {
    let mut teamvec:Vec<Team> = teams;
    let index = teamvec.iter().position(|x| *x.team == team.to_string()).unwrap();
    teamvec[index].team=teamchange.to_string();
    return teamvec;

}
use serde::{Serialize, Deserialize};
use serde_json;
pub(crate) fn mainlist() -> Vec<Team> {

    let mut teams: Vec<Team> = Vec::new();
    let newteam: String = "team1".to_string();


    let result  = add_team_to_team(newteam.as_str(), teams.clone());
    match result {
        Ok(value) => teams=value,
        Err(error) => eprintln!("Error: {}", error),
    }
    let newteam: String = "team2".to_string();

    let result  = add_team_to_team(newteam.as_str(), teams.clone());
    match result {
        Ok(value) => teams=value,
        Err(error) => eprintln!("Error: {}", error),
    }
    let mut owned_vector = teams.clone();
    owned_vector=add_team_to_poule("team1",owned_vector.clone(),1);
    owned_vector=add_team_to_poule("team2",owned_vector.clone(),1);


    let json_string = serde_json::to_string_pretty(&owned_vector).unwrap();
    println!("{}", json_string);
    return owned_vector;
}
#[derive(Debug)]
#[derive(PartialEq,Serialize,Deserialize)]
#[derive(Clone)]
pub(crate) struct Team {
    pub team: String,
    pub poule:i32
}


#[cfg(test)]
mod tests {
    use crate::app::Team;
    use crate::app::list::{add_team_to_team, change_name};
    use super::add_team_to_poule;
    use super::remove_team;





    #[test]
    fn add2teams_but_notdoublon_return_error() {
        let mut teams: Vec<Team> = Vec::new();
        let newteam: String = "team1".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let owned_vector = teams.clone();



        assert_eq!(owned_vector, Vec::from([Team { team: "team1".parse().unwrap(), poule: 0 }]) );
    }


    #[test]
    fn test_greet_remove_team() {
        let mut teams: Vec<Team> = Vec::new();
        let newteam: String = "team1".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let newteam: String = "team2".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let owned_vector = teams.clone();
        let removeteam: Vec<Team> = remove_team("team1", owned_vector);

        assert_eq!(removeteam, vec![Team { team: "team2".to_string(), poule: 0 }] );
    }


    #[test]
    fn test_creates_poules() {
        let mut teams: Vec<Team> = Vec::new();
        let newteam: String = "team1".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let newteam: String = "team2".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        let owned_vector = teams.clone();
        assert_eq!(owned_vector,  Vec::from([Team { team: "team1".parse().unwrap(), poule: 0 }, Team { team: "team2".parse().unwrap(), poule: 0 }]) );
    }


    #[test]
    fn test_changenameteam() {
        let mut teams: Vec<Team> = Vec::new();
        let newteam: String = "team1".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        teams = change_name("team1", "teamchange", teams);

        assert_eq!(teams, vec![Team { team: "teamchange".to_string(), poule: 0 }]);
    }
    #[test]
    fn affect_poule_to_team() {
        let mut teams: Vec<Team> = Vec::new();
        let newteam: String = "team1".to_string();

        let result = add_team_to_team(newteam.as_str(), teams.clone());
        match result {
            Ok(value) => teams = value,
            Err(error) => eprintln!("Error: {}", error),
        }
        teams = add_team_to_poule("team1", teams,1);

        assert_eq!(teams, vec![Team { team: "team1".to_string(), poule: 1 }]);
    }
}

