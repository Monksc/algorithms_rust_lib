

pub fn double_mapping_into_vector<T : std::hash::Hash + Eq + Copy, I> (
    iter: I
) -> (Vec<T>, std::collections::HashMap<T, usize>)
    where I : Iterator<Item=T>
{
    let mut vertice_to_index = std::collections::HashMap::new();
    let mut index_to_vertice = Vec::new();

    for value in iter {
        if !vertice_to_index.contains_key(&value) {
            vertice_to_index.insert(value, index_to_vertice.len());
            index_to_vertice.push(value);
        }
    }

    return (index_to_vertice, vertice_to_index);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_double_mapping_into_vector() {
        let v = vec![
            9, 3, 4, 9, 3, 29
        ];

        assert_eq!(
            double_mapping_into_vector(
                v.into_iter()
            ),
            (
                vec![
                    9, 3, 4, 29
                ],
                std::collections::HashMap::from([
                    (9,  0),
                    (3,  1),
                    (4,  2),
                    (29, 3),
                ]),
            )
        );
    }
}
