// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Integer,
        #[max_length = 255]
        course_name -> Varchar,
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
    users_courses (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        course_id -> Nullable<Integer>,
    }
}

diesel::joinable!(users_courses -> courses (course_id));
diesel::joinable!(users_courses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    users,
    users_courses,
);
