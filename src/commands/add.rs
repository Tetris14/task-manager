use crate::connect::establish_connection;

pub fn add_task(title: String) {
    let mut client = establish_connection();

    client
        .execute(
            "INSERT INTO tasks (title, completed) VALUES ($1, $2)",
            &[&title, &false],
        )
        .expect("Failed to insert task");

    println!("✅ Task added: {}", title);
}
