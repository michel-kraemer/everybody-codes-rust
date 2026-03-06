use std::fs;

/// Either a plug or a socket
struct Connector<'a> {
    color: &'a str,
    shape: &'a str,
}

impl Connector<'_> {
    /// Checks if this connector forms a strong bond with another connector
    fn matches_strong(&self, other: &Connector) -> bool {
        self.color == other.color && self.shape == other.shape
    }

    /// Checks if this connector forms a weak bond with another connector
    fn matches_weak(&self, other: &Connector) -> bool {
        self.color == other.color || self.shape == other.shape
    }
}

/// A tree node
struct Node<'a> {
    id: i32,
    plug: Connector<'a>,
    left_socket: Connector<'a>,
    right_socket: Connector<'a>,
    left_child: Option<usize>,
    right_child: Option<usize>,
}

/// Insert logic for part 1: Attach the child to the first free socket in the
/// tree in reading order that forms a strong bond with the child's plug
fn insert_part1(i: usize, child: &mut usize, nodes: &mut [Node]) -> bool {
    if let Some(left_child) = nodes[i].left_child {
        if insert_part1(left_child, child, nodes) {
            return true;
        }
    } else if nodes[i].left_socket.matches_strong(&nodes[*child].plug) {
        nodes[i].left_child = Some(*child);
        return true;
    }

    if let Some(right_child) = nodes[i].right_child {
        if insert_part1(right_child, child, nodes) {
            return true;
        }
    } else if nodes[i].right_socket.matches_strong(&nodes[*child].plug) {
        nodes[i].right_child = Some(*child);
        return true;
    }

    false
}

/// Insert logic for part 2: Attach the child to the first free socket in the
/// tree in reading order that forms a weak bond with the child's plug
fn insert_part2(i: usize, child: &mut usize, nodes: &mut [Node]) -> bool {
    if let Some(left_child) = nodes[i].left_child {
        if insert_part2(left_child, child, nodes) {
            return true;
        }
    } else if nodes[i].left_socket.matches_weak(&nodes[*child].plug) {
        nodes[i].left_child = Some(*child);
        return true;
    }

    if let Some(right_child) = nodes[i].right_child {
        if insert_part2(right_child, child, nodes) {
            return true;
        }
    } else if nodes[i].right_socket.matches_weak(&nodes[*child].plug) {
        nodes[i].right_child = Some(*child);
        return true;
    }

    false
}

/// Insert logic for part 3: Traverse the tree in reading order. If an occupied
/// socket forms weak bond with the plug of its child node and the plug of the
/// child to be inserted would form a strong bond, replace the socket child with
/// the child to be inserted and continue traversing with the replaced child.
/// Otherwise, find the first free socket that forms a weak bond with the plug
/// of the child to be inserted. Return `false` if no socket could be found. In
/// this case, the operation will be repeated with the current child to be
/// inserted from the tree's root.
fn insert_part3(i: usize, child: &mut usize, nodes: &mut [Node]) -> bool {
    if let Some(left_child) = nodes[i].left_child {
        if !nodes[i].left_socket.matches_strong(&nodes[left_child].plug)
            && nodes[i].left_socket.matches_strong(&nodes[*child].plug)
        {
            nodes[i].left_child = Some(*child);
            *child = left_child;
        } else if insert_part3(left_child, child, nodes) {
            return true;
        }
    } else if nodes[i].left_socket.matches_weak(&nodes[*child].plug) {
        nodes[i].left_child = Some(*child);
        return true;
    }

    if let Some(right_child) = nodes[i].right_child {
        if !nodes[i]
            .right_socket
            .matches_strong(&nodes[right_child].plug)
            && nodes[i].right_socket.matches_strong(&nodes[*child].plug)
        {
            nodes[i].right_child = Some(*child);
            *child = right_child;
        } else if insert_part3(right_child, child, nodes) {
            return true;
        }
    } else if nodes[i].right_socket.matches_weak(&nodes[*child].plug) {
        nodes[i].right_child = Some(*child);
        return true;
    }

    false
}

/// Traverse the tree in reading order and collect all node IDs
fn read(i: usize, nodes: &[Node], result: &mut Vec<i32>) {
    if let Some(left_child) = nodes[i].left_child {
        read(left_child, nodes, result);
    }
    result.push(nodes[i].id);
    if let Some(right_child) = nodes[i].right_child {
        read(right_child, nodes, result);
    }
}

/// Solve a part using the given insert function
fn solve<I>(file: &str, insert: I)
where
    I: Fn(usize, &mut usize, &mut [Node]) -> bool,
{
    let input = fs::read_to_string(file).expect("Could not read file");

    let mut nodes = Vec::new();
    for l in input.lines() {
        let parts = l.split(", ").collect::<Vec<_>>();
        let (_, id) = parts[0].split_once('=').unwrap();
        let id = id.parse::<i32>().unwrap();
        let (_, plug) = parts[1].split_once('=').unwrap();
        let (_, left_socket) = parts[2].split_once('=').unwrap();
        let (_, right_socket) = parts[3].split_once('=').unwrap();

        let (color, shape) = plug.split_once(' ').unwrap();
        let plug = Connector { color, shape };

        let (color, shape) = left_socket.split_once(' ').unwrap();
        let left_socket = Connector { color, shape };

        let (color, shape) = right_socket.split_once(' ').unwrap();
        let right_socket = Connector { color, shape };

        let mut pos = nodes.len();
        nodes.push(Node {
            id,
            plug,
            left_socket,
            right_socket,
            left_child: None,
            right_child: None,
        });

        if pos > 0 {
            // since the problem statement says it's guaranteed that all nodes
            // can be inserted, we retry until we succeed
            while !insert(0, &mut pos, &mut nodes) {}
        }
    }

    let mut order = Vec::new();
    read(0, &nodes, &mut order);

    let total = order
        .into_iter()
        .enumerate()
        .map(|(i, r)| (i as i32 + 1) * r)
        .sum::<i32>();
    println!("{total}");
}

fn main() {
    solve("everybody_codes_e3_q03_p1.txt", insert_part1);
    solve("everybody_codes_e3_q03_p2.txt", insert_part2);
    solve("everybody_codes_e3_q03_p3.txt", insert_part3);
}
