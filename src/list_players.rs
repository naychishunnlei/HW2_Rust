use clap::{Arg, App};

fn main() {
    let matches = App::new("list_players")
        .arg(
            Arg::with_name("player_one").
            index(1).
            required(false)
        ).arg(
            Arg::with_name("player_two")
            .index(2)
            .required(false)
        )
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .get_matches();

    let playerf = matches.value_of("player_one").unwrap_or("N/A");
    let players = matches.value_of("player_two").unwrap_or("N/A");

    println!("Player 1: {}\nPlayer 2: {}", playerf, players);
}