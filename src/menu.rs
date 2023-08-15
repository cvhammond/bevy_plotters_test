use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use plotters::prelude::*;
use plotters::backend::BGRXPixel;
use plotters::style::Color as PlottersColor;


pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(menu_setup);
    }
}

fn menu_setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut bytes: Vec<u8> = vec![0; 1000 * 1000 * 4];

    {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut bytes, (1000, 1000)).unwrap().into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 8).into_font())
            .margin(5)
            .x_label_area_size(5)
            .y_label_area_size(5)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();

        root.present().unwrap();
    }



    let image = images.add(Image::new(
        Extent3d {
            width: 1000,
            height: 1000,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Bgra8UnormSrgb,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    size: Size::new(Val::Px(1000.0), Val::Px(1000.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                //background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            });
        });
}
