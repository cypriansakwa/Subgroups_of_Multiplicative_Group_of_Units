use std::collections::HashSet;

// Function to compute the greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to find all elements of the multiplicative group Z_n^*
fn multiplicative_group_elements(n: u64) -> Vec<u64> {
    (1..n).filter(|&x| gcd(x, n) == 1).collect()
}

// Function to generate a cyclic subgroup of Z_n^* given a generator
fn generate_subgroup(n: u64, generator: u64) -> HashSet<u64> {
    let mut subgroup = HashSet::new();
    let mut current = generator;
    while !subgroup.contains(&current) {
        subgroup.insert(current);
        current = (current * generator) % n;
    }
    subgroup
}

// Function to generate all subgroups of Z_n^*
fn generate_all_subgroups(n: u64) -> Vec<HashSet<u64>> {
    let elements = multiplicative_group_elements(n);
    let mut subgroups = Vec::new();
    let mut seen: HashSet<u64> = HashSet::new(); // Type annotation added

    for &e in &elements {
        if !seen.contains(&e) {
            let subgroup = generate_subgroup(n, e);
            seen.extend(&subgroup);
            subgroups.push(subgroup);
        }
    }
    subgroups
}

fn main() {
    let n = 8; // Replace with the desired value of n
    let subgroups = generate_all_subgroups(n);
    println!("Subgroups of Z_{}^*:", n);
    for subgroup in subgroups {
        let mut subgroup_vec: Vec<u64> = subgroup.into_iter().collect();
        subgroup_vec.sort_unstable();
        println!("{:?}", subgroup_vec);
    }
}

