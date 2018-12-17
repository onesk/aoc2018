const INPUT: &'static str = include_str!("inputs/8.txt");

use std::collections::HashMap;

fn get_numbers() -> Vec<i32> {
    INPUT.split_whitespace()
        .map(|slice| slice.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("everything should parse.")
}

struct TreeNode<'a> {
    children: Vec<TreeNode<'a>>,
    meta: &'a [i32]
}

fn parse_tree<'a>(numbers: &'a [i32]) -> (TreeNode<'a>, &'a [i32]) {
    let num_children = numbers[0];
    let num_meta = numbers[1] as usize;

    let mut children = Vec::new();
    let mut remains = &numbers[2..];

    for _child_idx in 0..num_children {
        let (child_node, new_remains) = parse_tree(remains);
        remains = new_remains;

        children.push(child_node);
    }

    (TreeNode{ children, meta: &remains[..num_meta] }, &remains[num_meta..])
}

fn sum_meta<'a>(node: &TreeNode<'a>) -> i32 {
    node.children.iter().fold(0i32, |a, n| a + sum_meta(n)) + node.meta.iter().sum::<i32>()
}

fn sum_contrived<'a>(node: &TreeNode<'a>) -> i32 {
    let &TreeNode{ ref children, ref meta } = node;

    if children.len() == 0 {
        meta.iter().sum::<i32>()

    } else {
        let mut seen = HashMap::new();
        let mut sum = 0i32;

        for &idx in meta.iter() {
            if let Some(child_node) = children.get((idx-1) as usize) {
                sum += *seen.entry(idx).or_insert_with(|| sum_contrived(child_node));
            }
        }

        sum
    }
}

fn part_one<'a>(root: &TreeNode<'a>) {
    println!("{}", sum_meta(&root));
}

fn part_two<'a>(root: &TreeNode<'a>) {
    println!("{}", sum_contrived(&root));
}

fn main() {
    let numbers = get_numbers();
    let (root, remains) = parse_tree(&numbers);
    assert!(remains.is_empty());

    part_one(&root);
    part_two(&root);
}
