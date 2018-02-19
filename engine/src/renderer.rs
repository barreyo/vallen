
use gfx_hal::{Backend, Device};
use gfx_hal::command::{Rect, Viewport};
use gfx_hal::device::Extent;
use gfx_hal::format::Format;
use gfx_hal::pool::CommandPool;
use gfx_hal::queue::{CommandQueue, Graphics, Supports, Transfer, Submission};
use gfx_hal::window::{Backbuffer, Swapchain};

use petgraph::prelude::*;
use petgraph;

use winit::{EventsLoop, Window};

type FrameGraph = Graph;

pub struct Renderer<B: Backend, 'a> {
    window: Window,
    event_loop: &'a EventsLoop,
    frame_graph: FrameGraph,
    time: FrameTime,
    swapchain: B::Swapchain,
    format: Format,
    backbuffer: Backbuffer<B>,
    window_w: u16,
    window_h: u16,
}


impl<B: Backend> Renderer<B: Backend> {

}


