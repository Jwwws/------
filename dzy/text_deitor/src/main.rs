use iced::{Sandbox,Settings};
use iced::widget::{text};
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{

}
#[derive(Debug)]
enum Message{

}

impl Sandbox for Editor{
    type Message=Message;

    fn new()->Self{
        Self{

        }
    }

    fn title(&self)->String{
        String::from("Text Editor")
    }
    fn update(&mut self, message:Message){
        match message{
        }
    }

    fn view(&self)->iced::Element<'_,Message>{
        text("This is the text editor").into()
    }

}