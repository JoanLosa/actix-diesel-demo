// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author_id -> Int4,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        page_number -> Int4,
        content -> Text,
        book_id -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(books -> authors (author_id));
diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    pages,
    posts,
);
