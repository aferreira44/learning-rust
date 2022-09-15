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