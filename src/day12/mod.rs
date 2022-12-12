use std::{
    fs::File,
    io::{BufWriter, Write},
};

use color_eyre::Result;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::{
    algo::{connected_components, dijkstra},
    dot::{Config, Dot},
    graph::NodeIndex,
    Graph, Undirected,
};

use crate::{utils::four_neighbors, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Height {
    Start,
    End,
    Height(usize),
}

impl Height {
    fn from_byte(b: u8) -> Self {
        match b {
            b'S' => Self::Start,
            b'E' => Self::End,
            c @ b'a'..=b'z' => Self::Height((c - b'a') as _),
            _ => unreachable!(),
        }
    }

    fn height(self) -> usize {
        match self {
            Height::Start => 0,
            Height::End => (b'z' - b'a') as _,
            Height::Height(height) => height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    _coords: (usize, usize),
    height: Height,
}

impl Runner for Day {
    type Input<'input> = (Graph<Position, ()>, NodeIndex<u32>, NodeIndex<u32>);

    fn day() -> usize {
        12
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut start = None;
        let mut end = None;
        let v = input
            .lines()
            .map(|line| line.bytes())
            .flatten()
            .map(|b| (Height::from_byte(b), None))
            .collect_vec();
        let mut graph = Graph::with_capacity(height * width, height * width);
        let mut array = Array2::from_shape_vec((height, width), v)?;
        // Add nodes to graph
        array.indexed_iter_mut().for_each(|((r, c), h)| {
            let p = Position {
                _coords: (r, c),
                height: h.0,
            };
            h.1 = Some(graph.add_node(p));
        });

        // Add edges to graph
        array.indexed_iter().for_each(|((r, c), (h, id))| {
            if *h == Height::Start {
                start = id.clone();
            }
            if *h == Height::End {
                end = id.clone();
            }
            four_neighbors((r, c), (height, width))
                .filter_map(|(or, oc)| {
                    let other = array[(or, oc)];
                    let height = other.0.height();
                    let diff = height as isize - h.height() as isize;
                    (diff < 2).then(|| other.1)
                })
                .for_each(|o_id| {
                    graph.update_edge(id.unwrap(), o_id.unwrap(), ());
                });
        });

        Ok((graph, start.unwrap(), end.unwrap()))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (graph, start, end) = input;

        // println!("connected: {}", connected_components(&graph));
        // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
        // let mut writer = BufWriter::new(File::create("output.dot")?);
        // write!(writer, "{:?}", dot)?;
        let path = dijkstra(graph, *start, None, |_| 1);
        // println!();
        // println!("{path:#?}");
        Ok(*path.get(end).unwrap())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (graph, _, end) = input;
        let mut graph = graph.clone();
        graph.reverse();

        let paths = dijkstra(&graph, *end, None, |_| 1);
        Ok(*graph
            .node_indices()
            .map(|id| (id, graph.node_weight(id).unwrap().clone()))
            .filter(|(_, n)| n.height.height() == Height::Start.height())
            .map(|(id, _)| (id, paths.get(&id)))
            .min_by_key(|(_, d)| d.unwrap_or(&i32::MAX))
            .unwrap()
            .1
            .unwrap() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Sabqponm\n\
                abcryxxl\n\
                accszExk\n\
                acctuvwj\n\
                abdefghi\n";
            part1 = 31;
            part2 = 29;
    }

    prod_case! {
        part1 = 361;
        part2 = 201684;
    }
}
