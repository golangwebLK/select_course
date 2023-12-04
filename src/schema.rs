// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        name -> Longtext,
        enable -> Bool,
        note -> Nullable<Longtext>,
        speciality_id -> Unsigned<Bigint>,
        class_time_start -> Datetime,
        class_time_end -> Datetime,
    }
}

diesel::table! {
    course_details (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        course_id -> Unsigned<Bigint>,
        class_id -> Unsigned<Bigint>,
        note -> Nullable<Longtext>,
    }
}

diesel::table! {
    courses (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        class_id -> Unsigned<Bigint>,
        student_data_id -> Unsigned<Bigint>,
        note -> Nullable<Longtext>,
        total_courses -> Unsigned<Bigint>,
        status -> Bool,
    }
}

diesel::table! {
    specialities (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        name -> Longtext,
        enable -> Bool,
        note -> Nullable<Longtext>,
    }
}

diesel::table! {
    student_data (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        name -> Longtext,
        age -> Bigint,
        school -> Nullable<Longtext>,
        telephone -> Longtext,
        sex -> Longtext,
        status -> Nullable<Bool>,
        note -> Nullable<Longtext>,
        specialities -> Nullable<Longtext>,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 256]
        password -> Varchar,
    }
}

diesel::table! {
    student_details (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        student_data_id -> Unsigned<Bigint>,
        class_id -> Unsigned<Bigint>,
        note -> Nullable<Longtext>,
    }
}

diesel::table! {
    sys_menus (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        menuname -> Longtext,
        target -> Nullable<Longtext>,
        parent_id -> Unsigned<Bigint>,
        icon -> Longtext,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        #[max_length = 191]
        username -> Varchar,
        password_hash -> Longtext,
        enabled -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    course_details,
    courses,
    specialities,
    student_data,
    student_details,
    sys_menus,
    users,
);
