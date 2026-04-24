use std::collections::HashMap;

#[derive(Debug)]
struct TeamDetails {
    mp: u8,
    w: u8,
    d: u8,
    l: u8,
    p: u8,
}

pub fn tally(match_results: &str) -> String {
    if match_results.trim().is_empty() {
        return String::from(format!("Team{}| MP |  W |  D |  L |  P", " ".repeat(27)));
    }

    let mut data: HashMap<String, TeamDetails> = HashMap::new();
    let results: Vec<&str> = match_results.split("\n").collect();

    for result in results {
        let splitted: Vec<&str> = result.split(";").collect();
        let left_team = splitted[0];
        let right_team = splitted[1];
        let match_result = splitted[2];

        // Win
        if match_result == "win" {
            if let Some(left_team_data) = data.get_mut(left_team) {
                left_team_data.mp += 1;
                left_team_data.w += 1;
                left_team_data.p += 3;
            } else {
                data.insert(
                    String::from(left_team),
                    TeamDetails {
                        mp: 1,
                        w: 1,
                        d: 0,
                        l: 0,
                        p: 3,
                    },
                );
            }

            if let Some(right_team_data) = data.get_mut(right_team) {
                right_team_data.mp += 1;
                right_team_data.l += 1;
            } else {
                data.insert(
                    String::from(right_team),
                    TeamDetails {
                        mp: 1,
                        w: 0,
                        d: 0,
                        l: 1,
                        p: 0,
                    },
                );
            }
        }

        // Loss
        if match_result == "loss" {
            if let Some(left_team_data) = data.get_mut(left_team) {
                left_team_data.mp += 1;
                left_team_data.l += 1;
            } else {
                data.insert(
                    String::from(left_team),
                    TeamDetails {
                        mp: 1,
                        w: 0,
                        d: 0,
                        l: 1,
                        p: 0,
                    },
                );
            }

            if let Some(right_team_data) = data.get_mut(right_team) {
                right_team_data.mp += 1;
                right_team_data.w += 1;
                right_team_data.p += 3;
            } else {
                data.insert(
                    String::from(right_team),
                    TeamDetails {
                        mp: 1,
                        w: 1,
                        d: 0,
                        l: 0,
                        p: 3,
                    },
                );
            }
        }

        // Draw
        if match_result == "draw" {
            if let Some(left_team_data) = data.get_mut(left_team) {
                left_team_data.mp += 1;
                left_team_data.d += 1;
                left_team_data.p += 1
            } else {
                data.insert(
                    String::from(left_team),
                    TeamDetails {
                        mp: 1,
                        w: 0,
                        d: 1,
                        l: 0,
                        p: 1,
                    },
                );
            }

            if let Some(right_team_data) = data.get_mut(right_team) {
                right_team_data.mp += 1;
                right_team_data.d += 1;
                right_team_data.p += 1
            } else {
                data.insert(
                    String::from(right_team),
                    TeamDetails {
                        mp: 1,
                        w: 0,
                        d: 1,
                        l: 0,
                        p: 1,
                    },
                );
            }
        }
    }

    let mut output = String::new();
    output += &format!("Team{}| MP |  W |  D |  L |  P\n", " ".repeat(27));

    let mut teams: Vec<(&String, &TeamDetails)> = data.iter().collect();

    // Sort teams by winners on the top
    // In case of draw, sort by alphabetically
    teams.sort_by(|a, b| b.1.p.cmp(&a.1.p).then_with(|| a.0.cmp(b.0)));

    for (team, team_data) in teams {
        let team_name_length = team.len();

        let mp = team_data.mp;
        let w = team_data.w;
        let d = team_data.d;
        let l = team_data.l;
        let p = team_data.p;

        output += &format!(
            "{team}{}| {mp:2} | {w:2} | {d:2} | {l:2} | {p:2}\n",
            " ".repeat(31 - team_name_length)
        );
    }

    // Remove last newline character
    output.pop();

    output
}

// Team                           | MP |  W |  D |  L |  P
// Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
// Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
// Blithering Badgers             |  3 |  1 |  0 |  2 |  3
// Courageous Californians        |  3 |  0 |  1 |  2 |  1

// '                           '
