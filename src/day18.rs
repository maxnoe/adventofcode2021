use crate::input;
use std::time::Instant;


#[derive(Debug,Copy,Clone)]
struct Pair {
    left: usize,
    right: usize,
    parent: Option<usize>,
}

#[derive(Debug,Copy,Clone)]
struct Literal {
    value: u8,
    parent: Option<usize>,
}

#[derive(Debug,Copy,Clone)]
enum Node {
    Pair(Pair),
    Literal(Literal),
}

impl Node {
    fn parent(&self) -> Option<usize> {
        match self {
            Node::Pair(p) => p.parent,
            Node::Literal(l) => l.parent,
        }
    }

    fn parent_mut(&mut self) -> &mut Option<usize> {
        match self {
            Node::Pair(p) => &mut p.parent,
            Node::Literal(l) => &mut l.parent,
        }
    }

    fn pair_mut(&mut self) -> Option<&mut Pair> {
        match self {
            Node::Pair(p) => Some(p),
            _ => None,
        }
    }

    fn pair(&self) -> Option<&Pair> {
        match self {
            Node::Pair(p) => Some(p),
            _ => None,
        }
    }

    fn literal_mut(&mut self) -> Option<&mut Literal> {
        match self {
            Node::Literal(l) => Some(l),
            _ => None,
        }
    }

    fn literal(&self) -> Option<&Literal> {
        match self {
            Node::Literal(l) => Some(l),
            _ => None,
        }
    }

    fn new_pair(parent: Option<usize>) -> Node {
        Node::Pair(Pair{left: 0, right: 0, parent})
    }

    fn new_literal(value: u8, parent: Option<usize>) -> Node {
        Node::Literal(Literal{value, parent})
    }

    #[allow(dead_code)]
    fn to_string(&self, arena: &Vec<Node>) -> String {
        let mut s = String::new();
        match self {
            Node::Pair(p) => {
                let left = &arena[p.left];
                let right = &arena[p.right];
                s.push('[');
                s.push_str(&left.to_string(arena));
                s.push(',');
                s.push_str(&right.to_string(arena));
                s.push(']');
            },
            Node::Literal(l) => s.push_str(&l.value.to_string())
        }

        return s;
    }

}

fn shift_adresses(node: &Node, shift: usize) -> Node {
    let s = |p| p + shift;
    let sp = |p: Option<usize>| p.map_or(None, |v| Some(s(v)));

    match node {
        Node::Pair(p) => Node::Pair(Pair{left: s(p.left), right: s(p.right), parent: sp(p.parent)}),
        Node::Literal(l) => Node::Literal(Literal{value: l.value, parent: sp(l.parent)}),
    }
}

#[derive(Debug,Clone)]
struct Number {
    arena: Vec<Node>
}


impl Number {
    fn new() -> Number {
        Number{arena: Vec::new()}
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.arena[0].to_string(&self.arena)
    }

    fn add(&self, other: &Number) -> Number {
        let mut result = Number::new();
        let len = self.arena.len();

        result.arena.push(Node::new_pair(None));
        let mut pair = result.arena[0].pair_mut().unwrap();

        pair.left = 1;
        pair.right = 1 + len;

        result.arena.extend(self.arena.iter().map(|n| shift_adresses(n, 1)));
        result.arena.extend(other.arena.iter().map(|n| shift_adresses(n, 1 + len)));
        result.arena[1].pair_mut().unwrap().parent = Some(0);
        result.arena[1 + len].pair_mut().unwrap().parent = Some(0);

        result.reduce();

        result
    }

    fn reduce(&mut self) {
        loop {
            if let Some(pos) = self.needs_exploding() {
                self.explode(pos);
                continue;
            }

            if let Some(pos) = self.needs_splitting() {
                self.split(pos);
                continue;
            }

            break;
        }
    }

    fn needs_exploding(&self) -> Option<usize> {
        self.find_first_pair_at_depth(0, 4, 0)
    }

    fn needs_splitting(&self) -> Option<usize> {
        self.find_first_number_gt_10(0)
    }

    fn split(&mut self, pos: usize) {
        let value = self.arena[pos].literal().unwrap().value;
        let parent = self.arena[pos].parent();

        let left_value = value / 2;
        let right_value = value - left_value;
        self.arena.push(Node::new_literal(left_value, Some(pos)));
        self.arena.push(Node::new_literal(right_value, Some(pos)));
        self.arena[pos] = Node::Pair(Pair{left: self.arena.len() - 2, right: self.arena.len() - 1, parent});
    }

    fn explode(&mut self, pos: usize) {
        let (left, right) = {
            let node = self.arena[pos].pair().unwrap();
            (node.left, node.right)
        };

        let (left_neighbor, right_neighbor) = self.find_neighbors(pos);

        if let Some(next) = left_neighbor {
            self.arena[next].literal_mut().unwrap().value += self.arena[left].literal().unwrap().value;
        }

        if let Some(next) = right_neighbor {
            self.arena[next].literal_mut().unwrap().value += self.arena[right].literal().unwrap().value;
        }

        {
            let node = self.arena[pos].pair().unwrap();
            for child in [node.left, node.right] {
                *self.arena[child].parent_mut() = None;
            }
        }

        self.arena[pos] = Node::new_literal(0, self.arena[pos].parent());
        self.remove_unused_literals()
    }

    fn remove_unused_literals(&mut self) {
        let mut i = 0;
        while i < self.arena.len() {
            if let Some(lit) = self.arena[i].literal() {
                if lit.parent.is_none() {
                    self.remove_node(i);
                    continue;
                }
            }
            i += 1;
        }
    }

