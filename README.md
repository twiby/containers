# containers
Some ideas for useful custom containers


### SparseSet and StaticSparseSet
SparseSet is an efficient data structure with amortized constant cost for insertion, deletion, and fetching. 
It is very much like a Vec where every `remove` is a `swap_remove`. The particularity of this structure is that 
every insertion returns an index that is stable throughout the whole life of the container. Th only drawback is 
that contrary to `Vec::get`, `SparseSet::get` requires 2 indirections.

In effect this looks like a map, where indices are given to you at insertion, and not specified by the user.
Also no hashing of complex algorithms happen.

Finally, all the data is guaranteed to be continuous in memory, making iteration through the set as efficient 
as possible, though the ordering is not preserved.

2 versions exist: `SparseSet` for which memory is dynamically allocated on the heap, and `StaticSparseSet` 
for which every memory is on the stack and allocated up front (its size is specified by a generic parameter).

This is a Rust implementation for something we developed with https://github.com/tle-huu in C++.