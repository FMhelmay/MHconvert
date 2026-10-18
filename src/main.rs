use postgres::{Client, NoTls, Error};
use std::process::Command;

let mut client = Client::connect("
    host=127.0.0.1
    port=5432
    user=postgres
    password=Deltaforce
    dbname=sql_filebase_branch_O_FW", NoTls).unwrap();

trait status_changer {
    fn change(&self)
}

struct WorkInfo{
    path: &str,
    task: &str,
    status: &bool,
    id: &i16,
}


fn main() {
    println!("STARTED");
    let TESTS_command: &str = "ffmpeg -y -i \"Chips Lace？! Silksong Mods Are CRAZY… [66KnSJw5xXM].webm\" \"Chips Lace? HK-S.mp4\"";
    let TESTS_path: &str = "/home/mplhelmay/Documents/test/sql_testing2";


    if let Some(nf_summ_respone_sql) = get_data_sql(&mut client) {
        let summ_respone_sql = WorkInfo{
            task: nf_summ_respone_sql.task.clone(),
            path: nf_summ_respone_sql.path.clone(),
            status: nf_summ_respone_sql.status,
            id: nf_summ_respone_sql.id,
            is_arhive: nf_summ_respone_sql.is_arhive};
        println!("------------------\n");
        println!("Task:\n{}\n", summ_respone_sql.task);
        println!("Status:\n{}\n", summ_respone_sql.status);
        println!("Path:\n{}\n", summ_respone_sql.path);
        println!("Id:\n{}\n", summ_respone_sql.id);
        println!("Is arhive?\n{}\n", summ_respone_sql.is_arhive);
    }
    else {
        println!("No data, adding test data");
        let add_data = WorkInfo{task: command.to_string(), status: false, path: path.to_string(), id: 15, is_arhive: false};
        add_data_sql(&mut client, &add_data);
    }

let task_data = get_data_sql(&mut client).unwrap();

claim_data_sql(&mut client, &task_data.id);
run_command(&task_data.task, &task_data.path);
remove_data_sql(&mut client, &task_data.id);
println!("ENDED");
}


fn run_command(command: &str, path: &str) {
    let status = Command::new("sh").arg("-c").current_dir(&path).arg(&command).status().expect("Не удалось запустить процесс");
}

fn get_data_sql(client: &mut Client) -> Option<WorkInfo> {
    let mut respone_sql = client.query("SELECT * FROM tasks WHERE status = false ORDER BY id ASC LIMIT 1;", &[]).unwrap();

    if respone_sql.is_empty() {
        None}
    else {
        let respone_row_sql = respone_sql.remove(0);
        Some(WorkInfo{
            task: respone_row_sql.get(0),
            path: respone_row_sql.get(1),
            id: respone_row_sql.get(2),
            status: respone_row_sql.get(3),
            is_arhive: respone_row_sql.get(4)
        })
    }
}


//Add data
fn add_data_sql(client: &mut Client, data: &WorkInfo) {
    client.execute("INSERT INTO tasks VALUES ($1, $2, $3, $4, $5);", &[&data.task, &data.status, &data.path, &data.id, &data.is_arhive]);
}

//Take data
fn claim_data_sql(client: &mut Client, id: &i16) {
    client.execute("UPDATE tasks SET status = true WHERE id = $1;", &[&id]);
}

//Delete data
fn remove_data_sql(client: &mut Client, id: &i16) {
    client.execute("DELETE FROM tasks WHERE id = $1;", &[&id]);
}
