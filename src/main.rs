use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
mod read_amazon_meta_file;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}
// C:/Users/romus/Downloads/DS210/pagerank/pagerank_data
//test number of nodes
//test if adjacencylist is accurate
fn read_file(path: &str) -> (usize, Vec<(usize, usize)>) {
    let mut first_line = true;
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut num_edges = 0usize;
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        if first_line {
            num_edges = line_str.parse::<usize>().unwrap();
            first_line = false;
        } else {
            let v: Vec<&str> = line_str.trim().split("\t").collect();
            let x: usize = v[0].parse::<usize>().unwrap();
            let y: usize = v[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
    }
    return (num_edges, result);
}

impl Graph {
    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    fn random_walk(&self, v: Vertex) -> usize {
        let mut final_node = v;
        for _steps in 0..100usize {
            // if the node doesn't have neighbord or we hit the 10% random jump case
            if self.outedges[final_node].len() == 0 || rand::thread_rng().gen_range(0..100) > 90 {
                final_node = rand::thread_rng().gen_range(0..self.n);
            } else {
                let index = rand::thread_rng().gen_range(0..self.outedges[final_node].len());
                final_node = self.outedges[final_node][index];
            }
        }
        final_node
    }

    fn build_page_rank(&self) -> Vec<(usize, usize)> {
        let mut dest_counts = Vec::<(usize, usize)>::new();
        for v in 0..self.n {
            dest_counts.push((0, v));
        }
        for v in 0..self.n {
            for _walk in 0..100 {
                let final_node = self.random_walk(v);
                dest_counts[final_node].0 += 1;
            }
        }
        dest_counts.sort();
        dest_counts.reverse();
        dest_counts
    }
}

fn main() {
   // let args: Vec<String> = env::args().collect();
   // let file_path = &args[1];
    let (size, edges) = read_file("C:/Users/romus/Downloads/DS210/pagerank/Amazon0601.txt");

    let graph = Graph::create_directed(size, &edges);
    let dest_counts = graph.build_page_rank();
   
    // put the nearest neighbors of the inputted vertex to the third degree in a vector
    let mut neighbors: Vec<Vec<Vertex>> = vec![vec![];graph.n];
    for v in 0..graph.n {
        for u in graph.outedges[v].iter() {
            neighbors[v].push(*u);
            neighbors[v].sort();
            neighbors[v].dedup();
        }
    };
    // Now print the top 5 nodes, their pageranks, and their corresponding Amazon product descriptions
    for v in 0..5usize {
        let pagerank: f64 = dest_counts[v].0 as f64 / (100 * size) as f64;
        println!(
            "Vertex {}: approximate PageRank {}",
            dest_counts[v].1, pagerank
        );
        read_amazon_meta_file::read_amazon_meta(dest_counts[v].1);
        println!("Customers who liked product {:?} may also like {:?}", dest_counts[v].1, neighbors[dest_counts[v].1]);

    }
}
// cargo test without super. create directed graph and test if its neighbors and its amazon product description are correct
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_directed_graph(){
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let new_graph = Graph::create_directed(4, &edges);
        // test if number of nodes is correct
        assert_eq!(new_graph.n, 4);
        // test if edges are correct
        assert_eq!(new_graph.outedges[0], vec![1]);
        assert_eq!(new_graph.outedges[1], vec![2]);
        assert_eq!(new_graph.outedges[2], vec![3]);
        assert_eq!(new_graph.outedges[3], vec![0]);
        // test neighbors
        let mut neighbors: Vec<Vec<Vertex>> = vec![vec![];new_graph.n];
        for v in 0..new_graph.n {
            for u in new_graph.outedges[v].iter() {
                neighbors[v].push(*u);
                neighbors[v].sort();
                neighbors[v].dedup();
            }
        };
        assert_eq!(neighbors[0], vec![1]);
        assert_eq!(neighbors[1], vec![2]);
        assert_eq!(neighbors[2], vec![3]);
        assert_eq!(neighbors[3], vec![0]);
        // test amazon product description
        read_amazon_meta_file::read_amazon_meta(0);
        read_amazon_meta_file::read_amazon_meta(1);
        read_amazon_meta_file::read_amazon_meta(2);
        read_amazon_meta_file::read_amazon_meta(3);
        assert_eq!(read_amazon_meta_file::read_amazon_meta(0), "".to_string());
        assert_eq!(read_amazon_meta_file::read_amazon_meta(1), "Patterns of Preaching: A Sermon Sampler".to_string());
        assert_eq!(read_amazon_meta_file::read_amazon_meta(2), "Candlemas: Feast of Flames".to_string());
        assert_eq!(read_amazon_meta_file::read_amazon_meta(3), "World War II Allied Fighter Planes Trading Cards".to_string());

    }
}
