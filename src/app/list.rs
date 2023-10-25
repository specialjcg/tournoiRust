
pub(crate) fn remove_team(team : &str, teams: Vec<Team>) -> Vec<Team>  {

    let mut teamvec: Vec<Team> = teams.clone();
    let team_poule:Team=Team{team: team.to_string(), poule: 0 ,is_removed:false};
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
use serde::{Deserialize, Serialize};
use yew::{Properties};

#[derive(Debug)]
#[derive(PartialEq,Serialize,Deserialize)]
#[derive(Clone,Properties)]
pub struct Team {
    pub team: String,
    pub poule:i32,
    pub is_removed: bool
}



#[cfg(test)]
mod tests {
    use crate::app::list::{change_name, Team};
    use crate::app::addteam_to_poule::add_team_to_poule;
    use crate::app::add_team_to_team::add_team_to_team;
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



        assert_eq!(owned_vector, Vec::from([Team { team: "team1".parse().unwrap(), poule: 0 ,is_removed:false}]) );
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

        assert_eq!(removeteam, vec![Team { team: "team2".to_string(), poule: 0 ,is_removed:false}] );
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
        assert_eq!(owned_vector,  Vec::from([Team { team: "team1".parse().unwrap(), poule: 0 ,is_removed:false}, Team { team: "team2".parse().unwrap(), poule: 0,is_removed:false }]) );
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

        assert_eq!(teams, vec![Team { team: "teamchange".to_string(), poule: 0 ,is_removed:false}]);
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

        assert_eq!(teams, vec![Team { team: "team1".to_string(), poule: 1 ,is_removed:false}]);
    }
}

