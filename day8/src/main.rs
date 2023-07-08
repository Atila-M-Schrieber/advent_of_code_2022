#[cfg_attr(debug_assertions, allow(dead_code), allow(unused_variables))]
#[allow(dead_code)]
#[allow(unused_variables)]

struct Forest<T>(Vec<Vec<T>>);

use Direction::*;
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl<T> From<Vec<Vec<T>>> for Forest<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        Forest::new(v)
    }
}

impl<T> Forest<T> {
    fn new(matrix: Vec<Vec<T>>) -> Self {
        let first_len = matrix[0].len();
        let rows_same_len = matrix
            .iter()
            .skip(1)
            .fold((first_len, true), |(prev_len, prev_state), subvec| {
                (subvec.len(), prev_state && prev_len == subvec.len())
            })
            .1;
        assert!(rows_same_len, "Not all rows are the same length.");
        Forest(matrix)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

impl Forest<u8> {
    fn visibility_score_from(&self, dir: Direction) -> Forest<usize> {
        let height = self.height();
        let width = self.width();

        // Vec of "iterators" to turn into the bool forest
        let it: Vec<Vec<_>> = match dir {
            Left => self.0.clone(),
            Right => self
                .0
                .iter()
                .map(|row| row.iter().cloned().rev().collect())
                .collect(),
            Top => (0..width)
                .map(|i| self.0.iter().map(|row| row[i]).collect())
                .collect(),
            Bottom => (0..width)
                .map(|i| self.0.iter().map(|row| row[i]).rev().collect())
                .collect(),
        };

        // get the number of trees visible from each side of the tree
        let mut vis: Vec<Vec<usize>> = Vec::new();
        for (i, row) in it.iter().enumerate() {
            // first tree always visible
            vis.push(Vec::new());
            vis[i].push(0); // first in virtual row sees 0 trees

            for (j, tree) in row.iter().enumerate().skip(1) {
                // let is_vis = it[i].iter().take(j).all(|prev_height| tree > prev_height);
                let num_vis = {
                    let mut nv = 0;
                    let h = it[i][j];
                    for tree in it[i].iter().take(j).rev() {
                        nv += 1;
                        if tree >= &h {
                            break;
                        }
                    }
                    nv
                };
/*                let num_vis = {
                    let seen = it[i]
                        .iter()
                        .take(j)
                        .rev()
                        .take_while(|&&h| h < it[i][j])
                        .count();
                    it[i].iter().take(seen + 1).count()
                };
                */
                vis[i].push(num_vis);
                #[cfg(test)]
                {
                    println!("Tree of index {j} in virtual row {i}:{:?} can see {num_vis} trees in the {:?} direction", it[i], dir);
                }
            }
        }
        // for

        // reverses the effects of transposition / reversion done in 'it'
        vis = match dir {
            Left => vis,
            Right => vis
                .iter()
                .map(|row| row.iter().cloned().rev().collect())
                .collect(),
            Top => (0..height)
                .map(|i| vis.iter().map(|row| row[i]).collect())
                .collect(),
            Bottom => (0..height)
                .rev()
                .map(|i| vis.iter().map(|row| row[i]).collect())
                .collect(),
        };

        Forest::new(vis)
    }

    fn visibility_scores(&self) -> Forest<usize> {
        let l = self.visibility_score_from(Left).0;
        let r = self.visibility_score_from(Right).0;
        let t = self.visibility_score_from(Top).0;
        let b = self.visibility_score_from(Bottom).0;

        let width = self.width();
        let height = self.height();

        let mut vis = Vec::new();
        for i in 0..height {
            vis.push(Vec::new());
            for j in 0..height {
                let vis_score = l[i][j] * r[i][j] * t[i][j] * b[i][j];
                vis[i].push(vis_score);
            }
        }

        Forest::new(vis)
    }

    fn visible_from(&self, dir: Direction) -> Forest<bool> {
        let height = self.height();
        let width = self.width();

        // Vec of "iterators" to turn into the bool forest
        let it: Vec<Vec<_>> = match dir {
            Left => self.0.clone(),
            Right => self
                .0
                .iter()
                .map(|row| row.iter().cloned().rev().collect())
                .collect(),
            Top => (0..width)
                .map(|i| self.0.iter().map(|row| row[i]).collect())
                .collect(),
            Bottom => (0..width)
                .map(|i| self.0.iter().map(|row| row[i]).rev().collect())
                .collect(),
        };

        // fn visible(i: usize, along: Vec<u8>) -> bool
        let mut vis: Vec<Vec<bool>> = Vec::new();
        for (i, row) in it.iter().enumerate() {
            // first tree always visible
            vis.push(Vec::new());
            vis[i].push(true);

            for (j, tree) in row.iter().enumerate().skip(1) {
                let is_vis = it[i].iter().take(j).all(|prev_height| tree > prev_height);
                // println!("{}", it[i].iter().take(j).len());
                vis[i].push(is_vis);
                #[cfg(test)]
                {
                    println!(
                        "Tree of index {j} in virtual row {:?} visible from {:?}: {is_vis}",
                        it[i], dir
                    );
                }
            }
        }
        // for

        // reverses the effects of transposition / reversion done in 'it'
        vis = match dir {
            Left => vis,
            Right => vis
                .iter()
                .map(|row| row.iter().cloned().rev().collect())
                .collect(),
            Top => (0..height)
                .map(|i| vis.iter().map(|row| row[i]).collect())
                .collect(),
            Bottom => (0..height)
                .rev()
                .map(|i| vis.iter().map(|row| row[i]).collect())
                .collect(),
        };

        Forest::new(vis)
    }

    fn visible(&self) -> Forest<bool> {
        let l = self.visible_from(Left).0;
        let r = self.visible_from(Right).0;
        let t = self.visible_from(Top).0;
        let b = self.visible_from(Bottom).0;

        let width = self.width();
        let height = self.height();

        let mut vis = Vec::new();
        for i in 0..height {
            vis.push(Vec::new());
            for j in 0..height {
                let is_vis = l[i][j] || r[i][j] || t[i][j] || b[i][j];
                vis[i].push(is_vis);
            }
        }

        Forest::new(vis)
    }
}

fn main() {
    let trees: Forest<u8> = include_str!("../input.txt")
        .lines()
        .map(|s| s.as_bytes().iter().map(|c| c - 48).collect())
        .collect::<Vec<_>>()
        .into();

    let visible = trees.visible();

    let num_vis = visible.0.iter().flatten().filter(|&&b| b).count();

    println!("Number of trees visible: {num_vis}");

    let scores = trees.visibility_scores();

    let max_vis_score = scores.0.iter().flatten().max().unwrap();

    println!("Highest visibility score: {max_vis_score}");
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_forest() -> Forest<u8> {
        "30373\n25512\n65332\n33549\n35390"
            .lines()
            .map(|s| s.as_bytes().iter().map(|c| c - 48).collect())
            .collect::<Vec<_>>()
            .into()
    }

    #[test]
    fn visibility() {
        let forest = example_forest();
        let l = forest.visible_from(Left).0;
        let r = forest.visible_from(Right).0;
        let t = forest.visible_from(Top).0;
        let b = forest.visible_from(Bottom).0;
        let vis = forest.visible().0;

        let mut correct = vec![
            vec![
                vec![true, false, true, false],
                vec![false, false, true, false],
                vec![false, false, true, false],
                vec![true, true, true, false],
                vec![false, true, true, false],
            ],
            vec![
                vec![true, false, false, false],
                vec![true, false, true, false],
                vec![false, true, true, false],
                vec![false, false, false, false],
                vec![false, true, false, false],
            ],
            vec![
                vec![true, true, true, true],
                vec![false, true, false, false],
                vec![false, false, false, false],
                vec![false, true, false, false],
                vec![false, true, false, false],
            ],
            vec![
                vec![true, false, false, false],
                vec![false, false, false, false],
                vec![true, false, false, true],
                vec![false, false, false, false],
                vec![true, true, true, true],
            ],
            vec![
                vec![true, false, false, true],
                vec![true, false, false, true],
                vec![false, false, false, true],
                vec![true, true, true, true],
                vec![false, true, false, true],
            ],
        ];

        for row in correct.iter_mut() {
            for tree in row.iter_mut() {
                tree.push(tree.iter().any(|&dir| dir))
            }
        }

        let width = forest.width();
        let height = forest.height();

        for i in 0..height {
            for j in 0..width {
                correct[i][j].pop();
                assert_eq!(
                    correct[i][j],
                    vec![l[i][j], r[i][j], t[i][j], b[i][j]], //vis[i][j]],
                    "Indeces: ({i}, {j})"
                );
            }
        }
    }

    #[test]
    fn scores() {
        let forest = example_forest();
        let l = forest.visibility_score_from(Left).0;
        let r = forest.visibility_score_from(Right).0;
        let t = forest.visibility_score_from(Top).0;
        let b = forest.visibility_score_from(Bottom).0;
        let vis = forest.visibility_scores().0;

        assert_eq!(
            vec![2, 2, 2, 1, 8],
            vec![l[3][2], r[3][2], t[3][2], b[3][2], vis[3][2]]
        );
    }
}
