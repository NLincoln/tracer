use env_logger::{Builder, Env};
use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use libtrace::{
    renderer::Renderer,
    scene::{Rendered, Scene},
};

fn main() {
    Builder::from_env(
        Env::default()
            .filter("TRACER_LOG")
            .default_filter_or("warn"),
    )
    .init();
    lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    log::info!("Received request");
    let body = request.body();

    let scene: Scene = serde_json::from_slice(body)?;

    struct WorkerRenderer<'a> {
        scene: &'a Scene,
    }

    impl<'a> Renderer for WorkerRenderer<'a> {
        fn scene(&self) -> &Scene {
            self.scene
        }
    }

    let renderer = WorkerRenderer { scene: &scene };
    let pixels = renderer.render();

    Ok(serde_json::to_string(&Rendered {
        image: scene.image,
        pixels: pixels,
    })?)
}
