diesel::table! {
    carte (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        #[max_length = 32]
        category -> Varchar,
        #[max_length = 32]
        price -> Varchar,
    }
}