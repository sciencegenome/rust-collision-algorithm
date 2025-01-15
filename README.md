# rust-collision-algorithm
 - hash collision free graphlookup table. 
 - finding the offset and then i thought since there will be collision hashes,so i implemented and devised this that make the searched space unique and then use that to calculate the hash-indices and then if that is equal and also
  the unique count then the hashes are the same.
 - So my algorithm first find the hashes of the search string, then since there might be collisions also, so to avoid the collision, i implmented a way that it will make the graph offset key values unique and then multiply with the ASCII
   code and then it does the same for the find iter and then if the hashes are the same and also the unqiue value then it put them into the BTreeMap.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.
```
cargo build

```

```
➜ gauravsablok  rust-collision-algorithm git:(main) ✗ ./target/debug/rust-collision-algorithm -h
Usage: rust-collision-algorithm <FASTQFILE> <OFFSETSIZE>

Arguments:
  <FASTQFILE>   please provide the path to the fastq file
  <OFFSETSIZE>  please provide the kmer lookup table construction

Options:
  -h, --help     Print help
  -V, --version  Print version

```



 Gaurav Sablok
