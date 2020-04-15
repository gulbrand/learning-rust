use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn len(
        edges: &HashMap<i32, Vec<i32>>,
        visited: &HashSet<i32>,
        start: i32) -> i32 {

        if let Some(_v) = visited.get(&start) {
            return 0;
        }

        let max_length_so_far = std::i32::MIN;
        if let Some(es) = edges.get(&start) {
            for _i in es {

            }
        }
        return max_length_so_far;
    }

    pub fn path(_edges: &HashMap<i32, Vec<i32>>, _start: i32) -> Vec<Vec<i32>> {
        Vec::new()
    }

    #[allow(unused)]
    pub fn critical_connections(_n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in connections.iter() {
            edges.entry(edge[0]).or_default().push(edge[1]);
            edges.entry(edge[1]).or_default().push(edge[0]);
        }

        println!("{:?}", edges);

        let _lengths: HashMap<i32, i32> = HashMap::new();
        let mut max_length: (i32, i32) = (0, std::i32::MIN);
        for (start, _nodes) in edges.iter() {
            let mut visited: HashSet<i32> = HashSet::new();
            let length = Solution::len(&edges, &mut visited, *start);
            if length > max_length.1 {
                max_length = (*start, length);
            }
        }

        Solution::path(&edges, max_length.0)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Solution;

    pub fn run_test(
        connections: Vec<[i32; 2]>,
        expected: Vec<[i32; 2]>) {
        let connections: Vec<Vec<i32>> =
            connections.iter().map(|pair| vec![pair[0], pair[1]]).collect();
        let expected: Vec<Vec<i32>> =
            expected.iter().map(|pair| vec![pair[0], pair[1]]).collect();
        let n = connections.len() as i32;
        let critical_connections = Solution::critical_connections(n, connections);
        assert_eq!(critical_connections, expected);
    }

    #[test]
    pub fn simple_test() {
        let connections = vec![[0,1],[1,2],[2,0],[1,3]];
        run_test(connections, vec![[3, 1]]);
    }

    #[test]
    pub fn harder_test() {
        let connections = vec![[0,1],[1,2],[2,0],[1,3],[3,4],[4,5],[5,3]];
        run_test(connections, vec![[3, 1]]);
    }

}


pub fn main() {
    println!("hi");
}