use crate::connect::establish_connection;

pub fn complete_task(title: String) {
    let mut client = establish_connection();

    client
        .execute(
            "UPDATE tasks SET completed = $1 WHERE title = $2",
            &[&true, &title],
        )
        .expect("Failed to complete task");

    println!("✅ Task completed: {}", title);
}
