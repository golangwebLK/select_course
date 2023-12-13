// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Nullable<Integer>,
        #[max_length = 255]
        class_name -> Varchar,
        start_time -> Nullable<Datetime>,
        end_time -> Nullable<Datetime>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        student_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users_classes (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        class_id -> Nullable<Integer>,
    }
}

diesel::joinable!(users_classes -> classes (class_id));
diesel::joinable!(users_classes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    users,
    users_classes,
);
