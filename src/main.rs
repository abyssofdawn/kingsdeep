use bracket_lib::{self, prelude::{embedded_resource, link_resource, main_loop, BError, BTermBuilder}, terminal::*};
use kingsdeep::*;
use mlua::UserDataMetatable;

fn main() -> BError {
    embedded_resource!(FONT_10X10, "../resources/10x10.png");
    
    link_resource!(FONT_10X10, "10x10.png");
    let mut gs: State = State::new()?;
    let font_10x10 = "10x10.png";
    let ctx = BTermBuilder::new()
        .with_dimensions(gs.win_size.x, gs.win_size.y)
        .with_automatic_console_resize(true)
        .with_title("title")
        .with_tile_dimensions(10, 10)
        .with_simple_console(gs.win_size.x, gs.win_size.y, font_10x10)
        .with_gutter(10)
        .with_font(font_10x10,10, 10)
        .build()?;


    gs.world.loaded_tiles.insert(0, LoadedTile { name: "test 1".to_string(), icon: 219, sprite: Vec2d::new((5, 5).into()) });

    gs.world.loaded_tiles.get_mut(&0).unwrap().sprite.set(vec![
        219, 219, 219, 219, 219,
        219, 0, 0, 0, 219,
        219, 0, 0, 0, 219,
        219, 0, 0, 0, 219,
        219, 219, 219, 219, 219,
    ]);


    gs.world.loaded_areas.insert(0, LoadedArea {
        name: "test 2".to_string(),
        size: Vec2::new(0, 0),
        tiles: Vec2d::new((7, 7).into())
    });

    gs.world.loaded_areas.get_mut(&0).unwrap().tiles.fill(0);

    register_palette_color("blue", RGB::named(AQUAMARINE));
    main_loop(ctx, gs)
}
