use std::{collections::BTreeMap, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
struct Valve<'a> {
    id: &'a str,
    flow_rate: u64,
    tunnels: Vec<&'a str>,
}

impl PartialOrd for Valve<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Valve<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(other.id)
    }
}

mod parsers {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, alpha1, newline},
        multi::separated_list1,
        sequence::{preceded, tuple},
        IResult, Parser,
    };

    use crate::Valve;

    pub(crate) fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
        separated_list1(
            newline,
            tuple((
                preceded(tag("Valve "), alpha1),
                preceded(tag(" has flow rate="), complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), alpha1),
                ),
            ))
            .map(|(id, flow_rate, tunnels)| Valve {
                id,
                flow_rate,
                tunnels,
            }),
        )(input)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Valves(u64);

impl Valves {
    fn pressure_per_tick(&self, working_valves: &Vec<&Valve>) -> u64 {
        let mut pressure = 0;
        (0..working_valves.len()).for_each(|i| {
            if self.is_open(i) {
                pressure += working_valves[i].flow_rate;
            }
        });
        pressure
    }

    fn close(&self, idx: usize) -> Self {
        Self(self.0 & !(1 << idx))
    }

    fn is_open(&self, idx: usize) -> bool {
        self.0 & (1 << idx) != 0
    }
}

fn max_pressure_dp(valves: &Vec<Valve>, time: usize) -> u64 {
    let mut dp: Vec<BTreeMap<(&str, Valves), u64>> = vec![];
    for _ in 0..=time {
        dp.push(BTreeMap::new());
    }
    dp[1].insert(("AA", Valves(0)), 0);

    let working_valves: Vec<_> = valves.iter().filter(|valve| valve.flow_rate > 0).collect();
    let number_of_valve_permutations = 2u64.pow(working_valves.len() as u32);
    let working_valve_indexes = working_valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.id, i))
        .collect::<BTreeMap<_, _>>();

    let mut connected_from: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for valve in valves {
        for &tunnel in &valve.tunnels {
            connected_from.entry(tunnel).or_default().push(valve.id);
        }
    }

    for min in 2..=time {
        println!("Starting minute {}", min);
        for current in valves {
            for valves in 0..number_of_valve_permutations {
                let valves = Valves(valves);
                let pressure_per_tick = valves.pressure_per_tick(&working_valves);

                // Either moved from another location
                for &src in &connected_from[current.id] {
                    if let Some(&prior_pressure) = dp[min - 1].get(&(src, valves)) {
                        let pressure = dp[min].entry((current.id, valves)).or_default();
                        *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                    }
                }

                // or opened this valve if it was closed
                if let Some(working_valve_idx) = working_valve_indexes.get(current.id) {
                    if valves.is_open(*working_valve_idx) {
                        let valves_when_closed = valves.close(*working_valve_idx);
                        if let Some(&prior_pressure) =
                            dp[min - 1].get(&(current.id, valves_when_closed))
                        {
                            let pressure = dp[min].entry((current.id, valves)).or_default();
                            *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                        }
                    }
                }
            }
        }
    }

    dp[time]
        .iter()
        .map(|(_, &pressure)| pressure)
        .max()
        .unwrap_or(0)
}

fn max_pressure_dp_with_partner(valves: &Vec<Valve>, time: usize) -> u64 {
    let mut dp: Vec<BTreeMap<((&str, &str), Valves), u64>> = vec![];
    for _ in 0..=time {
        dp.push(BTreeMap::new());
    }
    dp[1].insert((("AA", "AA"), Valves(0)), 0);

    let working_valves: Vec<_> = valves.iter().filter(|valve| valve.flow_rate > 0).collect();
    let number_of_valve_permutations = 2u64.pow(working_valves.len() as u32);
    let working_valve_indexes = working_valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.id, i))
        .collect::<BTreeMap<_, _>>();

    let mut connected_from: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for valve in valves {
        for &tunnel in &valve.tunnels {
            connected_from.entry(tunnel).or_default().push(valve.id);
        }
    }

    for min in 2..=time {
        println!("Starting minute {}", min);
        for my_current in valves {
            let my_valve_idx = working_valve_indexes.get(my_current.id);
            for partner_current in valves {
                let partner_valve_idx = working_valve_indexes.get(partner_current.id);
                for valves in 0..number_of_valve_permutations {
                    let valves = Valves(valves);
                    let pressure_per_tick = valves.pressure_per_tick(&working_valves);

                    // Either we both moved to this location

                    for &src in &connected_from[my_current.id] {
                        for &partner_src in &connected_from[partner_current.id] {
                            if let Some(&prior_pressure) =
                                dp[min - 1].get(&((src, partner_src), valves))
                            {
                                let pressure = dp[min]
                                    .entry(((my_current.id, partner_current.id), valves))
                                    .or_default();
                                *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                            }
                        }
                    }

                    // Or I moved and partner opened valve
                    if let Some(working_valve_idx) = partner_valve_idx {
                        if valves.is_open(*working_valve_idx) {
                            let valves_when_closed = valves.close(*working_valve_idx);
                            for &src in &connected_from[my_current.id] {
                                if let Some(&prior_pressure) = dp[min - 1]
                                    .get(&((src, partner_current.id), valves_when_closed))
                                {
                                    let pressure = dp[min]
                                        .entry(((my_current.id, partner_current.id), valves))
                                        .or_default();
                                    *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                                }
                            }
                        }
                    }

                    // or partner moved and I opened valve
                    if let Some(working_valve_idx) = my_valve_idx {
                        if valves.is_open(*working_valve_idx) {
                            let valves_when_closed = valves.close(*working_valve_idx);
                            for &partner_src in &connected_from[partner_current.id] {
                                if let Some(&prior_pressure) = dp[min - 1]
                                    .get(&((my_current.id, partner_src), valves_when_closed))
                                {
                                    let pressure = dp[min]
                                        .entry(((my_current.id, partner_current.id), valves))
                                        .or_default();
                                    *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                                }
                            }
                        }
                    }

                    // or we both opened the valve if it was closed
                    if let (Some(working_valve_idx), Some(partner_working_valve_idx)) =
                        (my_valve_idx, partner_valve_idx)
                    {
                        if valves.is_open(*working_valve_idx)
                            && valves.is_open(*partner_working_valve_idx)
                        {
                            let valves_when_closed = valves
                                .close(*working_valve_idx)
                                .close(*partner_working_valve_idx);

                            if let Some(&prior_pressure) = dp[min - 1]
                                .get(&((my_current.id, partner_current.id), valves_when_closed))
                            {
                                let pressure = dp[min]
                                    .entry(((my_current.id, partner_current.id), valves))
                                    .or_default();
                                *pressure = (*pressure).max(prior_pressure + pressure_per_tick);
                            }
                        }
                    }
                }
            }
        }
    }

    dp[time]
        .iter()
        .map(|(_, &pressure)| pressure)
        .max()
        .unwrap_or(0)
}

fn part1(input: &str) -> u64 {
    let valves = parsers::parse_input(input).unwrap().1;
    max_pressure_dp(&valves, 30)
}

fn part2(input: &str) -> u64 {
    let valves = parsers::parse_input(input).unwrap().1;
    max_pressure_dp_with_partner(&valves, 26)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1651);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 1707);
    }
}
