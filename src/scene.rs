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

use std::collections::{HashMap, VecDeque};

use engine_core::error_log;

#[allow(unused_variables)]
pub trait Scene<Data, Event> {
    fn on_start     (&mut self, data: &mut Data) {}
    fn on_enter     (&mut self, data: &mut Data) {}
    fn on_exit      (&mut self, data: &mut Data) {}
    fn on_update    (&mut self, data: &mut Data) {}
    fn on_render    (&mut self, data: &mut Data) {}
    fn on_event     (&mut self, data: &mut Data, event: Event) {}
    fn poll_events  (&mut self, data: &mut Data, events: &mut VecDeque<Event>) {}
}

pub struct SceneManager<Data, Event> {
    scenes: HashMap<String, Box<dyn Scene<Data, Event>>>,
    current_scene: Option<String>,
    events: VecDeque<Event>,
}

impl<Data, Event> SceneManager<Data, Event> {
    pub fn new() -> SceneManager<Data, Event> {
        SceneManager {
            scenes: HashMap::new(),
            current_scene: None,
            events: VecDeque::new(),
        }
    }

    pub fn start(&mut self, data: &mut Data) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_start(data),
            None => {}
        }
    }

    pub fn update(&mut self, data: &mut Data) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_update(data),
            None => {}
        }
    }

    pub fn render(&mut self, data: &mut Data) {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_render(data),
            None => {}
        }
    }

    pub fn send_event(&mut self, data: &mut Data, event: Event) {
        match &mut self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_event(data, event),
            None => {}
        }
    }

    pub fn poll_events(&mut self, data: &mut Data) -> VecDeque<Event>{
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().poll_events(data, &mut self.events),
            None => {}
        }

        self.events.drain(..).collect()
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene<Data, Event>>, name: &str) {
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

    pub fn current_scene(&self) -> Option<&Box<dyn Scene<Data, Event>>> {
        match &self.current_scene {
            Some(name) => Some(&self.scenes[name]),
            None => None,
        }
    }

    pub fn current_scene_mut(&mut self) -> Option<&mut Box<dyn Scene<Data, Event>>> {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name),
            None => None,
        }
    }

    pub fn scene(&self, name: &str) -> Option<&Box<dyn Scene<Data, Event>>> {
        self.scenes.get(name)
    }

    pub fn scene_mut(&mut self, name: &str) -> Option<&mut Box<dyn Scene<Data, Event>>> {
        self.scenes.get_mut(name)
    }
}