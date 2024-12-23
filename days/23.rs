use std::collections::{HashMap, HashSet};

type EdgeList = HashMap<String, HashSet<String>>;

fn list_3_connected(item: &str, edges: &EdgeList) -> HashSet<String> {
    let first = edges.get(item).unwrap();
    let mut triangles = HashSet::new();

    for x in first.iter() {
        for y in first.iter() {
            if x < y && edges.get(x).is_some_and(|ee| ee.contains(y)) {
                let mut items = [item.to_string(), x.to_string(), y.to_string()];
                items.sort();
                triangles.insert(items.join(""));
            }
        }
    }

    triangles
}

fn part1(vertices: &HashSet<String>, edges: &EdgeList) -> usize {
    vertices
        .iter()
        .filter(|v| v.starts_with('t'))
        .flat_map(|v| list_3_connected(v, edges))
        .collect::<HashSet<String>>()
        .len()
}

fn part2(vertices: &HashSet<String>, edges: &EdgeList) -> String {
    let mut cliques = Vec::new();
    bron_kerbosch2(
        HashSet::new(),
        vertices.clone(),
        HashSet::new(),
        edges,
        &mut cliques,
    );
    let mut max_clique = cliques
        .iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<String>>();

    max_clique.sort();
    max_clique.join(",")
}

type VL = HashSet<String>;

fn bron_kerbosch2(r: VL, mut p: VL, mut x: VL, g: &EdgeList, cliques: &mut Vec<VL>) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            cliques.push(r);
        }
        return;
    }

    let (_d, pivot) = p
        .union(&x)
        .map(|e| (g.get(e).unwrap().len(), e.to_string()))
        .max_by_key(|x| x.0)
        .unwrap();

    for v in p
        .difference(g.get(&pivot).unwrap())
        .cloned()
        .collect::<Vec<_>>()
    {
        let mut r2 = r.clone();
        r2.insert(v.clone());
        bron_kerbosch2(
            r2,
            p.intersection(g.get(&v).unwrap()).cloned().collect(),
            x.intersection(g.get(&v).unwrap()).cloned().collect(),
            g,
            cliques,
        );

        p.remove(&v);
        x.insert(v);
    }
}
// algorithm BronKerbosch2(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     choose a pivot vertex u in P ⋃ X
//     for each vertex v in P \ N(u) do
//         BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}

fn parse_input() -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let input = include_str!("../inputs/23.txt");
    let mut vertices = HashSet::new();
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap().to_string();
        vertices.insert(a.clone());
        vertices.insert(b.clone());
        let entry = edges.entry(a.clone()).or_default();
        entry.insert(b.clone());
        let entry = edges.entry(b.clone()).or_default();
        entry.insert(a.clone());
    }
    (vertices, edges)
}

pub fn main() {
    let (vertices, edges) = parse_input();
    println!("Part 1: {}", part1(&vertices, &edges));
    println!("Part 2: {}", part2(&vertices, &edges));
}
