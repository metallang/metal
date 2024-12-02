# Metal Architecture

This is a high-level overview and generalization of Metal's core architecture. It is not representative of how different components of Metal interact with each other,
and is definitely not a standard. This is purely for education and as a high-level language resource.

Metal is split up into four general pieces:

- User frontend (`drill`, `metallic`, etc.)
- Compiler frontend (`metallic`, `metal-parser`, `metal-lexer`, `metal-ast`, etc.)
- Compiler middle-end (`metallic`, `metal-mir`)
- Compiler backend (`metallic`, `metal_codegen_llvm`, etc.)

These all have a hand at making sure Metal code is efficient, readable, and user-friendly. It also represents the multiple levels at which Metal code is desugared:

- 1. Full Metal code
- 2. Converted into AST, parsed and macros are compiled
- 3. AST transfers to the mostly desugared MIR representation
- 4. MIR transfers into LLVM IR which can be compiled to machine code, WASM, etc.

## User Frontend

The user frontend largely represents the `drill` CLI tool. It is the central CLI of Metal and handles everything from package management to formatting and installation of Metal projects.

It largely serves as a easier-to-use, less sophisticated frontend to `metallic` as metallic is aimed to be the compiler frontend and only that.

## Compiler Frontend

The compiler frontend parses Metal code and makes sure at a basic level it is syntactically correct, as well as converting the raw text into an AST representation to make the entire process easier.

This process is also where macros are parsed, interpreted, then compiled into their replaced syntax forms.

Note for `metallic` specifically: the frontend does not parse dependencies. Only a list of module files which may depend on each other.

## Compiler Middle-end

The middle end is where Metal AST representations are deconstructed and desugared. More verification may also happen in this area, like dead code checks or other optimizations possible with such a specific representation.

## Compiler Backend

The backend takes the final MIR (Mid-level Intermediate Representation) and turns it into LLVM IR which can essentially be run universally as well as be easily cross-compiled.

