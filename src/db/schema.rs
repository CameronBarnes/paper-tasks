// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    task_categories (task_id, category_id) {
        task_id -> Integer,
        category_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        scheduled_at -> Nullable<Text>,
        recurrence_rule -> Nullable<Text>,
        archived -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(task_categories -> categories (category_id));
diesel::joinable!(task_categories -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(categories, task_categories, tasks,);
