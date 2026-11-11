# code-gen

This library aids in code generation.

This library is concerned with generating source code as strings. It provides utilities for easily
composing code in various programming languages. It provides no functionality for code analysis,
file I/O, or formatting.

## Dependencies & Features

    code-gen = "0.12.1"

This crate has no dependencies.

### Features

    c             # support for the C programming language
    cpp           # support for the C++ programming language
    csharp        # support for the C# programming language
    go            # support for the Go programming language
    java          # support for the Java programming language
    kotlin        # support for the Kotlin programming language
    python        # support for the Python programming language
    rust          # support for the Rust programming language
    typescript    # support for the TypeScript & JavaScript programming languages

## Example

```rust
use code_gen::rust::*;
use code_gen::*;

let mut s = Struct::from("Message");
s.set_access(Access::Public);
s.add_derive("Clone");
s.add_derive("Debug");
s.add_field(StructField::from(Var::from(("id", "u64"))).with_access(Access::Public));
s.add_field(StructField::from(Var::from(("body", "String"))).with_access(Access::Public));

let code: String = s.to_code();
```

Output:

```rust
#[derive(Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub body: String,
}
```
