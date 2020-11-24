use std::{
    fs::File,
    io::{BufReader, Error},
};

use druid::{
    im::Vector,
    text::{Attribute, EditAction, RichText},
    widget::TextBox,
    Data, Env, EventCtx, FontStyle, Lens, Selector,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const REBUILD: Selector<Uuid> = Selector::new("todo.rebuild");
pub const SELECT: Selector<Uuid> = Selector::new("todo.select");
pub const UNSELECT: Selector<Uuid> = Selector::new("todo.unselect");
pub const SAVE: Selector<Uuid> = Selector::new("todo.save");

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    #[data(same_fn = "PartialEq::eq")]
    pub id: Uuid,
    pub done: bool,
    pub editing: bool,
    pub selected: bool,
    text: String,
    // We use this to remember what the text was before an edit, in case it's cancelled
    stash: String,
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
            stash: self.text.clone(),
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
            stash: text.to_string(),
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
        if !data.selected {
            ctx.submit_command(SELECT.with(data.id));
            ctx.request_focus();
        }
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

    pub fn rebuild(&mut self) {
        self.rendered = Self::render(&self.text, self.done);
    }

    pub fn gain_selection(&mut self) {
        self.selected = true;
        self.editing = true;
        self.stash = self.text.clone();
    }

    pub fn cancel_edit(&mut self) {
        self.text = self.stash.clone();
    }

    pub fn lose_selection(&mut self) {
        self.editing = false;
        self.selected = false;
        self.rendered = Self::render(&self.text, self.done);
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub todos: Vector<TodoItem>,
    new_todo: String,
    #[data(same_fn = "PartialEq::eq")]
    pub selected: Option<Uuid>,
}

impl AppState {
    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_todo();
    }

    pub fn add_todo(&mut self) {
        self.todos.push_front(TodoItem::new(&self.new_todo));
        self.new_todo = String::new();
        self.save_to_json().unwrap();
    }

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

    pub fn save_to_json(&self) -> Result<(), Error> {
        let mut to_serialize = vec![];
        for TodoItem { text, done, id, .. } in self.todos.clone() {
            to_serialize.push(SerdeTodoItem { text, done, id });
        }
        let serialized = serde_json::to_string_pretty(&to_serialize)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn clear_completed(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        // TODO: can I get rid of this clone?
        let new_todos: Vector<TodoItem> = data
            .todos
            .iter()
            .cloned()
            .filter(|item| !item.done)
            .collect();

        data.todos = new_todos;

        data.save_to_json().unwrap();
    }
}
