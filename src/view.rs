use druid::{
    im::Vector,
    lens,
    theme::*,
    widget::Painter,
    widget::{Button, Checkbox, Either, Flex, List, RawLabel, TextBox},
    LensExt, RenderContext, Widget, WidgetExt,
};

use crate::data::*;
use crate::double_click::DoubleClick;

pub fn todo_item() -> impl Widget<TodoItem> {
    let painter = Painter::new(move |ctx, data: &TodoItem, env| {
        let selected = data.selected;
        let bounds = ctx.size().to_rect().inset(-2.).to_rounded_rect(3.);
        if ctx.is_active() && !selected {
            ctx.fill(bounds, &env.get(BACKGROUND_DARK));
        } else if ctx.is_hot() || selected {
            ctx.fill(bounds, &env.get(BACKGROUND_LIGHT));
        } else {
            ctx.fill(bounds, &env.get(WINDOW_BACKGROUND_COLOR))
        }

        if selected {
            ctx.stroke(bounds, &env.get(PRIMARY_DARK), 2.);
        }
    });

    let checkbox = Checkbox::new("").lens(TodoItem::done);

    let label = RawLabel::new()
        //TODO: find a better way to match textbox's padding
        .padding((-2., 0., 0., -2.))
        .lens(TodoItem::rendered);

    let save_button = Button::new("Save").on_click(TodoItem::save);
    let text_box = TextBox::new()
        .lens(TodoItem::text)
        .expand_width()
        .env_scope(|env, data| {
            env.set(TEXTBOX_INSETS, 0.);
            env.set(TEXTBOX_BORDER_WIDTH, 0.)
        });

    let edit_label = Flex::row()
        .with_flex_child(text_box, 1.)
        .with_child(save_button);

    let either = Either::new(
        |data, _env| data.selected && data.editing,
        edit_label,
        label,
    )
    .controller(TodoItemController)
    .expand_width()
    .controller(DoubleClick::new(TodoItem::double_click));

    Flex::row()
        .with_child(checkbox)
        .with_flex_child(either, 1.)
        .padding(5.)
        .background(painter)
        .on_click(TodoItem::select)
}

pub fn build_ui() -> impl Widget<AppState> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder("Add a new todo")
        .expand_width()
        .lens(AppState::new_todo);

    let add_todo_button = Button::new("Add").on_click(AppState::add_todo);

    let create = Flex::row()
        .with_flex_child(new_todo_textbox, 1.)
        .with_spacer(5.)
        .with_child(add_todo_button);

    let todo_list = List::new(todo_item).with_spacing(10.).lens(AppState::todos);
    // .lens(lens::Identity.map(
    //     |data: &AppState| (data.clone(), data.todos),
    //     |data: &mut AppState, (new_data, _): (AppState, Vector<TodoItem>)| {
    //         *data = new_data;
    //     },
    // ));

    Flex::column()
        .with_child(create)
        .with_spacer(10.)
        .with_child(todo_list)
        .with_spacer(10.)
        .with_child(Button::new("Save").on_click(AppState::click_save))
        .padding(10.)
}
