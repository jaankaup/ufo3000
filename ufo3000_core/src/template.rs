use std::future::Future;

#[cfg(target_arch = "wasm32")]
use {
    web_sys::{ImageBitmapRenderingContext, OffscreenCanvas},
    winit::platform::web::WindowExtWebSys,
    wasm_bindgen::JsCast,
}

#[cfg(not(target_arch = "wasm32"))]
use simple_logger::SimpleLogger;

use log::LevelFilter;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub use winit::event::VirtualKeyCode as Key;

use crate::input::InputCache;

/// A trait for wgpu-rs based application.
pub trait Application: Sized + 'static {

    /// Creates the Application.
    fn init(configuration: &WGPUConfiguration) -> Self;

    /// The render function for application.
    fn render(&mut self,
              device: &wgpu::Device,
              queue: &mut wgpu::Queue,
              surface: &wgpu::Surface,
              sc_desc: &wgpu::SurfaceConfiguration,
              #[cfg(target_arch = "wasm32")]
              offscreen_canvas_setup: &OffscreenCanvasSetup,
              spawner: &Spawner);

    /// A function that handles inputs.
    fn input(&mut self, queue: &wgpu::Queue, input_cache: &InputCache);

    /// A function for resizing.
    fn resize(&mut self, device: &wgpu::Device, sc_desc: &wgpu::SurfaceConfiguration, new_size: winit::dpi::PhysicalSize<u32>);

    /// A function for updating the state of the application.
    fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, input: &InputCache, spawner: &Spawner);

    /// A function for program exit event.
    fn exit(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, input: &InputCache, spawner: &Spawner);
}

#[cfg(target_arch = "wasm32")]
pub struct OffscreenCanvasSetup {
    pub offscreen_canvas: OffscreenCanvas,
    pub bitmap_renderer: ImageBitmapRenderingContext,
}

/// A trait for Loops.
pub trait Loop: Sized + 'static {

    /// Initialize loop.
    fn init() -> Self;

    /// Run function that starts the loop. Beware: run takes ownership of application and
    /// configuration.
    fn run<A: Application>(&self, application: A, configuration: WGPUConfiguration);
}

/// A struct that holds the wgpu-rs application resources.
pub struct WGPUConfiguration {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
    pub instance: wgpu::Instance,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SurfaceConfiguration,
    #[cfg(target_arch = "wasm32")]
    pub offscreen_canvas_setup: OffscreenCanvasSetup,
}

/// A trait to configure wgpu-rs engine.
pub trait WGPUFeatures: Sized + 'static {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::default()
    }
    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities {
            flags: wgpu::DownlevelFlags::empty(),
            shader_model: wgpu::ShaderModel::Sm5,
            ..wgpu::DownlevelCapabilities::default()
        }
    }
}

/// A "basic" loop.
pub struct BasicLoop { }

impl Loop for BasicLoop {

    fn init() -> Self {
        BasicLoop {}
    }

