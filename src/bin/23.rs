advent_of_code::solution!(23);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
struct PC {
    name: String,
    connections: HashSet<String>,
}

impl PC {
    fn add_connection(&mut self, connection: String) {
        self.connections.insert(connection);
    }
}

impl Hash for PC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for PC {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn input_to_pcs(input: &str) -> HashSet<PC> {
    let mut pcs: HashSet<PC> = HashSet::new();
    for line in input.lines() {
        let parts = line.split("-").collect::<Vec<&str>>();
        let left = parts[0].to_string();
        let right = parts[1].to_string();
        let mut pc = PC {
            name: left.clone(),
            connections: HashSet::new(),
        };
        if let Some(existing_pc) = pcs.take(&pc) {
            pc.connections = existing_pc.connections;
        }
        pc.add_connection(right.clone());
        pcs.insert(pc);

        // Now need to add the connection in the other direction
        let mut pc = PC {
            name: right.clone(),
            connections: HashSet::new(),
        };
        if let Some(existing_pc) = pcs.take(&pc) {
            pc.connections = existing_pc.connections;
        }
        pc.add_connection(left.clone());
        pcs.insert(pc);
    }
    pcs
}

#[derive(Clone, Debug, Eq)]
struct Triplet {
    pc1: PC,
    pc2: PC,
    pc3: PC,
}

impl Triplet {
    fn new(pc1: PC, pc2: PC, pc3: PC) -> Self {
        Self { pc1, pc2, pc3 }
    }
    fn contains_t(&self) -> bool {
        self.pc1.name.starts_with('t')
            || self.pc2.name.starts_with('t')
            || self.pc3.name.starts_with('t')
    }
}
impl Hash for Triplet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We don't care about the order, so we need to sort the PCs
        let mut pcs = vec![&self.pc1, &self.pc2, &self.pc3];
        pcs.sort_by(|a, b| a.name.cmp(&b.name));
        pcs.hash(state);
    }
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Self) -> bool {
        let mut self_pcs = vec![&self.pc1, &self.pc2, &self.pc3];
        let mut other_pcs = vec![&other.pc1, &other.pc2, &other.pc3];
        self_pcs.sort_by(|a, b| a.name.cmp(&b.name));
        other_pcs.sort_by(|a, b| a.name.cmp(&b.name));
        self_pcs == other_pcs
    }
}

fn find_triplets(pcs: &HashSet<PC>) -> HashSet<Triplet> {
    // Given a set of PCs, find all sets of pcs that are connected to 2 other pcs that are
    // connected to each other
    let mut triplets: HashSet<Triplet> = HashSet::new();
    for pc in pcs {
        for connection_1 in pc.connections.clone() {
            let c1 = pcs
                .get(&PC {
                    name: connection_1.clone(),
                    connections: HashSet::new(),
                })
                .expect("PC not found");
            for connection_2 in pc.connections.clone() {
                let c2 = pcs
                    .get(&PC {
                        name: connection_2.clone(),
                        connections: HashSet::new(),
                    })
                    .expect("PC not found");
                if c1 == c2 {
                    continue;
                }
                if c1.connections.contains(&c2.name) {
                    if pc.name == "aq" {
                        println!("Found triplet:");
                        println!("{:?}", pc);
                        println!("{:?}", c1);
                        println!("{:?}", c2);
                    }
                    let triplet = Triplet::new(pc.clone(), c1.clone(), c2.clone());
                    triplets.insert(triplet);
                }
            }
        }
    }
    triplets
}

#[derive(Clone, Debug, Eq)]
struct Lan {
    pcs: HashSet<PC>,
}

impl Lan {
    fn size(&self) -> usize {
        self.pcs.len()
    }
    fn code(&self) -> String {
        let mut pcs = self.pcs.iter().collect::<Vec<&PC>>();
        pcs.sort_by(|a, b| a.name.cmp(&b.name));
        pcs.iter()
            .map(|pc| pc.name.clone())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn all_connected(&self) -> bool {
        // Check if all PCs are connected to each other
        for pc in self.pcs.iter() {
            // For every PC, we want to make sure that all other PCs are connected to it
            for other_pc in self.pcs.iter() {
                if pc == other_pc {
                    continue;
                }
                if !pc.connections.contains(&other_pc.name) {
                    return false;
                }
            }
        }
        true
    }
}

// debug for LAN that just prints PC names
impl std::fmt::Display for Lan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut pcs = self.pcs.iter().collect::<Vec<&PC>>();
        pcs.sort_by(|a, b| a.name.cmp(&b.name));
        write!(
            f,
            "{}",
            pcs.iter()
                .map(|pc| pc.name.clone())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Hash for Lan {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pcs = self.pcs.iter().collect::<Vec<&PC>>();
        pcs.sort_by(|a, b| a.name.cmp(&b.name));
        pcs.hash(state);
    }
}

impl PartialEq for Lan {
    fn eq(&self, other: &Self) -> bool {
        let mut self_pcs = self.pcs.iter().collect::<Vec<&PC>>();
        let mut other_pcs = other.pcs.iter().collect::<Vec<&PC>>();
        self_pcs.sort_by(|a, b| a.name.cmp(&b.name));
        other_pcs.sort_by(|a, b| a.name.cmp(&b.name));
        self_pcs == other_pcs
    }
}

fn generate_power_set(pcs: &HashSet<String>) -> Vec<HashSet<String>> {
    let mut power_set: Vec<HashSet<String>> = Vec::new();
    for i in 0..=pcs.len() {
        for subset in pcs.iter().combinations(i) {
            let mut set = HashSet::new();
            for pc in subset {
                set.insert(pc.clone());
            }
            power_set.push(set);
        }
    }
    power_set
}

fn find_largest_lan(pcs: &HashSet<PC>) -> Lan {
    // Given some PCs, find the largest LAN that PC is connected to
    // A LAN is a set of PCs where each PC is in the LAN is connected directly to
    // every other PC in the LAN
    let mut lans: HashSet<Lan> = HashSet::new();
    for pc in pcs {
        // Make a trial LAN out of every combination of PCs we are connected to
        let mut set = pc.connections.clone();
        set.insert(pc.name.clone());

        // The Power set is all combinations of PC names from the original connections

        let power_set = generate_power_set(&set);
        // Sort the power set by size with biggest first
        let mut sorted_power_set = power_set.clone();
        sorted_power_set.sort_by_key(|set| set.len());
        sorted_power_set.reverse();

        for set in sorted_power_set {
            let mut lan = Lan {
                pcs: HashSet::new(),
            };
            for pc_name in set {
                let pc = pcs
                    .get(&PC {
                        name: pc_name.clone(),
                        connections: HashSet::new(),
                    })
                    .expect("PC not found");
                lan.pcs.insert(pc.clone());
            }
            if lan.all_connected() {
                // The first one we find will be the largest
                lans.insert(lan);
                break;
            }
        }
    }
    // Now return the LAN with the most PCs
    lans.iter()
        .max_by_key(|lan| lan.size())
        .expect("No LANs found")
        .clone()
}

pub fn part_one(input: &str) -> Option<usize> {
    let pcs = input_to_pcs(input);
    let triplets = find_triplets(&pcs);
    Some(triplets.iter().filter(|t| t.contains_t()).count())
}

pub fn part_two(input: &str) -> Option<String> {
    let pcs = input_to_pcs(input);
    let lan = find_largest_lan(&pcs);
    Some(lan.code())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
