// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,   // 得球数
    goals_conceded: u8, // 失球数
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap(); // 1的得球就是2的失球
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap(); // 2的得球就是1的失球

        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        // 球队名(不要重复插入覆盖) + 得分统计
        scores
            .entry(team_1_name)
            // add_modify方法: 传入闭包操作修改; or_insert_with: 不存再初始化(闭包)
            .and_modify(|team: &mut TeamScores| {
                team.goals_scored += team_1_score;
                team.goals_conceded += team_2_score;
            })
            .or_insert_with(|| TeamScores {
                // 如果不存在初始化该结构体
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            });

        scores
            .entry(team_2_name)
            .and_modify(|team: &mut TeamScores| {
                team.goals_scored += team_2_score;
                team.goals_conceded += team_1_score;
            })
            .or_insert_with(|| TeamScores {
                // 如果不存在初始化该结构体
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            });
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        // 哈希的值是个结构体struct TeamScores
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
