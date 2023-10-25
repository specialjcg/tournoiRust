use crate::app::list::Team;

pub(crate) fn is_in_list_team(teamtest: &str, teams: Vec<Team>) -> bool {
    for teambool in teams {
        if teamtest == teambool.team.to_string() {return true}}

    return false;
}
