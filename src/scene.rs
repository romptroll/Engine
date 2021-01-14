/*
 *   Copyright (c) 2021 Ludwig Bogsveen
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

use std::collections::HashMap;

use engine_core::error_log;

use crate::game::GameData;

/*
 *   Copyright (c) 2021 Ludwig Bogsveen
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

#[allow(unused_variables)]
pub trait Scene {
    fn on_start     (&mut self, gd: &mut GameData) {}
    fn on_enter     (&mut self, gd: &mut GameData) {}
    fn on_exit      (&mut self, gd: &mut GameData) {}
    fn on_update    (&mut self, gd: &mut GameData) {}
    fn on_render    (&mut self, gd: &mut GameData) {}
    fn on_event     (&mut self, gd: &mut GameData) {}
}

pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<String>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }

    pub fn start(&mut self, gd: &mut GameData) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_start(gd),
            None => {}
        }
    }

    pub fn update(&mut self, gd: &mut GameData) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_update(gd),
            None => {}
        }
    }

    pub fn render(&mut self, gd: &mut GameData) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_render(gd),
            None => {}
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>, name: &str) {
        self.scenes.insert(name.to_string(), scene);
    }

    pub fn set_current_scene(&mut self, name: &str) {
        if name == "" {
            self.current_scene = None;
        } else if !self.scenes.contains_key(name)  {
            error_log!("Tried to set current scene to: \"{}\" but scene didn't exist!", name);
        } else {
            self.current_scene = Some(name.to_string());
        }
    }

    pub fn current_scene_name(&self) -> &str {
        match &self.current_scene {
            Some(name) => &name,
            None => ""
        }
    }

    pub fn current_scene(&self) -> Option<&Box<dyn Scene>> {
        match &self.current_scene {
            Some(name) => Some(&self.scenes[name]),
            None => None,
        }
    }

    pub fn current_scene_mut(&mut self) -> Option<&mut Box<dyn Scene>> {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name),
            None => None,
        }
    }
}