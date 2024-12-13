use std::fs::{File, OpenOptions};
use std::string::ToString;
use csv::{Reader, Writer};


#[derive(Debug, Clone)]
pub(crate) struct Task{
    id: i32,
    pub(crate) title: String,
    pub(crate) deadline: String,
    pub(crate) done: bool,
    user_login: String
}


impl Task{
    pub(crate) fn new(title: String, deadline: String, done: bool, user_login: String) -> Self{
        let tasks_file = File::open("tasks.csv").unwrap();
        let mut reader = Reader::from_reader(tasks_file);
        let mut last_id = 0;
        for record in reader.records() {
            last_id = record.unwrap().get(0).unwrap().parse::<i32>().unwrap();
        }
        return Task{
            id: last_id + 1,
            title: title,
            deadline: deadline,
            done: done,
            user_login: user_login
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct User{
    pub(crate) login: String,
    password: String
}

impl User{
    pub(crate) fn get_tasks(&self) -> Vec<Task>{
        let mut tasks_vec = Vec::new();

        let tasks_file = File::open("tasks.csv").unwrap();
        let mut rdr = Reader::from_reader(tasks_file);
        for record in rdr.records() {
            let task = record.unwrap();
            if task.get(4).unwrap().to_string() == self.login{
                tasks_vec.push(Task{
                    id: task.get(0).unwrap().parse::<i32>().unwrap(),
                    title: task.get(1).unwrap().to_string(),
                    deadline: task.get(2).unwrap().to_string(),
                    done: task.get(3).unwrap().parse::<bool>().unwrap(),
                    user_login: self.login.clone()}
                );
            }
        }
        tasks_vec
    }

    pub(crate) fn add_task(&self, task: Task){
        let file = OpenOptions::new().write(true).append(true).open("tasks.csv").unwrap();
        let mut wrtr = Writer::from_writer(file);
        let to_write = [task.id.to_string(),
            task.title,
            task.deadline,
            task.done.to_string(),
            self.login.clone()];
        wrtr.write_record(&to_write).unwrap();
        wrtr.flush().expect("Writing failed");
    }

    pub(crate) fn delete_task(&self, task: Task) {
        let task_id = task.id;
        let mut tasks = Vec::new();
        let tasks_file = File::open("tasks.csv").unwrap();
        let mut rdr = Reader::from_reader(tasks_file);
        for record in rdr.records(){
            let task = record.unwrap();
            if task.get(0).unwrap().parse::<i32>().unwrap() != task_id{
                tasks.push(Task{
                    id: task.get(0).unwrap().parse::<i32>().unwrap(),
                    title: task.get(1).unwrap().to_string(),
                    deadline: task.get(2).unwrap().to_string(),
                    done: task.get(3).unwrap().parse::<bool>().unwrap(),
                    user_login: task.get(4).unwrap().to_string()}
                );
            }
        }

        let mut wrtr = Writer::from_path("tasks.csv").unwrap();
        wrtr.write_record(&["id","title","deadline","done","user_login"]).unwrap();
        for task in tasks {
            let to_write = [task.id.to_string(),
                task.title,
                task.deadline,
                task.done.to_string(),
                self.login.clone()];
            wrtr.write_record(&to_write).unwrap();
        }
        wrtr.flush().expect("Writing failed");
    }

    pub(crate) fn update_task(&self, task_to_update: Task){
        let task_id = task_to_update.id;
        let mut tasks = Vec::new();
        let tasks_file = File::open("tasks.csv").unwrap();
        let mut rdr = Reader::from_reader(tasks_file);
        for record in rdr.records(){
            let task = record.unwrap();
            if task.get(0).unwrap().parse::<i32>().unwrap() != task_id{
                tasks.push(Task{
                    id: task.get(0).unwrap().parse::<i32>().unwrap(),
                    title: task.get(1).unwrap().to_string(),
                    deadline: task.get(2).unwrap().to_string(),
                    done: task.get(3).unwrap().parse::<bool>().unwrap(),
                    user_login: task.get(4).unwrap().to_string()}
                );
            }
            else {
                tasks.push(task_to_update.clone());
            }
        }

        let mut wrtr = Writer::from_path("tasks.csv").unwrap();
        wrtr.write_record(&["id","title","deadline","done","user_login"]).unwrap();
        for task in tasks {
            let to_write = [task.id.to_string(),
                task.title,
                task.deadline,
                task.done.to_string(),
                task.user_login.clone()];
            wrtr.write_record(&to_write).unwrap();
        }
        wrtr.flush().expect("Writing failed");
    }

}


pub(crate) fn authenticate(login: String, password: String) -> Result<User, String> {
    let users_file = File::open("users.csv").unwrap();
    let mut rdr = Reader::from_reader(users_file);
    for record in rdr.records() {
        let user = record.unwrap();
        if user.get(0).unwrap().to_string() == login {
            if user.get(1).unwrap().to_string() == password {
                return Ok(User { login, password })
            } else {
                return Err(String::from("Неправильний пароль"));
            }
        }
    }
    return Err(String::from("Неправильний логін"));
}

pub(crate) fn registration(login: String, password: String) -> Result<User, String> {
    let users_file = File::open("users.csv").unwrap();
    let mut rdr = Reader::from_reader(users_file);
    for record in rdr.records() {
        let user = record.unwrap();
        if user.get(0).unwrap() == login {
                return Err(String::from("Такий користувач уже існує"));
        }
    }

    let file = OpenOptions::new().write(true).append(true).open("users.csv").unwrap();
    let mut wrtr = Writer::from_writer(file);
    let to_write = [login.clone(), password.clone()];
    wrtr.write_record(&to_write).unwrap();
    wrtr.flush().expect("Writing failed");
    return Ok(User{login, password});
}
