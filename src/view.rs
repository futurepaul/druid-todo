use druid::{
    theme::*,
    widget::Painter,
    widget::Scroll,
    widget::{Button, Checkbox, Either, Flex, List, RawLabel, TextBox},
    Color, Insets, RenderContext, Widget, WidgetExt,
};

use crate::{
    controllers::{AddTodoController, TodoItemController},
    data::{AppState, TodoItem},
    double_click::DoubleClick,
};

pub fn todo_item() -> impl Widget<TodoItem> {
    let painter = Painter::new(move |ctx, data: &TodoItem, env| {
        let selected = data.selected;
        let bounds = ctx.size().to_rect().inset(-2.).to_rounded_rect(3.);
        if ctx.is_hot() && !ctx.is_active() {
            ctx.fill(bounds, &env.get(WINDOW_BACKGROUND_COLOR))
        } else {
            ctx.fill(bounds, &Color::BLACK)
        }

        if selected {
            ctx.fill(bounds, &Color::BLACK);
            ctx.stroke(bounds, &env.get(PRIMARY_DARK), 2.);
        }
    });

    let checkbox = Checkbox::new("").lens(TodoItem::done);

    let label = RawLabel::new()
        //TODO: find a better way to match textbox's padding
        .padding(Insets::new(-2., 0., -2., 0.))
        .lens(TodoItem::rendered);

    let text_box = TextBox::new()
        .lens(TodoItem::text)
        .expand_width()
        .env_scope(|env, _data| {
            env.set(TEXTBOX_INSETS, 0.);
            env.set(TEXTBOX_BORDER_WIDTH, 0.);
            env.set(BACKGROUND_LIGHT, Color::BLACK);
            let primary = env.get(PRIMARY_DARK);
            env.set(SELECTION_COLOR, primary);
        });

    let edit_label = Flex::row().with_flex_child(text_box, 1.);

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
        .with_spacer(5.)
        .with_flex_child(either, 1.)
        .padding(10.)
        .background(painter)
        .on_click(TodoItem::select)
}

pub fn build_ui() -> impl Widget<AppState> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder("Add a new todo")
        .expand_width()
        .lens(AppState::new_todo)
        .env_scope(|env, _data| {
            env.set(TEXTBOX_INSETS, 5.);
            env.set(TEXTBOX_BORDER_WIDTH, 2.);
            env.set(BACKGROUND_LIGHT, Color::WHITE);
            env.set(LABEL_COLOR, Color::BLACK);
            env.set(CURSOR_COLOR, Color::BLACK);
            env.set(PRIMARY_LIGHT, Color::BLACK);
            env.set(BORDER_DARK, Color::WHITE);
        })
        .controller(AddTodoController);

    let add_todo_button = Button::new("Add").on_click(AppState::click_add);

    let create = Flex::row()
        .with_flex_child(new_todo_textbox, 1.)
        .with_spacer(5.)
        .with_child(add_todo_button)
        .padding(10.)
        .background(PRIMARY_DARK);

    let todo_list = List::new(todo_item)
        .with_spacing(5.)
        .padding(10.)
        .lens(AppState::todos);

    let clear_completed_button = Button::new("Clear completed").on_click(AppState::clear_completed);

    let actions_row = Flex::row()
        .with_child(clear_completed_button)
        .with_flex_spacer(1.)
        .padding(10.);

    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(create)
                .with_flex_child(Scroll::new(todo_list).vertical(), 1.),
            1.,
        )
        .with_child(actions_row)
        .background(Color::BLACK)
}
