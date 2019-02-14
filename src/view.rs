use graphics::types::Color;
use graphics::{Context, Graphics};
use crate::controler::Controler;
use crate::pacman::map::{Tile, PU};
use crate::pacman::Direction;

pub struct View {
    // background_color: Color,
    wall_color: Color,
    ghost_colors: [Color;4],
    frightened_color: Color,
    dot_color: Color,
    tile_size: f64,
    offset: f64,
}

impl View {
    pub fn new() -> Self {
        View {
            // background_color: [0.1294, 0.1294, 0.8706, 1.0],
            wall_color: [0.1294, 0.1294, 0.8706, 1.0],
            ghost_colors: [
                [1.0, 0.0, 0.0, 1.0],
                [1.0, 0.7216, 1.0, 1.0],
                [0.0, 1.0, 1.0, 1.0],
                [1.0, 0.7216, 0.3176, 1.0],
            ],
            frightened_color: [0.0039, 0.0902, 1.0, 1.0],
            dot_color: [1.0, 1.0, 1.0, 1.0],
            tile_size: 20.0,
            offset: 0.0,
        }
    }

    pub fn draw<G: Graphics>(&self, controler: &Controler, c: &Context, g: &mut G) {
        use graphics::{Rectangle, CircleArc, Text};

        let mut x = 0.0;
        let mut y = 0.0;
        for line in controler.get_map().scan_lines() {
            for tile in line.iter() {
                match tile {
                    Tile::Wall => {
                        let sqr = [
                            self.offset + x + self.tile_size / 4.0,
                            self.offset + y + self.tile_size / 4.0,
                            self.tile_size / 2.0,
                            self.tile_size / 2.0
                        ];
                        Rectangle::new(self.wall_color)
                            .draw(sqr, &c.draw_state, c.transform, g);
                    },
                    Tile::NotWall(PU::Dot) => {
                        let sqr = [
                            self.offset + x + self.tile_size * (5.0/12.0),
                            self.offset + y + self.tile_size * (5.0/12.0),
                            self.tile_size / 6.0,
                            self.tile_size / 6.0
                        ];
                        Rectangle::new(self.dot_color)
                            .draw(sqr, &c.draw_state, c.transform, g);
                    },
                    Tile::NotWall(PU::PowerUp) => {
                        let sqr = [
                            self.offset + x + self.tile_size * (3.0/8.0),
                            self.offset + y + self.tile_size * (3.0/8.0),
                            self.tile_size / 4.0,
                            self.tile_size / 4.0
                        ];
                        CircleArc::new(self.dot_color, self.tile_size / 4.0, 0.0, 2.0 * 3.14)
                            .draw(sqr, &c.draw_state, c.transform, g);
                    },
                    _ => (),
                }
                x += self.tile_size;
            }
            y += self.tile_size;
            x = 0.0;
        }

        let (x, y, d) = controler.get_player();
        let pacman = [
            self.offset + 5.0 + (x as f64) * self.tile_size,
            self.offset + 5.0 + (y as f64) * self.tile_size,
            self.tile_size / 2.0,
            self.tile_size / 2.0,
        ];
        const DOWN_RIGHT: f64 = 3.14 / 4.0;
        const DOWN_LEFT: f64  = ((3.0 * 3.14) / 4.0);
        const UP_RIGHT: f64   = -DOWN_RIGHT;
        const UP_LEFT: f64    = -DOWN_LEFT;
        let (start, end) = match d {
            Direction::Up    => (UP_RIGHT,   UP_LEFT),
            Direction::Down  => (DOWN_LEFT,  DOWN_RIGHT),
            Direction::Left  => (UP_LEFT,    DOWN_LEFT),
            Direction::Right => (DOWN_RIGHT, UP_RIGHT),
        };
        CircleArc::new([1.0, 1.0, 0.0, 1.0], self.tile_size / 4.0, start, end)
            .draw(pacman, &c.draw_state, c.transform, g);

        let pick_color = |c| if controler.frightened() { self.frightened_color } else { c };

        for (i, ghost) in controler.get_ghosts().iter().enumerate() {
            let g_square = self.ghost_square(ghost.x(), ghost.y());
            Rectangle::new(pick_color(self.ghost_colors[i]))
                .draw(g_square, &c.draw_state, c.transform, g);
        }

        let stats = controler.get_stats();

        // DEBUG
        // for (i, sqr) in controler.ghost_targets().iter().enumerate() {
        //     let g_square = self.ghost_square(sqr.0, sqr.1);
        //     Rectangle::new_border(self.ghost_colors[i], 1.0)
        //         .draw(g_square, &c.draw_state, c.transform, g);
        // }
    }

    fn ghost_square(&self, x: i32, y: i32) -> [f64; 4] {
        [
            self.offset + x as f64 * self.tile_size,
            self.offset + y as f64 * self.tile_size,
            self.tile_size,
            self.tile_size
        ]
    }
}

