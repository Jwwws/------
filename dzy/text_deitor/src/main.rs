use iced::Theme;
use iced::{Command,executor,Application,Settings,Length};
use iced::widget::{container, horizontal_space, row, text, text_editor};
use iced::widget::column;
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content: text_editor::Content,
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action)
}

impl Application for Editor{
    type Executor = executor::Default;
    type Message=Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags)->(Self,Command<Message>){
        (
            Self{
            content: text_editor::Content::new(),
        },
        Command::none()
    )
    }

    fn title(&self)->String{
        String::from("Text Editor")
    }
    fn update(&mut self, message:Message)->Command<Message>{
        match message{
            Message::Edit(action)=>{
                self.content.perform(action);
                Command::none()
            }
        }

    }

        fn view(&self) -> iced::Element<'_, Message> {
        let input_content = text_editor(&self.content).on_action(Message::Edit).height(Length::Fill).style(self.theme()).into();
        let poisition = {
        let (line, column) = &self.content.cursor_position();
        text(format!("{}:{}", line + 1, column + 1))
    };
    let status_bar = row!(horizontal_space(), poisition);
    container(column!(input_content, status_bar)).padding(10).into()
}

   
        

    fn theme(&self)-> iced::Theme{
        Theme::Dark
    }
    
    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }
    
    fn scale_factor(&self) -> f64 {
        1.0
    }
    
    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