    fn remove_node(&mut self, pos: usize) {
        let last_pos = self.arena.len() - 1;

        if pos == last_pos {
            self.arena.remove(pos);
            return
        }

        if let Some(pair) = self.arena[last_pos].pair_mut() {
            let left = pair.left;
            let right = pair.right;
            *self.arena[left].parent_mut() = Some(pos);
            *self.arena[right].parent_mut() = Some(pos);
        }

        if let Some(parent) = self.arena[last_pos].parent() {
            if let Some(pair) = self.arena[parent].pair_mut() {
                if pair.left == last_pos {
                    pair.left = pos;
                } else {
                    pair.right = pos;
                }
            }
        }

        self.arena.swap_remove(pos);
    }

    fn find_neighbors(&self, pos: usize) -> (Option<usize>, Option<usize>) {
        let parent = self.arena[self.arena[pos].parent().unwrap()].pair();

        if parent.is_none() {
            return (None, None);
        }

        let left = self.find_left_branch(pos).map_or(None, |p| Some(self.descent_right(p)));
        let right = self.find_right_branch(pos).map_or(None, |p| Some(self.descent_left(p)));

        (left, right)
    }

    fn find_left_branch(&self, pos: usize) -> Option<usize> {
        match self.arena[pos].parent() {
            None => None,
            Some(parent) => {
                let pair = self.arena[parent].pair().unwrap();

                if pair.left == pos {
                    self.find_left_branch(parent)
                } else {
                    Some(pair.left)
                }
            },
        }
    }

    fn find_right_branch(&self, pos: usize) -> Option<usize> {
        match self.arena[pos].parent() {
            None => None,
            Some(parent) => {
                let pair = self.arena[parent].pair().unwrap();

                if pair.right == pos {
                    self.find_right_branch(parent)
                } else {
                    Some(pair.right)
                }
            },
        }
    }

    fn descent_left(&self, pos: usize) -> usize {
        match self.arena[pos] {
            Node::Literal(_) => pos,
            Node::Pair(p) => self.descent_left(p.left),
        }
    }

    fn descent_right(&self, pos: usize) -> usize {
        match self.arena[pos] {
            Node::Literal(_) => pos,
            Node::Pair(p) => self.descent_right(p.right),
        }
    }

    fn find_first_pair_at_depth(&self, start: usize, depth: usize, current_depth: usize) -> Option<usize> {
        match self.arena[start] {
            Node::Literal(_) => None,
            Node::Pair(p) => {
                if depth == current_depth {
                    Some(start)
                } else {
                    self.find_first_pair_at_depth(p.left, depth, current_depth + 1)
                        .or_else(|| self.find_first_pair_at_depth(p.right, depth, current_depth + 1))
                }
            }
        }
    }

    fn find_first_number_gt_10(&self, start: usize) -> Option<usize> {
        match self.arena[start] {
            Node::Literal(l) => if l.value >= 10 {Some(start)} else {None},
            Node::Pair(p) => {
                self.find_first_number_gt_10(p.left)
                    .or_else(|| self.find_first_number_gt_10(p.right))
            }
        }
    }

    fn magnitude(&self) -> u64 {
        return self.magnitude_at_pos(0)
    }

    fn magnitude_at_pos(&self, pos: usize) -> u64 {
        match self.arena[pos] {
            Node::Literal(l) => l.value as u64,
            Node::Pair(p) => 3 * self.magnitude_at_pos(p.left) + 2 * self.magnitude_at_pos(p.right)
        }
    }
}

fn parse_number(line: &str) -> Result<Number, ()> {
    let mut number = Number::new();
    let mut node_index = None;

    for chr in line.chars() {
        match chr {
            '[' => {
                number.arena.push(Node::new_pair(node_index));
                node_index = Some(number.arena.len() - 1);
            },

            '0'..='9' => {
                number.arena.push(Node::new_literal(chr as u8 - b'0', node_index));
                let index = number.arena.len() - 1;

                let parent = number.arena.get_mut(node_index.unwrap()).unwrap();
                let pair = parent.pair_mut().unwrap();
                match pair.left {
                    0 => pair.left = index,
                    _ => pair.right = index,
                };
            },
            ']' => {
                let parent_index = number.arena[node_index.unwrap()].parent();
                if parent_index.is_none() {
                    continue;
                }

                let parent = number.arena.get_mut(parent_index.unwrap());
                match parent {
                    Some(n) => {
                        let pair = n.pair_mut().unwrap();
                        match pair.left {
                            0 => pair.left = node_index.unwrap(),
                            _ => pair.right = node_index.unwrap(),
                        }
                    }
                    None => {},
                }
                node_index = parent_index
            },
            ',' => {},
            _ => return Err(())
        }
    }
    Ok(number)
}

fn parse_input(input: &String) -> Vec<Number> {
    input.trim().lines().map(parse_number).map(Result::unwrap).collect()
}


fn part1(numbers: &Vec<Number>) -> u64 {
    let mut number = numbers[0].clone();
    for other in numbers[1..].iter() {
        number = number.add(other);
    }

    number.magnitude()
}


fn part2(numbers: &Vec<Number>) -> u64 {
    let mut max_mag = 0;
    for n1 in numbers {
        for n2 in numbers {
            let result = n1.add(n2);
            let mag = result.magnitude();
            if mag > max_mag {
                max_mag = mag;
            }
        }
    }

    max_mag
}


pub fn day18() {
    let input = input::get_input(18);
    let numbers = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&numbers));
    println!("Part2: {}", part2(&numbers));
    println!("Time: {} us", t0.elapsed().as_micros());
}
