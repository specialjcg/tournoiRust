use crate::app::is_in_list_team::is_in_list_team;
use crate::app::list::Team;
pub(crate) fn add_team_to_team(team: &str, teams: Vec<Team>) -> Result<Vec<Team>, String> {
    let newteam: String = team.to_string();

    if is_in_list_team(newteam.as_str(), teams.clone()){
        return Err(String::from("nom dèjà utiliser"))
    };
    let mut teamsvec = teams;
    teamsvec.push(Team { team: newteam.to_string(), poule: 0,is_removed: false });
    return Ok(teamsvec);
}
