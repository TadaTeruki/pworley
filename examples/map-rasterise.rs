use pworley::{
    map::{IDWStrategy, RasterizationMethod, WorleyMap},
    WorleyCell, WorleyParameters,
};

fn main() {
    let parameters = WorleyParameters::new(0.8, 0.8, 1.0, 0).unwrap();
    let cells = WorleyCell::from_inside_radius(0.0, 0.0, parameters, 5.0);
    let values = cells
        .iter()
        .map(|cell| (cell.hash_u64() % 10) as f64 * 0.1)
        .collect::<Vec<_>>();

    let map = WorleyMap::new(parameters, cells.into_iter().zip(values).collect());

    let image_width = 500;
    let image_height = 500;

    let raster = map.rasterise(
        image_width,
        image_height,
        ((-5.0, -5.0), (5.0, 5.0)),
        RasterizationMethod::IDW(IDWStrategy::default_from_parameters(&parameters)),
    );

    let mut image_buf = image::RgbImage::new(image_width as u32, image_height as u32);

    for (iy, row) in raster.iter().enumerate() {
        for (ix, value) in row.iter().enumerate() {
            if let Some(value) = value {
                let v = (value * 255.0) as u8;
                image_buf.put_pixel(ix as u32, iy as u32, image::Rgb([v, v, v]));
            } else {
                image_buf.put_pixel(ix as u32, iy as u32, image::Rgb([0, 0, 0]));
            }
        }
    }

    image_buf.save("data/output/map-rasterise.png").unwrap();
}
