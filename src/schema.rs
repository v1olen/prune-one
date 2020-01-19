table! {
    bridges (id) {
        id -> Int4,
        slug -> Varchar,
        target_url -> Varchar,
        active -> Bool,
    }
}
