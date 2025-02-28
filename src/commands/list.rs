use crate::connect::establish_connection;

pub fn list_tasks() {
    let mut client = establish_connection();

    let tasks = client
        .query("SELECT id, title, completed FROM tasks", &[])
        .expect("Failed to select tasks");

    for task in tasks {
        let id: i32 = task.get(0);
        let title: String = task.get(1);
        let completed: bool = task.get(2);

        let status = if completed { "✅" } else { "❌" };

        println!("{} [{}] {}", id, status, title);
    }
}
