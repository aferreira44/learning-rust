https://blog.thoughtram.io/rust/2015/05/11/rusts-ownership-model-for-javascript-developers.html

- Most languages (JavaScript included) use a **garbage collector** to ensure **memory safety**.

## what’s the job of a garbage collector anyway?
 
Basically it frees up memory that isn’t used anymore, that is, memory that nothing in the program points to anymore. C and C++ delegate that work to the developer.

### Problems that may arise with manual memory management

- **Memory leaks**. If you forget to free up memory, it will be used up and the program will crash.
- **Dangling pointers**. If you free up memory that is still being used, the program will crash.
- **Use after free**. If you use a pointer after you’ve freed up the memory it points to, the program will crash.
- **Double free**. If you free up memory that has already been freed, the program will crash.

## Lifetime annotations

## #[derive(Debug)]

## Traits

https://blog.thoughtram.io/ownership-in-rust/

## Stack and Heap

- A stack, where values are stored in order as they come in, and removed in the opposite order (which are very fast operations)
- A heap is more like a tree structure that requires a bit more computational effort to read and write data.

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

## [Box Pointers](https://doc.rust-lang.org/book/ch15-02-deref.html?highlight=Box%3CT%3E#defining-our-own-smart-pointer)

## Moves and Borrowing

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

https://willcrichton.net/notes/rust-memory-safety/

https://blog.thoughtram.io/references-in-rust/

## deref coercing

https://blog.thoughtram.io/string-vs-str-in-rust/


