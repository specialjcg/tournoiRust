use crate::app::list::Team;

pub(crate) fn add_team_to_poule(team : &str, teams: Vec<Team>, poule:i32) -> Vec<Team> {
    let mut teamvec:Vec<Team> = teams;
    let index = teamvec.iter().position(|x| *x.team == team.to_string()).unwrap();
    teamvec[index].poule=poule;
    return teamvec;

}
