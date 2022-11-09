#[derive(Debug)]
pub enum Color {
    Red,
    Blue,
}
pub struct Person {
    age: i32,
    name: String,
    fav_color: Color,
}
impl Person {
    pub fn print_person(&self) {
        println!(
            "name:{:?},age:{:?},color:{:?}",
            self.name, self.age, self.fav_color
        )
    }
}
pub fn filter_person_age(persons: &Vec<Person>) -> Vec<&Person> {
    // let persons = persons
    //     .iter()
    //     .filter(|p| p.age >= 10)
    //     .map(|f| f)
    //     .collect::<Vec<Person>>();
    // persons
    let mut result = vec![];
    for person in persons {
        if person.age <= 10 {
            result.push(person);
            person.print_person();
        }
    }
    result
}
#[cfg(test)]

mod tests {
    use crate::*;

    #[test]
    fn it_work() {
        let persons = vec![
            Person {
                age: 10,
                name: "a1".to_string(),
                fav_color: Color::Red,
            },
            Person {
                age: 9,
                name: "a2".to_string(),
                fav_color: Color::Blue,
            },
            Person {
                age: 20,
                name: "a3".to_string(),
                fav_color: Color::Blue,
            },
        ];
        filter_person_age(&persons);
        // assert_eq!(vec![persons[0],persons[1]], filter_person_age(&persons))
    }
}
