use id_tree::InsertBehavior::*;
use id_tree::*;

fn parse_input() -> Vec<Tree<isize>> {
    let mut trees = Vec::new();
    for line in include_str!("../../data/day18.txt").lines() {
        let mut tree: Tree<isize> = TreeBuilder::new().with_node_capacity(10).build();
        let root_id = tree.insert(Node::new(-1), AsRoot).unwrap();
        let mut child_id = root_id.clone();
        for char in line.chars().skip(1) {
            match char {
                '[' => {
                    child_id = tree.insert(Node::new(-1), UnderNode(&child_id)).unwrap();
                }
                '0'..='9' => {
                    tree.insert(
                        Node::new(char.to_string().parse::<isize>().unwrap()),
                        UnderNode(&child_id),
                    )
                    .unwrap();
                }
                ']' => {
                    let parent = tree.ancestor_ids(&child_id).unwrap().next();
                    if parent.is_some() {
                        child_id = parent.unwrap().clone();
                    }
                }
                _ => (),
            }
        }

        trees.push(tree);
    }

    trees
}

pub fn processor() -> (isize, isize) {
    let trees = parse_input();
    let mut output = (0, 0);

    // Part 1.
    let mut main_tree = trees[0].clone();
    for i in 1..trees.len() {
        add_tree(&mut main_tree, &trees[i]);
        reduce_tree(&mut main_tree);
    }

    count_tree(&main_tree, &mut output.0);

    // Part 2.
    for i in 0..trees.len() {
        for j in 0..trees.len() {
            if i == j {
                continue;
            }

            let mut left_tree = trees[i].to_owned();
            let mut right_tree = trees[j].to_owned();

            // Try left on right...
            add_tree(&mut left_tree, &trees[j]);
            reduce_tree(&mut left_tree);
            let mut magnitude = 0;
            count_tree(&left_tree, &mut magnitude);
            if magnitude > output.1 {
                output.1 = magnitude;
            }

            // ... then right on left.
            add_tree(&mut right_tree, &trees[i]);
            reduce_tree(&mut right_tree);
            let mut magnitude = 0;
            count_tree(&right_tree, &mut magnitude);
            if magnitude > output.1 {
                output.1 = magnitude;
            }
        }
    }

    output
}

fn add_tree(main_tree: &mut Tree<isize>, added_tree: &Tree<isize>) {
    // Add new common root, with left-side tree as first child.
    let new_root = main_tree.insert(Node::new(-1), AsRoot).unwrap();

    // Recursively add all right-side tree nodes as second child.
    recursive_insert(
        main_tree,
        added_tree,
        added_tree.root_node_id().unwrap(),
        &new_root,
    );
}

fn reduce_tree(tree: &mut Tree<isize>) {
    loop {
        if explode_tree(tree) {
            continue;
        }

        if split_tree(tree) {
            continue;
        }

        break;
    }
}

fn count_tree(tree: &Tree<isize>, output: &mut isize) {
    let root_id = tree.root_node_id().unwrap();
    let nodes: Vec<NodeId> = tree.traverse_post_order_ids(root_id).unwrap().collect();
    for i in 0..nodes.len() {
        let node = tree.get(&nodes[i]).unwrap();
        let mut data = *node.data();
        if data >= 0 {
            recursive_count_magnitude(tree, &nodes[i], &mut data, output);
        }
    }
}

fn recursive_insert(
    main_tree: &mut Tree<isize>,
    added_tree: &Tree<isize>,
    added_node_id: &NodeId,
    parent_id: &NodeId,
) {
    let added_node = added_tree.get(added_node_id).unwrap();
    let new_node_id = main_tree
        .insert(Node::new(*added_node.data()), UnderNode(parent_id))
        .unwrap();

    for child_id in added_node.children() {
        recursive_insert(main_tree, added_tree, child_id, &new_node_id);
    }
}

fn explode_tree(tree: &mut Tree<isize>) -> bool {
    let root_id = tree.root_node_id().unwrap();
    let nodes: Vec<NodeId> = tree.traverse_post_order_ids(root_id).unwrap().collect();
    for i in 0..nodes.len() {
        let node = tree.get(&nodes[i]).unwrap();
        let data = *node.data();
        if data < 0 && tree.ancestor_ids(&nodes[i]).unwrap().count() > 3 {
            let left_node_id = node.children().first().unwrap();
            let left_data = *tree.get(left_node_id).unwrap().data();
            let left_node_id = left_node_id.to_owned();

            let right_node_id = node.children().last().unwrap();
            let right_data = *tree.get(right_node_id).unwrap().data();
            let right_node_id = right_node_id.to_owned();

            tree.remove_node(left_node_id, RemoveBehavior::DropChildren)
                .unwrap();

            tree.remove_node(right_node_id, RemoveBehavior::DropChildren)
                .unwrap();

            *tree.get_mut(&nodes[i]).unwrap().data_mut() = 0;

            if i > 0 {
                for j in (0..i - 1).rev() {
                    if try_add_data(tree, &nodes[j], left_data) {
                        break;
                    }
                }
            }

            if i < nodes.len() - 1 {
                for j in i + 1..nodes.len() {
                    if try_add_data(tree, &nodes[j], right_data) {
                        break;
                    }
                }
            }

            return true;
        }
    }

    false
}

fn split_tree(tree: &mut Tree<isize>) -> bool {
    let root_id = tree.root_node_id().unwrap();
    let nodes: Vec<NodeId> = tree.traverse_post_order_ids(root_id).unwrap().collect();
    for i in 0..nodes.len() {
        let node = tree.get(&nodes[i]).unwrap();
        let data = *node.data();
        if data >= 10 {
            *tree.get_mut(&nodes[i]).unwrap().data_mut() = -1;

            let left_half = data / 2;
            tree.insert(Node::new(left_half), UnderNode(&nodes[i]))
                .unwrap();

            let right_half = data - left_half;
            tree.insert(Node::new(right_half), UnderNode(&nodes[i]))
                .unwrap();

            return true;
        }
    }

    false
}

fn try_add_data(tree: &mut Tree<isize>, node_id: &NodeId, data: isize) -> bool {
    let node = tree.get_mut(node_id);
    if node.is_err() {
        return false;
    }
    let node = node.unwrap();

    if *node.data() >= 0isize {
        *node.data_mut() += data;
        return true;
    }

    false
}

fn recursive_count_magnitude(
    tree: &Tree<isize>,
    node_id: &NodeId,
    count: &mut isize,
    total: &mut isize,
) {
    let node = tree.get(node_id).unwrap();
    if tree.root_node_id().unwrap() == node_id {
        *total += *count;
    } else {
        let parent = node.parent().unwrap();
        let is_left_child = tree.get(parent).unwrap().children().first().unwrap() == node_id;
        let multiplier: isize = if is_left_child { 3 } else { 2 };
        *count *= multiplier;
        recursive_count_magnitude(tree, parent, count, total);
    }
}

#[allow(dead_code)]
fn print_tree(prefix: &str, tree: &Tree<isize>) {
    print!("{}", prefix);
    print_node(&tree, tree.root_node_id().unwrap());
    println!();
}

fn print_node(tree: &Tree<isize>, node_id: &NodeId) {
    let node = tree.get(node_id).unwrap();
    let data = *node.data();
    if data < 0 {
        print!("[");
        let children = node.children();
        for i in 0..children.len() {
            print_node(tree, &children[i]);
            if i < children.len() - 1 {
                print!(",");
            }
        }
        print!("]");
    } else {
        print!("{}", data);
    }
}
