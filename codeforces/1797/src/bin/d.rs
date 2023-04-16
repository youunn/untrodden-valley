use std::collections::*;
use std::io::Write;

#[derive(Debug)]
struct Node {
    importance: i64,
    weight_of_subtree: Option<(u32, i64)>,
    parent: Option<usize>,
    children: BTreeSet<(i64, usize)>,
}

fn dfs(tree: &mut Vec<Node>, root: usize) -> Option<(u32, i64)> {
    if tree[root].weight_of_subtree.is_some() {
        return tree[root].weight_of_subtree;
    }

    let index = root;
    unsafe {
        let tree = tree as *mut Vec<Node>;
        (*tree)[index].weight_of_subtree = (*tree)[index].children.iter().fold(
            Some((1, (*tree)[index].importance)),
            |p, child| {
                p.and_then(|p| {
                    dfs(&mut *tree, child.1).map(|c| {
                        (p.0 + c.0, p.1 + c.1)
                    })
                })
            },
        );
    }

    tree[root].weight_of_subtree
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());

    let n: usize = scan.next()?;
    let m: u32 = scan.next()?;

    let mut tree = Vec::with_capacity(n);
    for _ in 0..n {
        // first is root
        let v: i64 = scan.next()?;
        tree.push(Node {
            importance: v,
            weight_of_subtree: None,
            parent: None,
            children: BTreeSet::new(),
        })
    }

    for _ in 0..n - 1 {
        let f: usize = scan.next()?;
        let s: usize = scan.next()?;
        tree[f - 1].children.insert((0, s - 1));
        tree[s - 1].children.insert((0, f - 1));
    }

    let mut to_visit = vec![0];
    while let Some(index) = to_visit.pop() {
        let tree = &mut tree as *mut Vec<Node>;
        unsafe {
            for next in &(*tree)[index].children {
                assert_ne!(next.1, index);
                (*tree)[next.1].parent = Some(index);
                (*tree)[next.1].children.remove(&(0, index));
                to_visit.push(next.1);
            }
        }
    }

    for _ in 0..m {
        let op: u8 = scan.next()?;
        let index = scan.next::<usize>()? - 1;
        if op == 2 {
            if tree[index].children.is_empty() {
                continue;
            }

            dfs(&mut tree, index).ok_or("")?;

            let child_pair = tree[index].children.first().ok_or("")?.clone();
            let child = child_pair.1;
            let parent = tree[index].parent.ok_or("")?;

            let child_weight = tree[child].weight_of_subtree.ok_or("")?;
            tree[child].weight_of_subtree = Some((
                tree[index].weight_of_subtree.unwrap().0,
                tree[index].weight_of_subtree.unwrap().1,
            ));
            tree[index].weight_of_subtree = Some((
                tree[index].weight_of_subtree.ok_or("")?.0 - child_weight.0,
                tree[index].weight_of_subtree.ok_or("")?.1 - child_weight.1,
            ));

            tree[index].parent = Some(child);
            tree[child].parent = Some(parent);
            tree[index].children.remove(&child_pair);
            tree[child]
                .children
                .insert((-(tree[index].weight_of_subtree.unwrap().0 as i64), index));
            tree[parent]
                .children
                .remove(&(-(tree[child].weight_of_subtree.unwrap().0 as i64), index));
            tree[parent]
                .children
                .insert((-(tree[child].weight_of_subtree.unwrap().0 as i64), child));
        } else {
            writeln!(out, "{}", dfs(&mut tree, index).ok_or("")?.1)?;
        }
    }

    Ok(())
}

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R> Scanner<R>
where
    R: std::io::BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    pub fn next<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: std::str::FromStr,
        <T>::Err: std::error::Error + 'static,
    {
        loop {
            if let Some(x) = self.buffer.pop() {
                let result = x.parse()?;
                return Ok(result);
            }
            let mut s = String::new();
            self.reader.read_line(&mut s)?;
            self.buffer = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}
