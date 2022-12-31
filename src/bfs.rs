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
}
