use iced::Theme;
use std::env;
use std::sync::Arc;
use std::path::{Path,PathBuf};
use std::io::ErrorKind;
use iced::{Command,executor,Application,Settings,Length};
use iced::widget::{column,container, horizontal_space, row, text, text_editor};
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content: text_editor::Content,
    error: Option<Error>,
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>,Error>),
}

impl Application for Editor{
    type Executor = executor::Default;
    type Message=Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags)->(Self,Command<Message>){
        (
            Self{
            content: text_editor::Content::with_text(include_str!("./main.rs")),
            error: None,
        },
        Command::perform(load_file(default_load_file()),Message::FileOpened)
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
            Message::FileOpened(Ok(contents))=>{
                self.content = text_editor::Content::with_text(&contents);
                Command::none()
            }
            Message::FileOpened(Err(error))=>{
                self.error = Some(error);
                Command::none()
            }
        }

    }

        fn view(&self) -> iced::Element<'_, Message> {
        let input_content = text_editor(&self.content).on_action(Message::Edit).height(Length::Fill).into();
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

#[derive(Debug,Clone)]
enum Error {
    IOFailed(ErrorKind),
    
}

async fn load_file(path:impl AsRef<Path>) -> Result<Arc<String>,Error> {
    tokio::fs::read_to_string(path).await.map(Arc::new).map_err(|error|Error::IOFailed(error.kind()))
}

fn default_load_file() -> PathBuf {
    PathBuf::from(format!("{}/rsc/main.rs",env!("CARGO_MANIFEST_DIR")))

}



