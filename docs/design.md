# Design Overview

The overall goal of Omni is to provide a simple but extensible IDL (OmniDL) and accompanying parser which will consume the IDL and produce an in-memory representation of the Omni schema for further processing by tooling.

The primary purpose of tooling will be code generation. Omni was created in order to reduce boilerplate "glue" code that gets in the way of your business logic in the client/server.

## OmniDL

TODO:

## CLI

The CLI will be the main driver for interacting with an Omni project, responsible for parsing, dependency management and code gen.

## Server Code Generation

An Omni schema can generate various Services in a target programming language. There are 2 components to the server code generation

- The SDK - this is the core logic shared between all frameworks, it is responsible for validation, serialization, deserialization and error handling at the "edge" or your service.
- The Framework - optional framework specific code that calls the SDK to validate and parse incoming requests


## Traits
There are many types of traits in an omni model.

* Model traits - these traits apply constraints to the Omni model itself, and do not affect code generation. Think of them like static analysis applied to your Omni project.
* Documentation traits - these traits are used only to document services, they can be used to generate API documentation for example
* Transformer traits - these traits apply transformations to the generated types in a pipeline like fashion
* Validation traits - these traits apply runtime validation to the generated types and throw defined Errors
