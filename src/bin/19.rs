advent_of_code::solution!(19);

use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    remaining: String,
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.split("\n\n");
    let towels = parts.next().unwrap().split(", ").collect();
    let patterns = parts.next().unwrap().lines().collect();
    (towels, patterns)
}

fn successors(node: &Node, towels: &[&str]) -> Vec<(Node, usize)> {
    let mut result = Vec::new();
    for towel in towels {
        if node.remaining.starts_with(towel) {
            result.push((
                Node {
                    remaining: node.remaining[towel.len()..].to_owned(),
                },
                1,
            ));
        }
    }
    result
}

fn count_paths(current: &Node, towels: &[&str], cache: &mut HashMap<Node, usize>) -> usize {
    if current.remaining.is_empty() {
        return 1;
    }
    if cache.contains_key(current) {
        return *cache.get(current).unwrap();
    }
    let count = successors(current, towels)
        .iter()
        .map(|(node, _)| count_paths(node, towels, cache))
        .sum();
    cache.insert(current.clone(), count);
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    Some(
        patterns
            .iter()
            .map(|pattern| {
                count_paths(
                    &Node {
                        remaining: pattern.to_string(),
                    },
                    &towels,
                    &mut HashMap::new(),
                )
            })
            .filter(|l| l > &0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    Some(
        patterns
            .iter()
            .map(|pattern| {
                count_paths(
                    &Node {
                        remaining: pattern.to_string(),
                    },
                    &towels,
                    &mut HashMap::new(),
                )
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_specific_example() {
        let towels = "bb, rgub, ubub, ugwbg, rwbb, rg, guuwbrw, gur, rrgb, bwgrbgg, ugg, wgu, ugbrbb, bbu, ubgw, guw, gubwu, grwr, rgbb, wrurw, rw, urww, uug, ugwu, ubr, rgwwbbb, urur, rwuwbwrr, gbbbburw, uwruu, rrrr, ubwgbbb, wrwr, uuu, uurubuuw, uwgw, gbbgu, wwbu, wuwr, brrug, bggggb, bgw, rrbr, wwuu, ubu, rggrub, rrgr, uwu, uuwguu, gbbbu, wuubb, rgb, bggwwr, g, gbw, wuu, bug, br, gwg, gbb, wgwrrr, rbu, bwwu, buubruwg, gwbub, bbrbw, wwwgb, wbwgw, www, rrw, uggwb, wubgw, gugrr, gruuwur, rrbgrg, bbwg, bgg, wgg, wuw, uugggrbg, wgrruu, uwuwrbw, wbbr, uuburr, wbg, ggbwurug, uubrwu, urug, gururb, bgbg, rurwg, brr, rbrg, bbubuu, rwrbw, uwgbwurb, wbuwb, bbw, rrrgbgb, ru, ugwru, bggrw, gggwbu, ggwuwu, rru, wrrug, rrrgrub, bwbww, guuu, wub, gu, buwbu, rwwww, wugbuwb, bgrr, uguuu, gbub, bww, grb, bwg, wwb, uurw, urwuu, grrbgu, rwg, rrbwb, bwu, wwug, ubwub, wrbg, wgrw, gww, rww, bbgrb, ub, rggbwu, bbuwbrgw, rb, wb, bwbugg, gurw, bubb, guug, wr, wrwg, uwgwbbr, bubwbrw, wru, rrbrbub, rgrg, rwgbrwb, rwbr, uwwrbbu, uwrbgw, gwub, uwbgr, ruugrrub, rubguggr, buuw, wwu, gbuub, gurg, wrg, bwub, rub, bgu, wbwrbb, ggb, bwbb, wwggbw, wgb, uwr, uwgur, bwb, u, urw, bgb, bbr, uwwu, bg, urr, rgbu, bugwg, rbbr, burb, bgruw, bwuu, rrugrgg, wrwubw, bubbgbg, bguwuru, ubg, ubwu, bwbbgbub, brguu, urwgrbu, guurrw, ubrgr, uubuwr, wrw, uuwr, ugur, ubwuwg, wbrw, wgr, rguuw, rwgwu, rbbgb, ggbgg, rgu, ug, gwrb, rrwgwr, rubuu, ugb, grrbr, grwb, bgugwwur, bbugruu, wbwg, ggu, guwbbw, bgr, grrrgw, brwwbgu, wburb, rgbg, bw, ruwb, brb, wwgbgwr, rbr, grr, ugbb, ruruu, wgbr, rwwbwg, wwg, ubgwu, urru, guuuwu, ururr, buuub, bru, bwrrwubu, wuww, rgbr, gub, wwrbg, grw, grg, wu, burguu, gbug, uggbr, brwub, uwb, grgru, rur, rgugugg, rubrbwg, rbrrw, gwrurgr, bur, uurbwu, ggw, rbwbu, wbw, uuw, ugu, bwr, wwwwru, gbwg, gbu, rgrwb, urbuw, rugwr, uwbbgrw, uurrbugg, wbgbrg, ubb, urb, burrb, gbgu, rwu, bbrruwb, uurrru, bugbw, b, ugguuw, uwgugwb, rug, wug, bwurgr, wgwubu, rgr, ugwb, gwwwgu, gwwbgggw, bwug, rbrug, uurb, ruw, wwwrb, brw, wggbubu, ugbrr, bbrgg, rruw, ggr, gbbw, wguwug, rbb, wuugbwgg, gug, rbuguu, gwb, ubrr, burrrb, rrwrrrg, ubw, rwgrrru, wugb, gggbbwg, uur, bu, urg, bgrb, brgrb, rbbwu, rgruu, gugruurr, bgbwb, rrbg, ruub, bgggrb, rugr, ggwb, uw, brrgr, wwugbrr, uruwub, wg, rwbugu, gru, bbrurr, rgbug, rrr, burw, gwgrubw, ww, wrburbb, rwwrwbu, uu, uww, urrw, wwwrbu, ugbugr, bbbb, wbuu, gbg, rrwuwb, rwgugg, ggrg, bub, gwr, rrwbw, rwr, rgg, r, guu, rrg, uggbbggr, wggbbb, wbuuwub, buwbw, ugrgu, gbbrr, gubgguu, bubwbg, ububw, uruw, wbr, gbwgrg, guwu, bwwur, rr, bggr, gg, rrwu, wrb, ruu, wbgb, ruwbwbuw, wbb, rwuug, rrb, ugw, bwbwuruu, ur, rbbbrrr, rgwg, gbr, grrb, wurr, uruwbr, burbww, ugrguw, wgw, wrgwug, buu, wbu, gubw, rgbgu, gbrrgg, buwb, wwr, bgwwwr, bbbug, rwwg, uuwgwbw, uwg, buw, gbbu, bbbw, bgrru, gwbu, ubwg, brg, wrr, ggg, uru, ugr, rwrww, bwgbbr, bwuw, brbgrrg, brrr, bwrb, gbgggu, gr, brubw, rwb, rbgr, bbb, gwu, wur";
        let towels = towels.split(", ").collect::<Vec<&str>>();
        let pattern = "uuuuguurgubrburwwwuruuruuuwurbbrwrwgrrbwrbbbggbbbuggrubgrrgw".to_owned();
        let count = count_paths(&Node { remaining: pattern }, &towels, &mut HashMap::new());
        assert_eq!(count, 0);
    }
}
