use queues::*;
use std::collections::HashMap;

pub fn bfs<
    T : std::hash::Hash + Eq + Clone,
    N: FnMut(T) -> Vec<T>,
    G: FnMut(T) -> bool
>(
    start_pos: T,
    mut next_state: N,
    mut has_reached_goal: G,
) -> Vec<T> {

    let mut queue: Queue<T> = queue![start_pos.clone()];
    let mut previous: HashMap<T, Option<T>> =
        HashMap::new();
    previous.insert(start_pos, None);

    while let Ok(top) = queue.remove() {
        if has_reached_goal(top.clone()) {
            let mut path: Vec<T> = Vec::new();

            let mut iter : Option<T> = Some(top);
            while let Some(next) = iter {
                path.push(next.clone());
                iter = previous[&next].clone();
            }
            path.reverse();
            return path;
        }
        for next in next_state(top.clone()) {
            if previous.contains_key(&next) {
                continue;
            }
            queue.add(next.clone());
            previous.insert(next.clone(), Some(top.clone()));
        }
    }
    return Vec::new();
}

pub fn bfs_address<
    'a,
    T : std::hash::Hash + Eq,
    N: FnMut(&'a T) -> Vec<&'a T>,
    G: FnMut(&'a T) -> bool
>(
    start_pos: &'a T,
    mut next_state: N,
    mut has_reached_goal: G,
) -> Vec<&'a T> {

    let mut queue: Queue<&'a T> = queue![start_pos];
    let mut previous: HashMap<&T, Option<&T>> =
        HashMap::new();
    previous.insert(&start_pos, None);

    while let Ok(top) = queue.remove() {
        if has_reached_goal(top) {
            let mut path: Vec<&'a T> = Vec::new();

            let mut iter : Option<&T> = Some(top);
            while let Some(next) = iter {
                path.push(next);
                iter = previous[next];
            }
            path.reverse();
            return path;
        }
        for next in next_state(top) {
            if previous.contains_key(&next) {
                continue;
            }
            queue.add(&next);
            previous.insert(&next, Some(top));
        }
    }
    return Vec::new();
}


pub fn get_parent_tree<
    T,
    K: Eq + Copy,
>(
    tree: &mut T,
    child: K
) -> K
where T : std::ops::IndexMut<K, Output=K>
{
    let parent = tree[child];
    if parent == child {
        return child
    }

    let parent = get_parent_tree(tree, parent);
    tree[child] = parent;
    return parent;
}

pub fn add_connection<
    T,
    K: Eq + Copy,
>(
    tree: &mut T,
    node1: K,
    node2: K
)
where T : std::ops::IndexMut<K, Output=K>
{
    let parent1 = get_parent_tree(tree, node1);
    let parent2 = get_parent_tree(tree, node2);
    tree[parent1] = parent2;
}

pub fn get_scc<
    T,
    I,
    M,
    K : std::hash::Hash + Eq + Copy,
>(
    vertices: M,
    graph: &T
) -> Vec<Vec<K>> 
where T : std::ops::Index<K, Output=I>,
      // I : std::iter::Iterator<Item=K> + Copy
      I : std::iter::IntoIterator<Item=K> + Clone,
      M : std::iter::IntoIterator<Item=K> + Clone
{
    let mut scc = Vec::new();

    let (mut index_to_vertice, mut vertice_to_index) =
        super::other::double_mapping_into_vector(vertices.clone().into_iter());

    let mut tree_parent = Vec::new();
    for i in 0..index_to_vertice.len() {
        tree_parent.push(i);
    }

    for vertex in vertices {
        let vertex_index = vertice_to_index[&vertex];
        for next in graph[vertex].clone() {
            let next_index = vertice_to_index[&next];
            add_connection(&mut tree_parent, vertex_index, next_index);
        }
    }

    let mut vertices_with_parents = Vec::new();
    for i in 0..tree_parent.len() { vertices_with_parents.push(Vec::new()); }
    for i in 0..tree_parent.len() {
        let parent = get_parent_tree(&mut tree_parent, i);
        vertices_with_parents[parent].push(index_to_vertice[i]);
    }

    for vertices in vertices_with_parents {
        if vertices.len() > 0 {
            scc.push(vertices);
        }
    }

    return scc;
}

// only for bidirectional graphs
pub fn get_scc_graph_vec_indexes(
    graph: &Vec<Vec<usize>>
) -> Vec<Vec<usize>>
{
    let mut scc = Vec::new();

    let mut tree_parent = Vec::new();
    for i in 0..graph.len() {
        tree_parent.push(i);
    }

    for vertex in 0..graph.len() {
        for next in &graph[vertex] {
            add_connection(&mut tree_parent, vertex, *next);
        }
    }

    let mut vertices_with_parents = Vec::new();
    for _ in 0..tree_parent.len() { vertices_with_parents.push(Vec::new()); }
    for i in 0..tree_parent.len() {
        let parent = get_parent_tree(&mut tree_parent, i);
        vertices_with_parents[parent].push(i);
    }

    for vertices in vertices_with_parents {
        if vertices.len() > 0 {
            scc.push(vertices);
        }
    }

    return scc;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_bfs() {
        let arr = vec![0,1,2,3,4,5];
        assert_eq!(
            bfs(
                &0,
                |x| vec![&arr[(*x+1)]],
                |x| *x == 5,
            ),
            vec![&0,&1,&2,&3,&4,&5],
        );
    }

    #[test]
    pub fn test_get_parent_three() {
        let mut tree : Vec<usize> = vec![
            0, 0, 1, 2, 3, 9, 9, 9, 9, 9, 9, 9, 9
        ];

        assert_eq!(get_parent_tree(&mut tree, 4 as usize), 0 as usize);
    }

    #[test]
    pub fn test_scc() {
        let graph = vec![
            vec![
                1,
            ],
            vec![
                2
            ],
            vec![
                0,
            ],
            vec![
                4
            ],
            vec![
                3
            ],
            vec![
                0
            ]
        ];

        // may not be perfect as ordering does not matter
        assert_eq!(
            get_scc_graph_vec_indexes(
                &graph
            ),
            vec![
                vec![
                    0, 1, 2, 5
                ],
                vec![
                    3, 4
                ],
            ],
        );
    }
}
