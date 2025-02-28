use crate::connect::establish_connection;

pub fn remove_task(title: String) {
    let mut client = establish_connection();

    client
        .execute("DELETE FROM tasks WHERE title = $1", &[&title])
        .expect("Failed to remove task");

    println!("❌ Task removed: {}", title);
}
