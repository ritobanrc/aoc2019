use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    reactants: Vec<(String, u64)>,
    product: (String, u64),
}

#[aoc_generator(day14)]
fn parse_reactions(input: &str) -> Vec<Reaction> {
    let mut rxns = Vec::new();

    for line in input.lines() {
        let line: Vec<_> = line.split(' ').collect();
        let mut rxn = Reaction {
            reactants: Vec::new(),
            product: (String::from(""), 0),
        };
        let mut i = 0usize;
        while i < line.len() {
            if line[i] == "=>" {
                rxn.product = (String::from(line[i + 2]), line[i + 1].parse().unwrap());
                break;
            }
            let reactant = line[i + 1].trim_end_matches(',');
            rxn.reactants
                .push((String::from(reactant), line[i].parse().unwrap()));
            i += 2;
        }

        rxns.push(rxn);
    }

    rxns
}

fn ore_for_fuel(rxns: &[Reaction], fuel: u64) -> u64 {
    let mut need = HashMap::new();

    let mut excess: HashMap<String, u64> = HashMap::new();

    need.insert(String::from("FUEL"), fuel);

    let mut ore = 0;

    while !need.is_empty() {
        //println!("Need {:?}", need);
        //println!("Excess {:?}", excess);
        let product = need.keys().next().unwrap().clone();
        let mut quantity = need.remove(&product).unwrap();

        //println!("Product: {:?} Quantity: {:?}", product, quantity);

        if product == "ORE" {
            ore += quantity;
        }

        if excess.contains_key(&product) {
            if excess[&product] > quantity {
                *excess.get_mut(&product).unwrap() -= quantity;
                continue;
            } else {
                quantity -= excess[&product];
                excess.remove(&product);
            }
        }

        for rxn in rxns {
            if *product == rxn.product.0 {
                let scale = ceil_div(quantity, rxn.product.1);

                let excess_amount = rxn.product.1 * scale - quantity;

                for reactant in &rxn.reactants {
                    let amount = reactant.1 * scale;

                    need.entry(reactant.0.clone())
                        .and_modify(|q| *q += amount)
                        .or_insert(amount);
                }
                if excess_amount != 0 {
                    excess
                        .entry(product.clone())
                        .and_modify(|q| *q += excess_amount)
                        .or_insert(excess_amount);
                }
            }
        }
    }

    ore
}

#[aoc(day14, part1)]
fn solve_p1(rxns: &[Reaction]) -> u64 {
    ore_for_fuel(rxns, 1)
}

#[aoc(day14, part2)]
fn solve_p2(rxns: &[Reaction]) -> u64 {
    let mut lower = 1; // we can definately produce 1 fuel
    let mut upper = 1_000_000_000_000; // random big number

    // perform binary search
    while lower <= upper {
        let midpoint = (lower + upper) / 2;
        let ore = ore_for_fuel(rxns, midpoint);
        if ore < 1_000_000_000_000 {
            lower = midpoint + 1;
        } else if ore > 1_000_000_000_000 {
            upper = midpoint - 1;
        } else {
            return midpoint;
        }
    }

    lower - 1
}

fn ceil_div(x: u64, y: u64) -> u64 {
    x / y + if x % y != 0 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_sample1() {
        let rxns = parse_reactions(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );

        assert_eq!(solve_p1(&rxns), 31);
    }

    #[test]
    fn day14_sample2() {
        let rxns = parse_reactions(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        );

        assert_eq!(solve_p1(&rxns), 165);
    }

    #[test]
    fn day14_sample3() {
        let rxns = parse_reactions(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );

        assert_eq!(solve_p1(&rxns), 13312);
    }

    #[test]
    fn day14_sample4() {
        let rxns = parse_reactions(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );

        assert_eq!(solve_p1(&rxns), 180697);
    }

    #[test]
    fn day14_sample5() {
        let rxns = parse_reactions(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );

        assert_eq!(solve_p1(&rxns), 2210736);
    }

    #[test]
    fn day14_sample3_p2() {
        let rxns = parse_reactions(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );

        assert_eq!(solve_p2(&rxns), 82892753);
    }

    #[test]
    fn day14_sample4_p2() {
        let rxns = parse_reactions(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );

        assert_eq!(solve_p2(&rxns), 5586022);
    }

    #[test]
    fn day14_sample5_p2() {
        let rxns = parse_reactions(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );

        assert_eq!(solve_p2(&rxns), 460664);
    }
}
