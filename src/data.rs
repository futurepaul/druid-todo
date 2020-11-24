use std::{
    fs::File,
    io::{BufReader, Error},
};

use druid::{
    im::Vector,
    text::{Attribute, EditAction, RichText},
    widget::{Controller, Either, TextBox},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Event, EventCtx, FontStyle, Handled, Lens,
    Selector, Target, UpdateCtx, Widget,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const REBUILD: Selector = Selector::new("todo.rebuild");
const SELECT: Selector<Uuid> = Selector::new("todo.select");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(id) = cmd.get(SELECT) {
            dbg!("selecting...");
            data.selected = Some(id.clone());
            for mut todo in data.todos.iter_mut() {
                if id == &todo.id {
                    dbg!(id);
                    todo.selected = true;
                    todo.editing = true;
                } else {
                    todo.selected = false;
                    todo.editing = false;
                }
            }
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    #[data(same_fn = "PartialEq::eq")]
    id: Uuid,
    done: bool,
    pub editing: bool,
    pub selected: bool,
    text: String,
    rendered: RichText,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerdeTodoItem {
    id: Uuid,
    done: bool,
    text: String,
}

impl Into<TodoItem> for SerdeTodoItem {
    fn into(self) -> TodoItem {
        TodoItem {
            id: self.id,
            done: self.done,
            text: self.text.clone(),
            editing: false,
            selected: false,
            rendered: TodoItem::render(&self.text, self.done),
        }
    }
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            done: false,
            editing: false,
            selected: false,
            text: text.to_string(),
            rendered: TodoItem::render(text, false),
        }
    }

    pub fn double_click(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.editing = true;
        ctx.request_layout();
        ctx.submit_command(
            TextBox::PERFORM_EDIT
                .with(EditAction::SelectAll)
                .to(ctx.widget_id()),
        );
    }

    pub fn select(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        dbg!("clicked select");
        if !data.selected {
            ctx.submit_command(SELECT.with(data.id));
            ctx.request_focus();
        }
        // data.selected = true;
    }

    pub fn save(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.editing = false;
        ctx.submit_command(REBUILD);
    }

    fn render(text: &str, done: bool) -> RichText {
        if done {
            RichText::new(format!("~{}~", text).into())
                .with_attribute(0.., Attribute::style(FontStyle::Italic))
                .with_attribute(0.., Attribute::text_color(druid::theme::PLACEHOLDER_COLOR))
        } else {
            RichText::new(text.into())
        }
    }
}

pub struct TodoItemController;

impl Controller<TodoItem, Either<TodoItem>> for TodoItemController {
    fn event(
        &mut self,
        child: &mut Either<TodoItem>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TodoItem,
        env: &Env,
    ) {
        match event {
            Event::Command(command) => {
                if command.is(REBUILD) {
                    dbg!("got a rebuild");
                    data.rendered = TodoItem::render(&data.text, data.done);
                    ctx.request_layout();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
    fn update(
        &mut self,
        child: &mut Either<TodoItem>,
        ctx: &mut UpdateCtx,
        old_data: &TodoItem,
        data: &TodoItem,
        env: &Env,
    ) {
        if old_data.done != data.done {
            dbg!("something is happening?");
            ctx.submit_command(REBUILD);
            ctx.request_layout();
        }
        child.update(ctx, old_data, data, env);
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    todos: Vector<TodoItem>,
    new_todo: String,
    #[data(same_fn = "PartialEq::eq")]
    selected: Option<Uuid>,
}

impl AppState {
    pub fn add_todo(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.todos.push_back(TodoItem::new(&data.new_todo))
    }

    // pub fn is_selected(&self, todo: &TodoItem) -> bool {
    //     if let Some(selected) = self.selected {
    //         selected == todo.id
    //     } else {
    //         false
    //     }
    // }

    pub fn load_from_json() -> Self {
        let file = File::open("todos.json");

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let serde_todos: Vec<SerdeTodoItem> =
                    serde_json::from_reader(reader).unwrap_or(vec![]);
                let todos: Vec<TodoItem> =
                    serde_todos.iter().map(|item| item.clone().into()).collect();
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                    selected: None,
                }
            }
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
                selected: None,
            },
        }
    }

    fn save_to_json(&self) -> Result<(), Error> {
        let mut to_serialize = vec![];
        for TodoItem { text, done, id, .. } in self.todos.clone() {
            to_serialize.push(SerdeTodoItem { text, done, id });
        }
        let serialized = serde_json::to_string_pretty(&to_serialize)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn click_save(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.save_to_json().unwrap();
    }
}
