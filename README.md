This Rust Program  determines and prints the subgroups of the multiplicative group Z*_n.

It contains the following;
1. The gcd function which computes the greatest common divisor using the Euclidean algorithm.
2. The multiplicative_group_elements function which finds all integers in the range 1to n-1 that are relatively prime to n.
3. The generate_subgroup function generates a cyclic subgroup starting from a given generator. It multiplies the generator by itself modulo n until it cycles back to the starting point.
4. The generate_all_subgroups function iterates over all elements of Z*_n, generating subgroups for each element not already seen in any previous subgroup.
   It uses a HashSet to keep track of elements that have already been included in subgroups.
5. he main function sets the value of n and prints all the subgroups of Z*_n, sorting each subgroup before printing for better readability.

   git clone https://github.com/cypriansakwa/Subgroups_of_Multiplicative_Group_of_Units.git
   cd Subgroups_of_Multiplicative_Group_of_Units
