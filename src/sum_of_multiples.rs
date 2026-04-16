use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();

    for _ in factors {
        for factor in factors {
            if *factor == 0 {
                continue;
            }

            let mut last_multiplied_by = 1;

            while factor * last_multiplied_by < limit {
                set.insert(factor * last_multiplied_by);
                last_multiplied_by += 1
            }
        }
    }

    set.iter().sum()
}
