use amethyst::core::timing::Time;

use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{Camera, ScreenDimensions};

use amethyst::core::cgmath::{Deg, InnerSpace, Vector2, Vector3};

pub struct FlyCamSystem {
    mouse_cache: Vector2<f32>,
}

impl Default for FlyCamSystem {
    fn default() -> Self {
        FlyCamSystem {
            mouse_cache: Vector2::new(0.0, 0.0),
        }
    }
}

impl<'s> System<'s> for FlyCamSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, Time>,
        Fetch<'s, InputHandler<String, String>>,
        Fetch<'s, ScreenDimensions>,
    );

    fn run(&mut self, (cams, mut transforms, time, input, dims): Self::SystemData) {
        let x_in = input.axis_value("camera_x");
        let y_in = input.axis_value("camera_y");
        let z_in = input.axis_value("camera_z");
        let rot_move = input.mouse_position();

        let x_move: f32 = {
            match x_in {
                Some(movement) => movement as f32,
                None => 0.0,
            }
        };

        let y_move: f32 = {
            match y_in {
                Some(movement) => movement as f32,
                None => 0.0,
            }
        };

        let z_move: f32 = {
            match z_in {
                Some(movement) => movement as f32,
                None => 0.0,
            }
        };

        let move_dir = Vector3::new(x_move, y_move, z_move);

        for (_cam, transform) in (&cams, &mut transforms).join() {
            if move_dir.magnitude() != 0.0 {
                transform.move_local(move_dir, time.delta_seconds() * 1.0);
            }
        }

        if let Some((mouse_x, mouse_y)) = rot_move {
            let half_width = dims.width() / 2.0;
            let half_height = dims.height() / 2.0;
            let _offset_width = half_width - mouse_x as f32;
            let _offset_height = half_height - mouse_y as f32;

            let diff = self.mouse_cache - Vector2::new(mouse_x as f32, mouse_y as f32);
            self.mouse_cache = Vector2::new(mouse_x as f32, mouse_y as f32);

            for (_cam, transform) in (&cams, &mut transforms).join() {
                transform.rotate_local(Vector3::unit_x(), Deg(diff.y * 0.05));
                transform.rotate_global(Vector3::unit_y(), Deg(-diff.x * 0.05));
            }
        }
    }
}
