#[derive(Debug)]
pub struct WorkEnvironment {pub grade: Link}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {pub role: String, pub name: String, pub next: Link}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {WorkEnvironment{grade: None}}

    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker{
            role: role,
            name: name,
            next: self.grade.take(),
        }))
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        let current_worker = self.grade.as_deref().unwrap();
        Some((String::from(&current_worker.name[..]), String::from(&current_worker.role[..])))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let worker_to_remove = self.grade.take();
        let mut name_to_remove = "<any>".to_string();
        if let Some(worker) = worker_to_remove {
            name_to_remove = String::from(&worker.name[..]);
            self.grade = worker.next;
        }
        Some(name_to_remove)
    }
}