    fn run<A: Application>(&self, mut application: A, WGPUConfiguration {
        window,
        event_loop,
        instance,
        mut size,
        surface,
        adapter,
        device,
        mut queue,
        mut sc_desc,
        #[cfg(target_arch = "wasm32")]
        offscreen_canvas_setup
        }: WGPUConfiguration,) {

    let spawner = Spawner::new();

    let mut input = InputCache::init();

    // Launch the loop.
    event_loop.run(move |event, _, control_flow| {

        // Force the ownerships to this closure.
        let _ = (&window,
                &instance,
                &mut size,
                &surface,
                &adapter,
                &device,
                &mut queue,
                &mut sc_desc,
                &mut application,
                &mut input,
                &spawner,
                #[cfg(target_arch = "wasm32")]
                &offscreen_canvas_setup
                );

        *control_flow = ControlFlow::Poll;
        //*control_flow = ControlFlow::Wait;

        match event {

            // Event::NewEvents(start_cause) => {
            //     match start_cause {
            //         Init => {}
            //         _ => {}
            //     }
            // }

            Event::LoopDestroyed => {
                application.exit(&device, &queue, &input, &spawner);
            }

            // TODO: check if pre_update and update are conficting in some circumstances.
            Event::MainEventsCleared => {
                application.input(&queue, &input);
                application.update(&device, &queue, &input, &spawner);
                input.pre_update();
                window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    spawner.run_until_stalled();
                }

                let close_application = input.key_state(&Key::Q);
                if !close_application.is_none() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent { event, ..} => {
                // Update input cache.
                input.update(&event);

                match event { // Add ScaleFactorChanged.
                    WindowEvent::Resized(new_size) => {
                        size = new_size;
                        sc_desc.width = new_size.width.max(1);
                        sc_desc.height = new_size.height.max(1);
                        surface.configure(&device, &sc_desc);
                        application.resize(&device, &sc_desc, size);
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                #[cfg(not(target_arch = "wasm32"))]
                application.render(&device, &mut queue, &surface, &sc_desc, &spawner);

                #[cfg(target_arch = "wasm32")]
                application.render(&device, &mut queue, &surface, &sc_desc, &offscreen_canvas_setup, &spawner);
            }
            _ => { } // Any other events
        } // match event
    }); // run
    }
}


/// Initializes wgpu-rs system. TODO: finish the Result<...>.
pub async fn setup<P: WGPUFeatures>(title: &str) -> Result<WGPUConfiguration, &'static str> {

    let title = title.to_owned();
    // env_logger::init();

    #[cfg(not(target_arch = "wasm32"))]
    {
        SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("ufo3000_core", LevelFilter::Info)
        .with_module_level("input", LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();
    }

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title);
    #[cfg(windows_OFF)] // TODO
    {
        use winit::platform::windows::WindowBuilderExtWindows;
        builder = builder.with_no_redirection_bitmap(true);
        log::info!("windows_OFF :: True");
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        console_log::init_with_level(log::Level::Trace).expect("could not initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    #[cfg(target_arch = "wasm32")]
    let offscreen_canvas = OffscreenCanvas::new(1024, 768).expect("couldn't create OffscreenCanvas");

    #[cfg(target_arch = "wasm32")]
    let bitmap_renderer = window
        .canvas()
        .get_context("bitmaprenderer")
        .expect("couldn't create ImageBitmapRenderingContext (Result)")
        .expect("couldn't create ImageBitmapRenderingContext (Option)")
        .dyn_into::<ImageBitmapRenderingContext>()
        .expect("couldn't convert into ImageBitmapRenderingContext");

    #[cfg(target_arch = "wasm32")]
    let offscreen_canvas_setup = OffscreenCanvasSetup { offscreen_canvas, bitmap_renderer, };

    let backend = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
    //let backend = wgpu::Backends::GL;
    
    let power_preference = if let Ok(power_preference) = std::env::var("WGPU_POWER_PREF") {
        match power_preference.to_lowercase().as_str() {
            "low" => wgpu::PowerPreference::LowPower,
            "high" => wgpu::PowerPreference::HighPerformance,
            other => panic!("Unknown power preference: {}", other),
        }
    } else {
        wgpu::PowerPreference::HighPerformance
    };
    log::info!("power_preference = {:?}", power_preference);
    let instance = wgpu::Instance::new(backend);
    let (size, surface) = unsafe {

        let size = window.inner_size();

        #[cfg(not(target_arch = "wasm32"))]
        let surface = instance.create_surface(&window);

        #[cfg(target_arch = "wasm32")]
        let surface = instance .create_surface_from_offscreen_canvas(&offscreen_canvas_setup.offscreen_canvas);

        (size, surface)
    };

    let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
        .await
        .expect("No suitable GPU adapters found on the system!");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapter_info = adapter.get_info();
        log::info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
    }

    let optional_features = P::optional_features();
    let required_features = P::required_features();
    let adapter_features = adapter.features();

    assert!(
        adapter_features.contains(required_features),
        "Adapter does not support required features for this example: {:?}",
        required_features - adapter_features
    );

    let required_downlevel_capabilities = P::required_downlevel_capabilities();
    let downlevel_capabilities = adapter.get_downlevel_capabilities();
    assert!(
        downlevel_capabilities.shader_model >= required_downlevel_capabilities.shader_model,
        "Adapter does not support the minimum shader model required to run this example: {:?}",
        required_downlevel_capabilities.shader_model
        );
    assert!(
        downlevel_capabilities
        .flags
        .contains(required_downlevel_capabilities.flags),
        "Adapter does not support the downlevel capabilities required to run this example: {:?}",
        required_downlevel_capabilities.flags - downlevel_capabilities.flags
        );

    let needed_limits = P::required_limits().using_resolution(adapter.limits());

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: (optional_features & adapter_features) | required_features,
                limits: needed_limits,
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");
      
    let sc_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
    };

    surface.configure(&device, &sc_desc);

    Ok(WGPUConfiguration {
            window: window,
            event_loop: event_loop,
            instance: instance,
            size: size,
            surface: surface,
            adapter: adapter,
            device: device,
            queue: queue,
            sc_desc: sc_desc,
            #[cfg(target_arch = "wasm32")]
            offscreen_canvas_setup,
    })
}

/// Initializes wgpu-rs basic components, application and starts the loop. Native version.
#[cfg(not(target_arch = "wasm32"))]
pub fn run_loop<A: Application, L: Loop, F: WGPUFeatures>() {
    log::info!("Setting up wgpu-rs.");
    let configuration = pollster::block_on(setup::<F>("jihuu")).expect("Failed to create WGPUConfiguration.");
    log::info!("Configurating application.");
    let app = A::init(&configuration);
    log::info!("Initializing loop.");
    let lo = L::init();
    log::info!("Launching the application.");
    lo.run(app, configuration); 
}

/// Initializes wgpu-rs basic components, application and starts the loop. wasm version.
#[cfg(target_arch = "wasm32")]
pub fn run_loop<A: Application, L: Loop, F: WGPUFeatures>() {
    use wasm_bindgen::{prelude::*, JsCast};

    wasm_bindgen_futures::spawn_local(async move {

        log::info!("Setting up wgpu-rs.");
        let configuration = setup::<F>("jihuu").await.unwrap();

        log::info!("Configurating application.");
        let app = A::init(&configuration); 

        log::info!("Creating the application.");
        let lo = L::init();

        let start_closure = Closure::once_into_js(move || lo.run(app, configuration));

        if let Err(error) = call_catch(&start_closure) {
            let is_control_flow_exception = error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
                    e.message().includes("Using exceptions for control flow", 0)
                    });

            if !is_control_flow_exception {
                web_sys::console::error_1(&error);
            }
        }

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(catch, js_namespace = Function, js_name = "prototype.call.call")]
            fn call_catch(this: &JsValue) -> Result<(), JsValue>;
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Spawner<'a> {
    executor: async_executor::LocalExecutor<'a>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Spawner<'a> {
    pub fn new() -> Self {
        Self {
            executor: async_executor::LocalExecutor::new(),
        }
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'a) {
        self.executor.spawn(future).detach();
    }

    fn run_until_stalled(&self) {
        while self.executor.try_tick() {}
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Spawner {}

#[cfg(target_arch = "wasm32")]
impl Spawner {
    fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(future);
    }
} 
