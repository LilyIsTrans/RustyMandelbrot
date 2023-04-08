use softbuffer::GraphicsContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn window_main() {
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
                let buffer = (0..((width * height) as usize))
                    .map(|index| {
                        let y = index / (width as usize);
                        let x = index % (width as usize);
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x + y) % 255;

                        let color = blue | (green << 8) | (red << 16);

                        color as u32
                    })
                    .collect::<Vec<_>>();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}