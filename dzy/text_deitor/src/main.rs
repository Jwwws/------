
use std::sync::Arc;
use std::path::{Path,PathBuf};
use std::io::ErrorKind;
use iced::{Theme,Command,executor,Application,Settings,Length};
use iced::widget::{button,column,container, horizontal_space, row, text, text_editor};
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content: text_editor::Content,
    error: Option<Error>,
    path: Option<PathBuf>,
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf,Arc<String>),Error>),
    Open,
    New,
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
            path: None,
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
            Message::FileOpened(Ok((path,contents)))=>{
                self.content = text_editor::Content::with_text(&contents);
                self.path = Some(path);
                Command::none()
            }
            Message::FileOpened(Err(error))=>{
                self.error = Some(error);
                Command::none()
            }
            Message::Open=>{
                Command::perform(pick_flie(),Message::FileOpened)
            }
            Message::New=>{
                self.content = text_editor::Content::new();
                self.path = None;
                Command::none()
            }
            
        }

    }

    fn view(&self) -> iced::Element<'_, Message> {
        let controls =row!{
            button("Open").on_press(Message::Open),
            button("New").on_press(Message::New),
        }.spacing(10);
        let input_content = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill);
            
        let poisition = {
        let (line, column) = &self.content.cursor_position();
        text(format!("{}:{}", line + 1, column + 1))
        };
        let file_path=if let Some(Error::IOFailed(error))=self.error.as_ref(){
            text(error.to_string())
        }else{
            match self.path.as_deref().and_then(Path::to_str){
                Some(path)=>text(path).size(15),
                None=>text("New File").size(15),

            }
        };
    let status_bar = row!(file_path,horizontal_space(), poisition);
    container(column!(controls,input_content, status_bar)).padding(10).into()
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
    
}

#[derive(Debug,Clone)]
enum Error {
    IOFailed(ErrorKind),
    DialogClosed,
}

async fn load_file(path:impl AsRef<Path>) -> Result<(PathBuf,Arc<String>),Error> {
    let contents =tokio::fs::read_to_string(path.as_ref()).await.map(Arc::new).map_err(|error|Error::IOFailed(error.kind()))?;
    Ok((path.as_ref().to_path_buf(),contents))
}

fn default_load_file() -> PathBuf {
    PathBuf::from(format!("{}/rsc/main.rs",env!("CARGO_MANIFEST_DIR")))

}
async fn pick_flie()->Result<(PathBuf,Arc<String>),Error> {
   let file_path= rfd::AsyncFileDialog::new().set_title("Choose a file")
                .pick_file()
                .await
                .ok_or(Error::DialogClosed)
                .map(|filehandle|filehandle.path().to_owned())?;
    load_file(file_path).await
}


