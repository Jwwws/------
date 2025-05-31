use iced::widget::tooltip::Position;
use iced::{application, highlighter, theme, Element, Font, Length, Task, Theme};
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip};
use std::ffi;
use std::path::{Path, PathBuf};
use std::io::ErrorKind;
use std::sync::Arc;
fn main() -> iced::Result{ 
    application(Editor::title,Editor::update,Editor::view)
    .theme(Editor::theme)
    .run_with(Editor::new)
}

struct Editor{
    content: text_editor::Content,
    error: Option<Error>,
    path: Option<PathBuf>,
    theme:highlighter::Theme,
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf,Arc<String>),Error>),
    Open,
    New,
    Save,
    FiledSaved(Result<PathBuf,Error>),
    ThemeSelected(highlighter::Theme),
}

impl  Editor{
  
    fn new()->(Self,Task<Message>){
        (
            Self{
            content: text_editor::Content::with_text(include_str!("./main.rs")),
            error: None,
            path: None,
            theme: highlighter::Theme::SolarizedDark,
        },
        Task::perform(load_file(default_load_file()),Message::FileOpened)
        )
    }

    fn title(&self)->String{
        String::from("Text Editor")
    }
    fn update(&mut self, message:Message)->Task<Message>{
        match message{
            Message::Edit(action)=>{
                self.content.perform(action);
                Task::none()
            }
            Message::FileOpened(Ok((path,contents)))=>{
                self.content = text_editor::Content::with_text(&contents);
                self.path = Some(path);
                Task::none()
            }
            Message::FileOpened(Err(error))=>{
                self.error = Some(error);
                Task::none()
            }
            Message::Open=>{
                Task::perform(pick_flie(),Message::FileOpened)
            }
            Message::New=>{
                self.content = text_editor::Content::new();
                self.path = None;
                Task::none()
            }
            Message::Save=>{
                let contents=self.content.text();
                Task::perform(save_file(self.path.clone(),contents), Message::FiledSaved)
            }
            Message::FiledSaved(Ok(path))=>{
                self.path = Some(path);
                Task::none()
            }
            Message::FiledSaved(Err(error))=>{
                self.error = Some(error);
                Task::none()
            }
            Message::ThemeSelected(theme)=>{
                self.theme=theme;
                Task::none()
            }
        }
            
        }

    

    fn view(&self) -> Element<'_, Message> {
        let controls =row![
            button("Open").on_press(Message::Open),
            button("New").on_press(Message::New),
            button("Save").on_press(Message::Save),
            horizontal_space(),
            pick_list(highlighter::Theme::ALL,Some(self.theme),Message::ThemeSelected),
        ].spacing(10);
        let input_content = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .highlight(self.path.as_deref()
            .and_then(Path::extension)
            .and_then(ffi::OsStr::to_str)
            .unwrap_or("rs"), self.theme);
            
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
    container(column![controls,input_content, status_bar]).padding(10).into()
    }

fn theme(&self) -> iced::Theme {
    if self.theme.is_dark(){
        Theme::Dark
    }else{
        Theme::Light
    }
    
        
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

async fn save_file(path:Option<PathBuf>,contents:String) -> Result<PathBuf,Error> {
    let path=if let Some(path)=path{
        path
    }else{
        rfd::AsyncFileDialog::new().set_title("Save file")
        .save_file().await
        .ok_or(Error::DialogClosed)
        .map(|filehandle|filehandle.path().to_owned())?
    };
    tokio::fs::write(&path,contents).await
        .map_err(|error|Error::IOFailed(error.kind()))?;
Ok(path)
}
