#[cfg(test)]
use games_grid::{list_all_games, list_game_by_id, list_game_by_name};

#[test]
fn list_games() {
    let allgames = list_all_games();
    assert!(allgames.is_err());

    let byid = list_game_by_id(1);
    assert!(byid.is_err());

    let byname = list_game_by_name("Minecraft".into());
    assert!(byname.is_err());
}
