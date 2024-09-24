#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {student.0}
pub fn first_name(student: &Student) -> String {student.1.clone()}
pub fn last_name(student: &Student) -> String {student.2.clone()}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let identifier = id(&student); assert_eq!(identifier, 20);
        let firstname = first_name(&student); assert_eq!(firstname, "Pedro".to_string());
        let lastname = last_name(&student); assert_eq!(lastname, "Domingos".to_string());
    }
}
