/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

//
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    // 修改1：实现 bfs_with_return 方法
    // 使用 VecDeque 实现 BFS，借助 visited 数组记录访问过的节点，返回访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 添加一个 VecDeque 用于队列操作
        let mut queue: VecDeque<usize> = VecDeque::new();
        // 添加一个 visited 向量，用于记录每个节点是否已访问
        let mut visited = vec![false; self.adj.len()];
        // 添加一个向量用于保存访问顺序
        let mut visit_order = vec![];

        // 修改2：将起始节点入队并标记为已访问
        queue.push_back(start); // 入队起始节点
        visited[start] = true; // 标记为已访问

        // 修改3：开始 BFS 循环，直到队列为空
        while let Some(current) = queue.pop_front() {
            // 出队当前节点
            visit_order.push(current); // 记录当前节点
                                       // 遍历所有与当前节点相邻的节点
            for &neighbor in &self.adj[current] {
                // 如果相邻节点未访问，则入队并标记
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor] = true;
                }
            }
        }

        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
