## Overview
This Rust project computes and lists all the cyclic subgroups of the multiplicative group $\mathbb{Z}_n^\*$, where $\mathbb{Z}_n^\*$ is the set of integers less than 
$n$ that are coprime with $n$. Given a positive integer $n$, the code will:
- Identify the elements of the multiplicative group $\mathbb{Z}_n^\*$.
- Generate all cyclic subgroups of $\mathbb{Z}_n^\*$ by using each element as a potential generator.
- Ensure that only unique subgroups are included in the final list.
- Display the subgroups.

## How it works
1. The gcd function which computes the greatest common divisor using the Euclidean algorithm.
2. The multiplicative_group_elements function which finds all integers in the range 1to $n-1$ that are relatively prime to n.
3. The generate_subgroup function generates a cyclic subgroup starting from a given generator. It multiplies the generator by itself modulo n until it cycles back to the starting point.
4. The generate_all_subgroups function iterates over all elements of Z*_n, generating subgroups for each element not already seen in any previous subgroup.
   It uses a HashSet to keep track of elements that have already been included in subgroups.
5. he main function sets the value of n and prints all the subgroups of Z*_n, sorting each subgroup before printing for better readability.

 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing the subgroups of the specified $\mathbb{Z}_n^\*$.
## Customizing
- To compute subgroups for a different value of $n$, simply modify the value of $n$ in the main function.
## Example Output
- If you set $n = 8$ in the main.rs file, the output will be:
>```
>Subgroups of Z_8^*:
>[1, 3, 5, 7]
>[1]
>[1, 3]      
>[1, 5]      
>[1, 7]  
- This output lists all subgroups of $\mathbb{Z}_{8}^\*$, with each subgroup represented as a sorted list of elements.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Subgroups_of_Multiplicative_Group_of_Units.git
   cd Subgroups_of_Multiplicative_Group_of_Units
