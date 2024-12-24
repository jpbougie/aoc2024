// DISCLAIMER: this only works on my input, since I did a manual swap from graph analysis instead of coding it

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gate {
    op: Op,
    a: String,
    b: String,
    out: String,
    tag: Option<(T, usize, usize)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum T {
    Carry,
    Intermediate,
    Out,
}

impl Gate {
    fn apply(&self, values: &mut Values) -> bool {
        let Some(a) = values.get(&self.a) else {
            return false;
        };

        let Some(b) = values.get(&self.b) else {
            return false;
        };

        values.insert(
            self.out.clone(),
            match self.op {
                Op::And => *a && *b,
                Op::Or => *a || *b,
                Op::Xor => *a != *b,
            },
        );

        true
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

type Values = HashMap<String, bool>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Logic {
    values: Values,
    gates: Vec<Gate>,
}

impl Logic {
    fn swap(&mut self, a: &str, b: &str) {
        for g in self.gates.iter_mut() {
            if g.out == a {
                g.out = b.to_string();
            } else if g.out == b {
                g.out = a.to_string();
            }
        }
    }
}

fn part1(mut logic: Logic) -> u64 {
    let mut unresolved_gates = logic.gates.clone();
    while !unresolved_gates.is_empty() {
        unresolved_gates.retain(|gate| !gate.apply(&mut logic.values));
    }

    make_num(&logic.values, 'z')
}

fn run(logic: &mut Logic) {
    let mut unresolved_gates = logic.gates.clone();
    while !unresolved_gates.is_empty() {
        unresolved_gates.retain(|gate| !gate.apply(&mut logic.values));
    }
}

fn tag0(logic: &mut Logic) -> bool {
    let mut tag_or = false;
    let mut tag_and = false;
    for g in logic.gates.iter_mut() {
        if !tag_or && g.op == Op::Or && (g.a == "x00" && g.b == "y00")
            || (g.a == "y00" && g.b == "x00")
        {
            g.tag = Some((T::Out, 0, 0));
            tag_or = true;
        }

        if !tag_and && g.op == Op::And && (g.a == "x00" && g.b == "y00")
            || (g.a == "y00" && g.b == "x00")
        {
            g.tag = Some((T::Carry, 0, 0));
            tag_and = true;
        }
    }

    tag_or && tag_and
}

#[derive(Debug, PartialEq, Eq)]
enum Out {
    Found,
    Swapped(String, String),
    Failed,
}

fn tagn(logic: &mut Logic, n: usize) -> Out {
    let xn = format!("x{n:02}");
    let yn = format!("y{n:02}");
    let zn = format!("z{n:02}");

    let Some(prev_carry) = logic
        .gates
        .iter()
        .find(|g| g.tag == Some((T::Carry, n - 1, 0)))
        .map(|g| g.out.clone())
    else {
        panic!("No previous carry for {n}");
    };

    println!("Previous carry is {}", prev_carry);

    // Given that we have tagged the previous carry, try to find the following gates:
    let mut a = None;
    // an intermediate that xors the 2 basic values
    // i01x = x01 XOR y01
    for g in logic.gates.iter_mut() {
        if let Some(ref t) = g.tag {
            if *t == (T::Intermediate, n, 0) {
                a = Some(g.out.clone());
                break;
            }
            continue;
        }
        if g.op == Op::Xor && ((g.a == xn && g.b == yn) || (g.a == yn && g.b == xn)) {
            g.tag = Some((T::Intermediate, n, 0));
            a = Some(g.out.clone());
            break;
        }
    }
    if a.is_none() {
        panic!("Could not find a XOR intermediate for {n}");
    }
    println!("a: {:?}", a);

    let mut b = None;
    // an intermediate that ands the previous carry with the intermediate
    // i01a = y01 AND x01
    for g in logic.gates.iter_mut() {
        if let Some(ref t) = g.tag {
            if *t == (T::Intermediate, n, 1) {
                b = Some(g.out.clone());
                break;
            }
            continue;
        }
        // if g.out.starts_with('z') {
        //     continue;
        // }
        if g.op == Op::And && ((g.a == xn && g.b == yn) || (g.a == yn && g.b == xn)) {
            g.tag = Some((T::Intermediate, n, 1));
            b = Some(g.out.clone());
            break;
        }
    }
    if b.is_none() {
        panic!("Could not find intermediate {xn} AND {yn}");
    }
    println!("b: {:?}", b);

    // z01 = (i01x XOR c00)
    let mut d = None;
    for g in logic.gates.iter_mut() {
        if g.op == Op::Xor
            && g.out == zn
            && ((g.a == prev_carry && &g.b == a.as_ref().unwrap())
                || (&g.a == a.as_ref().unwrap() && g.b == prev_carry))
        {
            g.tag = Some((T::Out, n, 0));
            d = Some(g.out.clone());
            break;
        }
    }
    if d.is_none() {
        for g in logic.gates.iter_mut() {
            if g.tag.is_none()
                && g.op == Op::Xor
                && ((g.a == prev_carry && &g.b == a.as_ref().unwrap())
                    || (&g.a == a.as_ref().unwrap() && g.b == prev_carry))
            {
                g.tag = Some((T::Out, n, 0));
                d = Some(g.out.clone());
                break;
            }
        }
        if let Some(d) = d {
            println!("SWAPPED {zn} with {:?}", d);
            logic.swap(&zn, &d);

            return Out::Swapped(zn, d);
        } else {
            // Since we can't find a simple
        }
    }
    println!("d: {:?}", d);

    let mut c = None;
    // an intermediate that ands the previous carry and the xor intermediate
    // i01b = c00 AND i01x
    for g in logic.gates.iter_mut() {
        if let Some(ref t) = g.tag {
            if *t == (T::Intermediate, n, 2) {
                c = Some(g.out.clone());
                break;
            }
            continue;
        }
        if g.out != zn
            && g.op == Op::And
            && ((g.a == prev_carry && &g.b == a.as_ref().unwrap())
                || (&g.a == a.as_ref().unwrap() && g.b == prev_carry))
        {
            g.tag = Some((T::Intermediate, n, 2));
            c = Some(g.out.clone());
            break;
        }
    }
    if c.is_none() {
        for g in logic.gates.iter_mut() {
            if let Some(ref _t) = g.tag {
                continue;
            }
            if g.op == Op::And
                && ((g.a == prev_carry && &g.b == a.as_ref().unwrap())
                    || (&g.a == a.as_ref().unwrap() && g.b == prev_carry))
            {
                g.tag = Some((T::Intermediate, n, 2));
                c = Some(g.out.clone());
                println!("FOUND {g:?} to swap output");
                break;
            }
        }

        return Out::Failed;
    }
    println!("c: {:?}", c);

    // c01 = i01a OR i01b
    let mut e = None;
    for g in logic.gates.iter_mut() {
        if !g.out.starts_with('z')
            && g.op == Op::Or
            && ((&g.a == b.as_ref().unwrap() && &g.b == c.as_ref().unwrap())
                || (&g.a == c.as_ref().unwrap() && &g.b == b.as_ref().unwrap()))
        {
            g.tag = Some((T::Carry, n, 0));
            e = Some(g.out.clone());
            println!("Carry {n} is {}", g.out);
            break;
        }
    }
    if e.is_none() {
        panic!("Could not find a gate setting carry for {zn} from intermediates {b:?} and {c:?}");
    }

    Out::Found
}

fn part2(mut logic: Logic) -> String {
    let x = make_num(&logic.values, 'x');
    let y = make_num(&logic.values, 'y');

    let expected = to_gates(x + y, 'z', 46);
    run(&mut logic);

    let wrong = expected
        .into_iter()
        .filter(|(gate, expected)| logic.values.get(gate).cloned().unwrap_or_default() != *expected)
        .collect::<Vec<_>>();

    let simple = wrong
        .iter()
        .filter(|(g, _)| {
            let Some(gate) = logic.gates.iter().find(|gg| gg.out == *g) else {
                return false;
            };

            (gate.a.starts_with('x') || gate.a.starts_with('y'))
                && (gate.b.starts_with('x') || gate.b.starts_with('y'))
        })
        .collect::<Vec<_>>();

    println!("wrong bits: {:?}", wrong.len());
    println!(
        "bits from simple gates: {:?}: {:?}",
        simple,
        logic.gates.iter().find(|g| g.out == simple[0].0)
    );
    for (w, _) in wrong.iter() {
        println!("{:?}", candidates(&logic, w));
    }

    if !tag0(&mut logic) {
        panic!("Could not tag 0");
    }
    logic.swap("hth", "tqr");
    let mut swaps = vec!["hth".to_string(), "tqr".to_string()];
    'outer: for i in 1..46 {
        while let Out::Swapped(a, b) = tagn(&mut logic, i) {
            swaps.push(a);
            swaps.push(b);
            if swaps.len() == 8 {
                break 'outer;
            }
        }
    }

    swaps.sort();
    swaps.join(",")
}

fn candidates(logic: &Logic, out: &str) -> Vec<String> {
    let mut to_visit = vec![out.to_string()];
    let mut cc = Vec::new();

    while let Some(n) = to_visit.pop() {
        let Some(gate) = logic.gates.iter().find(|g| g.out == n) else {
            continue;
        };
        cc.push(n.to_string());

        to_visit.push(gate.a.clone());
        to_visit.push(gate.b.clone());
    }

    cc
}

fn to_gates(num: u64, prefix: char, bits: usize) -> Vec<(String, bool)> {
    let mut res = Vec::with_capacity(bits);
    for i in 0..bits {
        res.push((format!("{prefix}{i:#02}"), num & (1 << i) != 0));
    }

    res
}

fn make_num(values: &Values, filter: char) -> u64 {
    let mut zs = values
        .iter()
        .filter(|(k, _v)| k.starts_with(filter))
        .collect::<Vec<_>>();

    zs.sort();
    zs.reverse();

    zs.iter().fold(0u64, |s, (_, b)| {
        let mut s = s << 1;
        if **b {
            s += 1;
        }
        s
    })
}

fn parse_input() -> Logic {
    let input = include_str!("../inputs/24.txt");
    let assign_re = Regex::new(r#"(.\d\d): (0|1)"#).unwrap();
    let gate_re = Regex::new(r#"(...) ((AND)|(OR)|(XOR)) (...) -> (...)"#).unwrap();

    let mut values = HashMap::new();
    for caps in assign_re.captures_iter(input) {
        values.insert(caps[1].to_string(), &caps[2] == "1");
    }

    let mut gates = Vec::new();
    for caps in gate_re.captures_iter(input) {
        gates.push(Gate {
            op: match &caps[2] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!(),
            },
            a: caps[1].to_string(),
            b: caps[6].to_string(),
            out: caps[7].to_string(),
            tag: None,
        });
    }

    Logic { values, gates }
}

pub fn main() {
    let input = parse_input();
    graph(&input);
    println!("Part 1 : {}", part1(input.clone()));
    println!("Part 2 : {}", part2(input.clone()));
}

#[allow(dead_code)]
fn graph(logic: &Logic) {
    for gate in logic.gates.iter() {
        let arrowhead = match gate.op {
            Op::And => "box",
            Op::Or => "normal",
            Op::Xor => "diamond",
        };
        println!("{} -> {} [arrowhead=\"{arrowhead}\"];", gate.a, gate.out);
        println!("{} -> {} [arrowhead=\"{arrowhead}\"];", gate.b, gate.out);
    }
}

// z00 = x00 XOR y00
// c00 = x00 AND y00
// z01 = ((x01 XOR y01) XOR c00)
// c01 = (y01 AND x01) OR (c00 AND (x01 XOR y01))
// i01x = x01 XOR y01
// i01a = y01 AND x01
// i01b = c00 AND i01x
// z01 = (i01x XOR c00)
// c01 = i01a OR i01b

// z01
// x | y | c | z
// 0 | 0 | 0 | 0
// 1 | 0 | 0 | 1
// 0 | 1 | 0 | 1
// 0 | 0 | 1 | 1
// 1 | 1 | 0 | 0
// 1 | 0 | 1 | 0
// 0 | 1 | 1 | 0
// 1 | 1 | 1 | 1

// c01 = (y01 AND x01) OR (c00 AND (x01 XOR y01))

// x | y | c-1 | c
// 0 | 0 | 0   | 0
// 1 | 0 | 0   | 0
// 0 | 1 | 0   | 0
// 0 | 0 | 1   | 0
// 1 | 1 | 0   | 1
// 1 | 0 | 1   | 1
// 0 | 1 | 1   | 1
// 1 | 1 | 1   | 1
