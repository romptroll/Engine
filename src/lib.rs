/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */
pub use engine_core as core;
pub use engine_renderer as renderer;
pub use engine_gui as gui;
pub use engine_ecs as ecs;

pub mod game;
pub mod scene;

#[cfg(test)]
mod tests {
    use crate::{game::{Game}, scene::{Scene, SceneManager}};

    use engine_core::{info_log, warn_log};


    struct GameScene {}

    impl Scene<(), ()> for GameScene {
        fn on_start(&mut self, _d: &mut ()) {
            info_log!("hello");
        }

        fn on_update(&mut self, _d: &mut ())  {
            warn_log!("dd");
        }
    }

    struct TestGame {
        scenes: SceneManager<(), ()>,
    }

    impl Game for TestGame {
        fn on_start(&mut self) -> bool {
            self.scenes.add_scene(Box::new(GameScene {}), "game");
            self.scenes.set_current_scene("game");
            self.scenes.start(&mut ());
            true
        }

        fn on_update(&mut self, _dt: f32) -> bool {
            self.scenes.update(&mut ());
            false
        }

        fn on_render(&mut self, _fps: u32) -> bool {
            self.scenes.render(&mut ());
            true
        }
    }
    
    #[test]
    fn t() {
        TestGame::run(TestGame { scenes: SceneManager::new()});
        info_log!("d");
    }
}