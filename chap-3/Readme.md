## Notes on Memory Ordering

Compiler and processor try to execute code with optimization that means that code execution is not ordered. This is okay if we are working with the main thread only, as the optimization or cache would not create any side-effects. For example, we  have two variables a and then b, but if the processor has a cache of b then it might execute b before a. 

### Memory Ordering in Rust

Memory Ordering in rust is very much similar or inspired from C++. All the Atomic operations take memory ordering enum as an argument. Mostly there are three types of ordering in rust

- Relaxed Ordering- `Ordering::Relaxed`
- Release and Acquire Ordering - `Ordering::{Release, Acquire, AcqRel}`
- Sequentially consistant Ordering - `Ordering::Seqcst`

