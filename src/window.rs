use softbuffer::GraphicsContext;
use winit::{
    event::{Event, WindowEvent, DeviceEvent, MouseScrollDelta, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::PhysicalPosition,
    
};

use crate::interface::{Frame, self, Viewport};

pub fn window_main() {
    let mut frame: Frame = Frame::default();
    let mut last_known_cursor_position: Option<PhysicalPosition<f64>> = None;
    let mut ctrl_pressed: bool = false;
    let event_loop = EventLoop::new();
    let window = match WindowBuilder::new().build(&event_loop) 
    {
        Result::Ok(window) => window,
        Result::Err(err) => panic!("winit failed to create window! Returned:\n{}", err)
    };
    let mut graphics_context = match unsafe { GraphicsContext::new(&window, &window) } 
    {
        Result::Ok(graphics_context) => graphics_context,
        Result::Err(err) => panic!("softbuffer failed to initialize graphics context! Returned:\n{}", err)
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                frame.change_window_size(width, height);
                update_frame(&mut frame, &mut graphics_context);
            },
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::CursorLeft { device_id: _ } => {
                        last_known_cursor_position = None;
                    },
                    WindowEvent::CursorMoved { device_id: _, position, ..} => {
                        last_known_cursor_position = Some(position);
                    },
                    WindowEvent::ModifiersChanged(modifiers) => {
                        ctrl_pressed = modifiers.ctrl();
                    },
                    WindowEvent::MouseWheel { device_id: _, delta: MouseScrollDelta::LineDelta(scroll, _), phase: _, modifiers: _ } => {
                        if last_known_cursor_position.is_some() {
                            let zoom_factor = 1f64 - (scroll as f64 * 0.1f64);
                            frame.viewport.zoom(frame.point_from_pixel(last_known_cursor_position.unwrap().x, last_known_cursor_position.unwrap().y), zoom_factor);
                        };
                        update_frame(&mut frame, &mut graphics_context);
                    },
                    WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                        if input.virtual_keycode.is_some() && input.state == ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::Equals => {
                                    if last_known_cursor_position.is_some() {
                                        frame.viewport.zoom(frame.point_from_pixel(last_known_cursor_position.unwrap().x, last_known_cursor_position.unwrap().y), 0.9f64);
                                        update_frame(&mut frame, &mut graphics_context);
                                    }
                                },
                                VirtualKeyCode::Minus => {
                                    if last_known_cursor_position.is_some() {
                                        frame.viewport.zoom(frame.point_from_pixel(last_known_cursor_position.unwrap().x, last_known_cursor_position.unwrap().y), 1.1f64);
                                        update_frame(&mut frame, &mut graphics_context);
                                    }
                                },
                                VirtualKeyCode::Home => {
                                    frame.viewport = Viewport::default();
                                    update_frame(&mut frame, &mut graphics_context);
                                },

                                _ => {}
                            }
                        }
                    },
                    _ => {},
                }
                
            },
            _ => {},
        }
    });
}

fn update_frame(frame: &mut Frame, graphics_context: &mut GraphicsContext) {
    let width = frame.buffer.width();
    let height = frame.buffer.height();
    frame.render();
    let buffer = (0..((width * height) as usize))
        .map(|index| {
            let y = index / (width as usize);
            let x = index % (width as usize);
            interface::hdr_to_display_colour(frame.buffer.get_pixel(x as u32, y as u32))
        })
        .collect::<Vec<_>>();

    graphics_context.set_buffer(&buffer, width as u16, height as u16);
}