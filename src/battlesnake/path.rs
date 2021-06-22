use std::convert::From;

use super::Direction;
use super::Point;

#[derive(Clone, PartialEq, Debug)]
pub struct Path {
    pub nodes: Vec<Point>,
}

impl Path {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_vec(nodes: Vec<Point>) -> Self {
        Self { nodes }
    }

    /// Extends _in front_ the path with node\[0\] + offset if the path is not empty,
    /// otherwise nothing happens.
    ///
    /// # Arguments
    /// * `offset` - A `Point` to offset the first node with
    ///
    /// # Example
    /// ```
    /// use path::Path;
    ///
    /// let mut p = Path::from_vec(vec![Point::new(1, 1)]);
    /// let offset = Point::new(0, 1);
    /// p.extend_front(&offset);
    /// assert_eq!(ones.nodes, vec![Point::new(1, 2), Point::new(1, 1)]);
    /// ```
    pub fn extend_front(&mut self, offset: &Point) {
        if !self.nodes.is_empty() {
            let new_head = self.nodes[0] + *offset;
            self.nodes.insert(0, new_head);
        }
    }

    pub fn extend_front_dir(&mut self, dir: &Direction) {
        if !self.nodes.is_empty() {
            let new_head = self.nodes[0] + *dir;
            self.nodes.insert(0, new_head);
        }
    }

    pub fn first(&self) -> Option<Point> {
        self.nodes.first().cloned()
    }

    pub fn last(&self) -> Option<Point> {
        self.nodes.last().cloned()
    }

    pub fn get_node(&self, index: usize) -> Option<Point> {
        self.nodes.get(index).cloned()
    }
}

impl From<&Vec<Point>> for Path {
    #[inline]
    fn from(input: &Vec<Point>) -> Self {
        Self {
            nodes: input.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut ones = Path::from_vec(vec![Point::new(1, 1)]);
        assert_eq!(ones.nodes, vec![Point::new(1, 1)]);

        assert_eq!(ones.first(), Some(Point::new(1, 1)));
        assert_eq!(ones.last(), Some(Point::new(1, 1)));
        let offset = Point::new(-1, 0);
        ones.extend_front(&offset);
        let offset = Point::new(0, -1);
        ones.extend_front(&offset);
        assert_eq!(ones.first(), Some(Point::new(0, 0)));
        assert_eq!(ones.last(), Some(Point::new(1, 1)));
        assert_eq!(ones.get_node(1), Some(Point::new(0, 1)));
    }

    #[test]
    fn extend() {
        let mut ones = Path::from_vec(vec![Point::new(5, 1)]);
        assert_eq!(ones.nodes, vec![Point::new(5, 1)]);

        assert_eq!(ones.first(), Some(Point::new(5, 1)));
        assert_eq!(ones.last(), Some(Point::new(5, 1)));

        let offset = Point::new(-1, 0);
        ones.extend_front(&offset);
        assert_eq!(ones.nodes, vec![Point::new(4, 1), Point::new(5, 1)]);
        ones.extend_front(&offset);
        assert_eq!(
            ones.nodes,
            vec![Point::new(3, 1), Point::new(4, 1), Point::new(5, 1)]
        );
        ones.extend_front(&offset);
        assert_eq!(
            ones.nodes,
            vec![
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(5, 1)
            ]
        );
    }

    #[test]
    fn extend_dir() {
        let mut path = Path::from_vec(vec![Point::new(5, 1)]);
        path.extend_front_dir(&Direction::Right);
        assert_eq!(path.nodes, vec![Point::new(6, 1), Point::new(5, 1)]);
        path.extend_front_dir(&Direction::Up);
        assert_eq!(
            path.nodes,
            vec![Point::new(6, 2), Point::new(6, 1), Point::new(5, 1)]
        );
        path.extend_front_dir(&Direction::Left);
        assert_eq!(
            path.nodes,
            vec![
                Point::new(5, 2),
                Point::new(6, 2),
                Point::new(6, 1),
                Point::new(5, 1)
            ]
        );
        path.extend_front_dir(&Direction::Down);
        assert_eq!(
            path.nodes,
            vec![
                Point::new(5, 1),
                Point::new(5, 2),
                Point::new(6, 2),
                Point::new(6, 1),
                Point::new(5, 1)
            ]
        );
    }
}
