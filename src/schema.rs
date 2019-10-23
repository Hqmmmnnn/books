table! {
    use diesel::sql_types::Int4;
    use diesel::sql_types::Varchar;
    use diesel::sql_types::Nullable;
    books (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        author -> Varchar,
        price -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(books -> users (user_id));

allow_tables_to_appear_in_same_query!(books, users,);
