# Actix-web REST API with Diesel ORM (for Postgres)

A simple CRUD backend for storing books, pages and posts (work in progress)

## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)


## APIs

### Address: **`localhost:8080`**

### Authors `/authors`

#### Create a new author
`POST /`

body example:

```
{
    "name": "J.K.Rowling",
}
```

### Posts `/posts`

#### Create a post `POST /`

body example

```
{
    "title": "My Post",
    "body": "Lorem ipsum dolorum",
}
```

#### Get all posts `GET /`

#### Update a post `PATCH /{id}`

#### Delete a post by id `DELETE /{id}`

### Books `/books`

#### Create a new book

`POST /`

body example:

```
{
    "title": "Harry Potter",
    "author_id": 1,
    "pages": [
      {
        "page_number": 0,
        "content": "lorem ipsum dolorum"
      }
    ]
}
```

#### Get a number of books `GET /?limit=50`

#### Get one book by id `GET /{id}`



