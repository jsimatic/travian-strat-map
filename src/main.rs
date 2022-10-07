mod t5_api;
mod t5_data;
mod tsm_config;

use plotters::prelude::*;
use t5_data::{GameWorld, Id, IdMap, Named};

fn search_by_name<T>(map: &IdMap<T>, key: &str) -> Option<Id>
where
    T: Named,
{
    map.iter()
        .find(|(_k, v)| key == v.name())
        .and_then(|(k, _v)| Some(*k))
}

fn rect_center_to_edges(coords: (i32, i32), size: i32) -> [(f32, f32); 2] {
    let x = coords.0 as f32;
    let y = coords.1 as f32;
    let size = size as f32;
    [
        (x - size / 2.0, y - size / 2.0),
        (x + size / 2.0, y + size / 2.0),
    ]
}

fn main() {
    let config = tsm_config::read_config("conf.yaml").unwrap();
    let raw_data = t5_api::get_raw_map_data(&config.key).unwrap();
    let gw = GameWorld::from_raw_data(raw_data.as_str()).unwrap();
    let radius = gw.radius as f32;

    const OUT_FILE_NAME: &'static str = "map.svg";
    let root = SVGBackend::new(OUT_FILE_NAME, (1024, 1024)).into_drawing_area();

    root.fill(&WHITE).unwrap();
    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-radius..radius, -radius..radius)
        .unwrap();
    scatter_ctx.configure_mesh().draw().unwrap();

    for (name, color) in tsm_config::kingdom_color_vector(&config.kingdoms) {
        let kid = search_by_name(&gw.kingdoms, name)
            .expect(format!("{} kingdom not found", name).as_str());
        scatter_ctx
            .draw_series(
                gw.kingdom_villages(&kid)
                    .iter()
                    .map(|v| Rectangle::new(rect_center_to_edges(v.coords, 1), color.filled())),
            )
            .expect("Nothing to draw");
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}
