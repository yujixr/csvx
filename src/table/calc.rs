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
                for (dependent_x, dependent_y) in new_dependents {
                    if dependent_y < dependents_table.len()
                        && dependent_x < dependents_table[dependent_y].len()
                    {
                        let entry = dependents_table[y][x]
                            .entry((dependent_x, dependent_y))
                            .or_insert(0);

                        if *entry == 0 {
                            ref_src_table[dependent_y][dependent_x].push((x, y));
                        }

                        *entry += 1;
                    }
                }

                // Decrement reference count.
                for (dependent_x, dependent_y) in old_dependents {
                    if let (true, true, Some(n)) = (
                        dependent_y < dependents_table.len(),
                        dependent_x < dependents_table[dependent_y].len(),
                        dependents_table[y][x].get_mut(&(dependent_x, dependent_y)),
                    ) {
                        *n -= 1;

                        if *n == 0 {
                            ref_src_table[dependent_y][dependent_x]
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
            let n_walks = walked_position.iter().fold(0, |b, walked_pos| {
                if walked_pos == &(x_of_target, y_of_target) {
                    b + 1
                } else {
                    b
                }
            });

            if n_walks < 2 {
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
