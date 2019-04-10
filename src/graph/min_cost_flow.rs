pub mod primal_dual {
    use std::cmp;
    use std::collections::BinaryHeap;
    use std::i64;
    type Flow = i64;
    type Cost = i64;
    const INF: Cost = i64::MAX >> 3;
    struct Edge {
        to: usize,
        capacity: Flow,
        flow: Flow,
        cost: Cost,
        reverse_to: usize,
        is_reversed: bool,
    }
    impl Edge {
        fn residue(&self) -> Flow {
            self.capacity - self.flow
        }
    }

    pub struct MinimumCostFlowSolver {
        graph: Vec<Vec<Edge>>,
        previous_edge: Vec<(usize, usize)>,
    }

    impl MinimumCostFlowSolver {
        pub fn new(n: usize) -> Self {
            MinimumCostFlowSolver {
                graph: (0..n).map(|_| Vec::new()).collect(),
                previous_edge: vec![(0, 0); n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, capacity: Flow, cost: Cost) {
            let reverse_from = self.graph[to].len();
            let reverse_to = self.graph[from].len();
            self.graph[from].push(Edge {
                to: to,
                capacity: capacity,
                flow: 0,
                cost: cost,
                reverse_to: reverse_from,
                is_reversed: false,
            });
            self.graph[to].push(Edge {
                to: from,
                capacity: capacity,
                flow: capacity,
                cost: -cost,
                reverse_to: reverse_to,
                is_reversed: true,
            });
        }

        pub fn solve(&mut self, source: usize, sink: usize, mut flow: Flow) -> Option<Flow> {
            let n = self.graph.len();
            let mut result = 0;
            let mut h = vec![0; n];
            let mut q: BinaryHeap<(Cost, usize)> = BinaryHeap::new();
            while flow > 0 {
                let mut dist = vec![INF; n];
                dist[source] = 0;
                q.push((0, source));
                while let Some((current_dist, v)) = q.pop() {
                    if dist[v] < current_dist {
                        continue;
                    }
                    for (i, e) in self.graph[v].iter().enumerate() {
                        if e.residue() == 0 {
                            continue;
                        }
                        if dist[e.to] + h[e.to] > current_dist + h[v] + e.cost {
                            dist[e.to] = current_dist + h[v] + e.cost - h[e.to];
                            self.previous_edge[e.to] = (v, i);
                            q.push((dist[e.to], e.to));
                        }
                    }
                }

                if dist[sink] == INF {
                    return None;
                }

                for i in 0..n {
                    h[i] += dist[i];
                }
                let mut df = flow;
                let mut v = sink;
                while v != source {
                    let (prev_v, prev_e) = self.previous_edge[v];
                    df = cmp::min(df, self.graph[prev_v][prev_e].residue());
                    v = prev_v;
                }
                flow -= df;
                result += df * h[sink];
                let mut v = sink;
                while v != source {
                    let (prev_v, prev_e) = self.previous_edge[v];
                    self.graph[prev_v][prev_e].flow += df;
                    let reversed_edge_id = self.graph[prev_v][prev_e].reverse_to;
                    self.graph[v][reversed_edge_id].flow -= df;
                    v = prev_v;
                }
            }
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helper::TestCaseProducer;

    #[test]
    fn solve_grl_6_b() {
        let mut input = TestCaseProducer::new_from_directory("./assets/GRL_6_B/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/GRL_6_B/out/");

        while !input.is_empty() {
            let v: usize = input.next();
            let e: usize = input.next();
            let f: i64 = input.next();

            let mut solver = primal_dual::MinimumCostFlowSolver::new(v);
            for _ in 0..e {
                let u: usize = input.next();
                let v: usize = input.next();
                let c: i64 = input.next();
                let d: i64 = input.next();
                solver.add_edge(u, v, c, d);
            }

            let ans = match solver.solve(0, v - 1, f) {
                Some(flow) => flow,
                None => -1,
            };
            assert_eq!(ans, output.next::<i64>());
        }
    }
}
