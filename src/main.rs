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
fn generate_subgroup(n: u64, generator: u64) -> Vec<u64> {
    let mut subgroup = Vec::new();
    let mut current = generator;
    while !subgroup.contains(&current) {
        subgroup.push(current);
        current = (current * generator) % n;
    }
    subgroup.sort_unstable();
    subgroup
}

// Function to generate all subgroups of Z_n^*
fn generate_all_subgroups(n: u64) -> Vec<Vec<u64>> {
    let elements = multiplicative_group_elements(n);
    let mut subgroups = Vec::new();
    let mut seen_subgroups = HashSet::new();

    // Add the full group as a potential subgroup
    if !elements.is_empty() {
        subgroups.push(elements.clone());
        seen_subgroups.insert(elements.clone());
    }

    for &e in &elements {
        let subgroup = generate_subgroup(n, e);
        if !seen_subgroups.contains(&subgroup) {
            seen_subgroups.insert(subgroup.clone());
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
        println!("{:?}", subgroup);
    }
}

