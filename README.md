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


### Example - A Bookshop service
To demonstrate an early usage of Omni IDL we will build a bookshop service. 

#### Features of the bookshop service
* CREATE a new book store
* CREATE a new book
* BULK CREATE many books
* GET a book by id
* GET a book by title
* GET a book by bookstore id and book id
* DELETE a book by id
* GET all books in a bookstore 

#### entities.omni
* first we will define the `entities` the bookstore service provides access to.

- `.omni` extension for IDL files.

```entities.omni
use { Geospatial } from 'github.com/someuser/omni-geospatial'


service BookStore {
    version: "1.0.0"

    // Defines 2 top level resources the service will provide
    // e.g /book-shops/2 /books/1
    resources: [BookShop, Book]
}

// Comments are supported
resource BookShop {
    properties: {
        name: String,
        address: Geospatial,
    }

    operations: [
        GetBookshopById,
        GetBookshops,
        GetBookshopBookById,
        ListBookshopBooks,
        CreateBook,
        BulkCreateBooks
    ]

    // Child entities in an array
    // REST: /book-shops/{shopId}/books/{bookId}
    // GraphQL: query getBook{ shop(id: shopId) { book(id: bookId) { name }}}
    children: [
        Book,
    ]
}

operation GetBookshopsById {
    input: {
        @id
        id: String
    }

    output: {

    }
}
operation ListBookshops {
}

resource Book {
    properties: {
        name: String
    }
}
```
