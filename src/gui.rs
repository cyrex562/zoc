use std::collections::{HashMap};
use cgmath::{Vector3, Matrix4, ortho};
use context::{Context};
use texture::{load_texture_raw};
use types::{Size2, ScreenPos};
use text;
use mesh::{Mesh};
use pipeline::{Vertex};

/// Check if this was a tap or swipe
pub fn is_tap(context: &Context) -> bool {
    let mouse = context.mouse();
    let diff = mouse.pos.v - mouse.last_press_pos.v;
    let tolerance = 20; // TODO: read from config file
    diff.x.abs() < tolerance && diff.y.abs() < tolerance
}

pub fn basic_text_size(context: &Context) -> f32 {
    // TODO: use different value for android
    let lines_per_screen_h = 14.0;
    (context.win_size().h as f32) / lines_per_screen_h
}

pub fn small_text_size(context: &Context) -> f32 {
    basic_text_size(context) / 2.0
}

pub fn get_2d_screen_matrix(win_size: Size2) -> Matrix4<f32> {
    let left = 0.0;
    let right = win_size.w as f32;
    let bottom = 0.0;
    let top = win_size.h as f32;
    let near = -1.0;
    let far = 1.0;
    ortho(left, right, bottom, top, near, far)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ButtonId {pub id: i32}

#[derive(Clone, Debug)]
pub struct Button {
    pos: ScreenPos,
    size: Size2,
    mesh: Mesh,
}

impl Button {
    pub fn new(context: &mut Context, label: &str, pos: ScreenPos) -> Button {
        let text_size = basic_text_size(context);
        Button::new_with_size(context, label, text_size, pos)
    }

    pub fn new_small(context: &mut Context, label: &str, pos: ScreenPos) -> Button {
        let text_size = small_text_size(context);
        Button::new_with_size(context, label, text_size, pos)
    }

    pub fn new_with_size(context: &mut Context, label: &str, size: f32, pos: ScreenPos) -> Button {
        let (texture_size, texture_data) = text::text_to_texture(context.font(), size, label);
        let texture = load_texture_raw(context.factory_mut(), texture_size, &texture_data);
        let h = texture_size.h as f32;
        let w = texture_size.w as f32;
        let vertices = &[
            Vertex{pos: [0.0, 0.0, 0.0], uv: [0.0, 1.0]},
            Vertex{pos: [0.0, h, 0.0], uv: [0.0, 0.0]},
            Vertex{pos: [w, 0.0, 0.0], uv: [1.0, 1.0]},
            Vertex{pos: [w, h, 0.0], uv: [1.0, 0.0]},
        ];
        let indices = &[0,  1,  2,  1,  2,  3];
        let mesh = Mesh::new(context, vertices, indices, texture);
        Button {
            pos: pos,
            size: texture_size,
            mesh: mesh,
        }
    }

    pub fn draw(&self, context: &mut Context) {
        context.draw_mesh(&self.mesh);
    }

    pub fn pos(&self) -> ScreenPos {
        self.pos
    }

    pub fn set_pos(&mut self, pos: ScreenPos) {
        self.pos = pos;
    }

    pub fn size(&self) -> Size2 {
        self.size
    }
}

#[derive(Clone, Debug)]
pub struct ButtonManager {
    buttons: HashMap<ButtonId, Button>,
    last_id: ButtonId,
}

impl ButtonManager {
    pub fn new() -> ButtonManager {
        ButtonManager {
            buttons: HashMap::new(),
            last_id: ButtonId{id: 0},
        }
    }

    pub fn buttons(&self) -> &HashMap<ButtonId, Button> {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut HashMap<ButtonId, Button> {
        &mut self.buttons
    }

    pub fn add_button(&mut self, button: Button) -> ButtonId {
        let id = self.last_id;
        self.buttons.insert(id, button);
        self.last_id.id += 1;
        id
    }

    pub fn remove_button(&mut self, id: ButtonId) {
        self.buttons.remove(&id).unwrap();
    }

    pub fn get_clicked_button_id(&self, context: &Context) -> Option<ButtonId> {
        let x = context.mouse().pos.v.x;
        let y = context.win_size().h - context.mouse().pos.v.y;
        for (&id, button) in self.buttons() {
            if x >= button.pos().v.x
                && x <= button.pos().v.x + button.size().w
                && y >= button.pos().v.y
                && y <= button.pos().v.y + button.size().h
            {
                return Some(id);
            }
        }
        None
    }

    pub fn draw(&self, context: &mut Context) {
        let proj_mat = get_2d_screen_matrix(context.win_size());
        for button in self.buttons().values() {
            let tr_mat = Matrix4::from_translation(Vector3 {
                x: button.pos().v.x as f32,
                y: button.pos().v.y as f32,
                z: 0.0,
            });
            context.set_mvp(proj_mat * tr_mat);
            button.draw(context);
        }
    }
}
