## Omni IDL
This project is my attempt at creating a WASM based [Interface Description Language](https://en.wikipedia.org/wiki/Interface_description_language) for generating REST/GraphQL Clients and Servers. Heavily inspired by [aws-smithy](https://smithy.io/2.0/index.html)

## Problem
Developers invest significant time and effort into crafting repetitive "interface," "controller," or "handler" code, which includes tasks such as validation, deserialization, serialization, managing request parameters, request bodies, and error messages. This code redundancy is prevalent across various services. Additionally because we often utilise so many framework specific functions in this process, it makes it difficult to transition to a new vendor or framework in future.

## Solution
Define your API interface in a platform-agnostic IDL and generate all your service controllers based on this.


## Design Goals
- Provide a new expressive language that allows users to quickly and efficiently define an interface for their web services.
- Provide codegen tooling for generating clients and servers in various languages based on the IDL.
- Tooling should utilise a WASM runtime to allow plugins to be written in any language that compiles to WASM.
- Tooling should also support a human readable JSON AST (which should be convertible to/from the Omni IDL)


