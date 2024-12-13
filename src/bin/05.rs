advent_of_code::solution!(5);

use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    IResult,
};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    before: usize,
    after: usize,
}

fn rule_applies(rule: &Rule, pages: &HashMap<usize, usize>) -> bool {
    // True if the page number before is less than the page number after
    // or if either page number is missing
    pages.get(&rule.before).is_none()
        || pages.get(&rule.after).is_none()
        || pages.get(&rule.before) < pages.get(&rule.after)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (before, after)) = separated_pair(digit1, tag("|"), digit1)(input)?;
    Ok((
        input,
        Rule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        },
    ))
}

fn parse_all_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    // Parse a list of rules
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_pages(input: &str) -> IResult<&str, HashMap<usize, usize>> {
    // Parse a line of comma separates usizes into a hasmap of the page numbers and their positions
    // in the list
    let (input, pages) = separated_list1(tag(","), digit1)(input)?;
    Ok((
        input,
        pages
            .into_iter()
            .enumerate()
            // convert into usize
            .map(|(i, page)| (page.parse().unwrap(), i))
            .collect(),
    ))
}

fn parse_all_pages(input: &str) -> IResult<&str, Vec<HashMap<usize, usize>>> {
    // Parse a list of pages
    separated_list1(line_ending, parse_pages)(input)
}

fn middle_page(pages: &HashMap<usize, usize>) -> usize {
    // Find the page number that is in the middle of the list
    let mut pages = pages.iter().collect::<Vec<_>>();
    pages.sort_by_key(|(_, &pos)| pos);
    *pages[pages.len() / 2].0
}

fn read_all_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<HashMap<usize, usize>>)> {
    // Parse the rules and the pages
    let (input, rules) = parse_all_rules(input)?;
    // Blank line
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, pages) = parse_all_pages(input)?;
    Ok((input, (rules, pages)))
}

fn fix_page_order_according_to_rules(
    rules: &[Rule],
    pages: &mut HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    // Fix the page order according to the rules
    let mut pages = pages.clone();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
            if pages.get(&rule.before) > pages.get(&rule.after) {
                // Just try swapping page numbesr
                let val1 = pages.get(&rule.before).copied();
                let val2 = pages.get(&rule.after).copied();
                if let (Some(val1), Some(val2)) = (val1, val2) {
                    pages.insert(rule.before, val2);
                    pages.insert(rule.after, val1);
                    changed = true;
                }
            }
        }
    }
    pages
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (rules, all_pages)) = read_all_input(input).unwrap();
    let result: usize = all_pages
        .iter()
        .filter(|pages| rules.iter().all(|rule| rule_applies(rule, pages)))
        .map(middle_page)
        .sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (rules, all_pages)) = read_all_input(input).unwrap();
    let result: usize = all_pages
        .iter()
        .filter(|pages| !rules.iter().all(|rule| rule_applies(rule, pages)))
        .map(|pages| fix_page_order_according_to_rules(&rules, &mut pages.clone()))
        .map(|pages| middle_page(&pages))
        .sum();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_read_all_rules() {
        let input = "1|2\n3|4";
        let result = parse_all_rules(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Rule {
                        before: 1,
                        after: 2
                    },
                    Rule {
                        before: 3,
                        after: 4
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_read_all_pages() {
        let input = "1,2\n3,4";
        let result = parse_all_pages(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    vec![(1, 0), (2, 1)].into_iter().collect(),
                    vec![(3, 0), (4, 1)].into_iter().collect()
                ]
            ))
        );
    }

    #[test]
    fn test_read_all_input() {
        let input = "1|2\n3|4\n\n1,2\n3,4";
        let result = read_all_input(input);
        assert_eq!(
            result,
            Ok((
                "",
                (
                    vec![
                        Rule {
                            before: 1,
                            after: 2
                        },
                        Rule {
                            before: 3,
                            after: 4
                        }
                    ],
                    vec![
                        vec![(1, 0), (2, 1)].into_iter().collect(),
                        vec![(3, 0), (4, 1)].into_iter().collect()
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_rule_applies() {
        let rule = Rule {
            before: 1,
            after: 2,
        };
        let pages = vec![(1, 0), (2, 1)].into_iter().collect();
        assert!(rule_applies(&rule, &pages));
    }

    #[test]
    fn test_rule_applies_if_pages_missing() {
        let rule = Rule {
            before: 1,
            after: 2,
        };
        let pages = vec![(1, 0), (6, 1)].into_iter().collect();
        assert!(rule_applies(&rule, &pages));
    }
}
