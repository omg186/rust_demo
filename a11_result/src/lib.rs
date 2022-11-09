#[derive(Debug)]
pub enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}
pub fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main_menu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}
pub fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    println!("choice:{:?}", choice);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_work() {
        let result = pick_choice("start");
        assert!(result.is_ok())
    }
    #[test]
    fn it_result_work() -> Result<(), String> {
        get_choice("start")?;
        Ok(())
    }
    #[test]
    fn it_err_work() {
        let result = pick_choice("end");
        assert!(result.is_err())
    }
    #[test]
    fn get_choice_ok_work() {
        let choice = get_choice("main_menu");
        assert!(choice.is_ok());
        println!("{:?}", choice.unwrap());
    }
    #[test]
    fn get_choice_err_work() {
        let choice = get_choice("other");
        assert!(choice.is_err());
    }
}
