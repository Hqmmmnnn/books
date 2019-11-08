table! {
    authors (id) {
        id -> Int4,
        fio -> Varchar,
        date_of_birth -> Varchar,
        country -> Varchar,
    }
}

table! {
    books (id) {
        id -> Int4,
        user_id -> Int4,
        author_id -> Int4,
        name -> Varchar,
        price -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        role -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users_books (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
        amount -> Int4,
    }
}

joinable!(books -> authors (author_id));
joinable!(books -> users (user_id));
joinable!(users_books -> books (book_id));
joinable!(users_books -> users (user_id));

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    users,
    users_books,
);
