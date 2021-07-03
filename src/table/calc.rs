use super::*;

impl Table {
    /// Calculate recursive
    pub fn calc(
        x: usize,
        y: usize,
        tree_table: &mut Vec<Vec<Box<ThreadSafeNode>>>,
        ref_src_table: &mut Vec<Vec<Vec<(usize, usize)>>>,
        dependents_table: &mut Vec<Vec<HashMap<(usize, usize), u32>>>,
        calculated_table: &mut Vec<Vec<Value>>,
        walked_position: &mut Vec<(usize, usize)>,
    ) {
        walked_position.push((x, y));

        calculated_table[y][x] =
            if ref_src_table[y][x]
                .iter()
                .fold(false, |b, (x_of_target, y_of_target)| {
                    b || walked_position.contains(&(*x_of_target, *y_of_target))
                })
            {
                Value::Error
            } else {
                // Calculate current item
                let (calculated_value, old_dependents, new_dependents) =
                    tree_table[y][x].calc(&calculated_table);

                // Increment reference count.
                for dependent in new_dependents {
                    let entry = dependents_table[y][x].entry(dependent).or_insert(0);

                    if *entry == 0 {
                        ref_src_table[dependent.1][dependent.0].push((x, y));
                    }

                    *entry += 1;
                }

                // Decrement reference count.
                for dependent in old_dependents {
                    if let Some(n) = dependents_table[y][x].get_mut(&dependent) {
                        *n -= 1;

                        if *n == 0 {
                            ref_src_table[dependent.1][dependent.0]
                                .retain(|&(x_src, y_src)| x_src == x && y_src == y);
                        }
                    }
                }

                // Sweep unnecessary item
                dependents_table[y][x].retain(|_, v| *v != 0);

                calculated_value
            };

        for i in 0..ref_src_table[y][x].len() {
            let (x_of_target, y_of_target) = ref_src_table[y][x][i];
            if !walked_position.contains(&(x_of_target, y_of_target)) {
                Self::calc(
                    x_of_target,
                    y_of_target,
                    tree_table,
                    ref_src_table,
                    dependents_table,
                    calculated_table,
                    walked_position,
                );
            }
        }
    }
}
