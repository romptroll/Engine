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

 /// Creates a single threaded game loop.
 /// Calls the applied callback to the applied state each frame.
 /// Gives the callback the current framerate and the curent deltatime for each call.
pub fn render_loop<State>(render_callback: &dyn Fn(&mut State, f32, u32) -> bool, state: &mut State) {
    let mut draw_count = 0;
    let mut previous_time = std::time::SystemTime::now();
    let mut previous_update_time = previous_time;
    let mut current_time = previous_time;
    let mut frame_rate = 0;

    while render_callback(state, current_time.duration_since(previous_update_time).unwrap().as_secs_f32(), frame_rate) == true {
        previous_update_time = current_time;
        current_time = std::time::SystemTime::now();

        /*let time_to_sleep = ((draw_count as f64 / (9999999.0 - 2.0) - (current_time - previous_time)) * 1000.0) as i64;
        if time_to_sleep > 0 {
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }*/

        draw_count += 1;

        if current_time.duration_since(previous_time).unwrap().as_secs_f32() >= 1.0
        {
            frame_rate = draw_count;

            draw_count = 0;
            previous_time = current_time;
        }
    }
}

#[allow(unused_variables)]
pub trait Game {
    //fn on_event(&mut self, event: EventHolder);
    fn on_start (&mut self)             -> bool { true }
    fn on_update(&mut self, dt: f32)    -> bool { true }
    fn on_render(&mut self, fps: u32)   -> bool { true }
    fn on_exit  (&mut self)             -> bool { true }

    fn run<State: Game>(game: State) {
        let mut running = true;

        let mut game = game;
        running &= game.on_start();

        let mut draw_count : u32 = 0;
        let mut previous_time = std::time::SystemTime::now();
        let mut previous_update_time = previous_time;
        let mut current_time = previous_time;
        let mut frame_rate = 0;
        let mut delta_time;

        while running {
            delta_time = current_time.duration_since(previous_update_time).unwrap().as_secs_f32();

            running &= game.on_update(delta_time);
            running &= game.on_render(frame_rate);

            //if self.poll_events() == false { running = false; }

            previous_update_time = current_time;
            current_time = std::time::SystemTime::now();

            draw_count += 1;

            if current_time.duration_since(previous_time).unwrap().as_secs_f32() >= 1.0 {
                frame_rate = draw_count;
                draw_count = 0;
                previous_time = current_time;
            }
        }

        game.on_exit();
    }
}