use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target};

use crate::data::*;

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(id) = cmd.get(SELECT) {
            data.selected = Some(id.clone());
            for todo in data.todos.iter_mut() {
                if id == &todo.id {
                    todo.gain_selection();
                } else {
                    todo.lose_selection();
                }
            }
            Handled::Yes
        } else if let Some(id) = cmd.get(UNSELECT) {
            dbg!("unselecting...");
            data.selected = None;
            for todo in data.todos.iter_mut() {
                if id == &todo.id {
                    todo.lose_selection();
                }
            }
            Handled::Yes
        } else if let Some(id) = cmd.get(REBUILD) {
            for todo in data.todos.iter_mut() {
                if id == &todo.id {
                    todo.rebuild();
                }
            }
            Handled::Yes
        } else if let Some(_id) = cmd.get(SAVE) {
            // TODO: use the id to do a more minimal diff
            data.save_to_json().unwrap();
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
