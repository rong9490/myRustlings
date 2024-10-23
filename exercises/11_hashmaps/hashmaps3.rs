use std::{
    collections::HashMap,
    str::{Lines, Split},
};

#[derive(Default)]
pub struct TeamScores {
    pub goals_scored: u8,   // 得球数
    pub goals_conceded: u8, // 失球数
}

// HACK 考虑使用 &str, 而非String
pub fn parse_lines(results: &str) -> Vec<(String, String, u8, u8)> {
    results
        // 先分行
        .lines()
        // 再Map迭代器分隔, 内层收集成Vec<&str>
        .map(|line| {
            let strs: Vec<&str> = line.split(',').collect::<Vec<&str>>();
            // 拿到四项
            let team_1_name: String = strs[0].to_string();
            let team_2_name: String = strs[1].to_string();
            let team_1_score: u8 = strs[2].parse().unwrap();
            let team_2_score: u8 = strs[3].parse().unwrap();
            // 构成四项元组
            (team_1_name, team_2_name, team_1_score, team_2_score)
        })
        // 外层再collect成容器
        .collect::<Vec<(String, String, u8, u8)>>()
}

pub fn build_scores_table(lines: Vec<(String, String, u8, u8)>) -> HashMap<String, TeamScores> {
    // 统计每个球队的(得球,失球数)
    let mut scores: HashMap<String, TeamScores> = HashMap::new();

    let mut scores2: HashMap<&str, TeamScores> = HashMap::new();

    // 外面遍历一行
    lines
        .into_iter()
        .for_each(|(team_1_name, team_2_name, team_1_score, team_2_score)| {
            scores
                .entry(team_1_name.clone())
                .and_modify(|team| {
                    team.goals_scored += team_1_score;
                    team.goals_conceded += team_2_score;
                })
                .or_insert(TeamScores {
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
        });

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // 比赛结果字符串, 需要解析
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    /// 测试字符串的解析(二维vec容器)
    #[test]
    fn parse_lines_test() {
        let lines: Vec<(String, String, u8, u8)> = parse_lines(RESULTS);
        assert_eq!(lines.len(), 5);
        assert_eq!(
            lines[0],
            (String::from("England"), String::from("France"), 4, 2)
        );
        assert_eq!(
            lines[1],
            (String::from("France"), String::from("Italy"), 3, 1)
        );
        assert_eq!(
            lines[2],
            (String::from("Poland"), String::from("Spain"), 2, 0)
        );
        assert_eq!(
            lines[3],
            (String::from("Germany"), String::from("England"), 2, 1)
        );
        assert_eq!(
            lines[4],
            (String::from("England"), String::from("Spain"), 1, 0)
        );
    }

    #[test]
    fn build_scores() {
        let lines: Vec<(String, String, u8, u8)> = parse_lines(RESULTS);
        let scores: HashMap<String, TeamScores> = build_scores_table(lines);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let lines: Vec<(String, String, u8, u8)> = parse_lines(RESULTS);
        let scores: HashMap<String, TeamScores> = build_scores_table(lines);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let lines: Vec<(String, String, u8, u8)> = parse_lines(RESULTS);
        let scores: HashMap<String, TeamScores> = build_scores_table(lines);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}


