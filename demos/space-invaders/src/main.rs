//! Space Invaders demo for the SLSTK3701A board.
//!
//! A simplified Space Invaders game on the 176x176 8-color Memory LCD.
//!
//! Controls:
//!   BTN0 (PC8) - Move left
//!   BTN1 (PC9) - Move right
//!   Auto-fire every few frames.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;
use efm32gg11b_pac as pac;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};
use slstk3701a::display;

// --- Colors (3-bit RGB: bit2=R, bit1=G, bit0=B) ---
const BLACK: u8 = 0b000;
const BLUE: u8 = 0b001;
const GREEN: u8 = 0b010;
const CYAN: u8 = 0b011;
const RED: u8 = 0b100;
const MAGENTA: u8 = 0b101;
const WHITE: u8 = 0b111;

// --- Game constants ---
const SCREEN_W: usize = 176;
const SCREEN_H: usize = 176;

const PLAYER_W: usize = 11;
const PLAYER_H: usize = 7;
const PLAYER_Y: usize = 164; // top of player sprite

const ALIEN_W: usize = 9;
const ALIEN_H: usize = 7;
const ALIEN_COLS: usize = 8;
const ALIEN_ROWS: usize = 4;
const ALIEN_SPACING_X: usize = 12;
const ALIEN_SPACING_Y: usize = 12;
const ALIEN_START_X: usize = 20;
const ALIEN_START_Y: usize = 20;

const BULLET_H: usize = 4;

const MAX_BULLETS: usize = 3;
const MAX_ENEMY_BULLETS: usize = 4;

const FIRE_INTERVAL: u32 = 8;
const ENEMY_FIRE_INTERVAL: u32 = 30;
const MOVE_INTERVAL: u32 = 6;

// Frame period in ms (~15 fps).
const FRAME_MS: u32 = 67;

static MILLIS: AtomicU32 = AtomicU32::new(0);
static BTN0_HELD: AtomicBool = AtomicBool::new(false);
static BTN1_HELD: AtomicBool = AtomicBool::new(false);

// --- Sprite data ---

#[rustfmt::skip]
static PLAYER_SPRITE: [[u8; 11]; 7] = [
    [0,0,0,0,0,1,0,0,0,0,0],
    [0,0,0,0,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,0,0,0,0],
    [0,0,0,1,1,1,1,1,0,0,0],
    [0,1,1,1,1,1,1,1,1,1,0],
    [1,1,1,1,1,1,1,1,1,1,1],
    [1,1,0,1,1,1,1,1,0,1,1],
];

#[rustfmt::skip]
static ALIEN_A: [[u8; 9]; 7] = [
    [0,0,1,0,0,0,1,0,0],
    [0,0,0,1,0,1,0,0,0],
    [0,0,1,1,1,1,1,0,0],
    [0,1,1,0,1,0,1,1,0],
    [1,1,1,1,1,1,1,1,1],
    [1,0,1,1,1,1,1,0,1],
    [1,0,1,0,0,0,1,0,1],
];

#[rustfmt::skip]
static ALIEN_B: [[u8; 9]; 7] = [
    [0,0,0,1,1,1,0,0,0],
    [0,1,1,1,1,1,1,1,0],
    [1,1,1,1,1,1,1,1,1],
    [1,1,0,1,0,1,0,1,1],
    [1,1,1,1,1,1,1,1,1],
    [0,0,1,0,0,0,1,0,0],
    [0,1,0,0,0,0,0,1,0],
];

// --- Game state ---

struct Bullet {
    x: i16,
    y: i16,
    active: bool,
}

struct Game {
    player_x: i16,
    aliens: [[bool; ALIEN_COLS]; ALIEN_ROWS],
    alien_offset_x: i16,
    alien_offset_y: i16,
    alien_dir: i16,
    bullets: [Bullet; MAX_BULLETS],
    enemy_bullets: [Bullet; MAX_ENEMY_BULLETS],
    score: u32,
    frame: u32,
    game_over: bool,
    aliens_remaining: u8,
    wave: u8,
    enemy_fire_counter: u32,
}

const INACTIVE: Bullet = Bullet {
    x: 0,
    y: 0,
    active: false,
};

impl Game {
    fn new() -> Self {
        Game {
            player_x: (SCREEN_W / 2 - PLAYER_W / 2) as i16,
            aliens: [[true; ALIEN_COLS]; ALIEN_ROWS],
            alien_offset_x: 0,
            alien_offset_y: 0,
            alien_dir: 1,
            bullets: [INACTIVE; MAX_BULLETS],
            enemy_bullets: [INACTIVE; MAX_ENEMY_BULLETS],
            score: 0,
            frame: 0,
            game_over: false,
            aliens_remaining: (ALIEN_ROWS * ALIEN_COLS) as u8,
            wave: 1,
            enemy_fire_counter: 0,
        }
    }

    fn reset_aliens(&mut self) {
        self.aliens = [[true; ALIEN_COLS]; ALIEN_ROWS];
        self.alien_offset_x = 0;
        self.alien_offset_y = 0;
        self.alien_dir = 1;
        self.aliens_remaining = (ALIEN_ROWS * ALIEN_COLS) as u8;
        self.wave += 1;
        for b in &mut self.enemy_bullets {
            b.active = false;
        }
    }

    fn fire(&mut self) {
        for b in &mut self.bullets {
            if !b.active {
                b.x = self.player_x + (PLAYER_W / 2) as i16;
                b.y = PLAYER_Y as i16 - BULLET_H as i16;
                b.active = true;
                break;
            }
        }
    }

    fn enemy_fire(&mut self) {
        let col = (self.frame as usize + self.score as usize) % ALIEN_COLS;
        for row in (0..ALIEN_ROWS).rev() {
            if self.aliens[row][col] {
                for b in &mut self.enemy_bullets {
                    if !b.active {
                        let ax = ALIEN_START_X as i16
                            + col as i16 * ALIEN_SPACING_X as i16
                            + self.alien_offset_x
                            + (ALIEN_W / 2) as i16;
                        let ay = ALIEN_START_Y as i16
                            + row as i16 * ALIEN_SPACING_Y as i16
                            + self.alien_offset_y
                            + ALIEN_H as i16;
                        b.x = ax;
                        b.y = ay;
                        b.active = true;
                        break;
                    }
                }
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.frame += 1;

        // Move player.
        if BTN0_HELD.load(Ordering::Relaxed) {
            self.player_x -= 3;
            if self.player_x < 0 {
                self.player_x = 0;
            }
        }
        if BTN1_HELD.load(Ordering::Relaxed) {
            self.player_x += 3;
            if self.player_x > (SCREEN_W - PLAYER_W) as i16 {
                self.player_x = (SCREEN_W - PLAYER_W) as i16;
            }
        }

        // Auto-fire.
        if self.frame % FIRE_INTERVAL == 0 {
            self.fire();
        }

        // Move player bullets up.
        for b in &mut self.bullets {
            if b.active {
                b.y -= 3;
                if b.y < 0 {
                    b.active = false;
                }
            }
        }

        // Move enemy bullets down.
        for b in &mut self.enemy_bullets {
            if b.active {
                b.y += 2;
                if b.y > SCREEN_H as i16 {
                    b.active = false;
                }
            }
        }

        // Enemy fire.
        self.enemy_fire_counter += 1;
        let interval = ENEMY_FIRE_INTERVAL
            .saturating_sub(self.wave as u32 * 3)
            .max(10);
        if self.enemy_fire_counter >= interval {
            self.enemy_fire_counter = 0;
            self.enemy_fire();
        }

        // Move aliens.
        if self.frame % MOVE_INTERVAL == 0 {
            self.alien_offset_x += self.alien_dir * 2;

            let mut min_col = ALIEN_COLS;
            let mut max_col = 0;
            for row in 0..ALIEN_ROWS {
                for col in 0..ALIEN_COLS {
                    if self.aliens[row][col] {
                        if col < min_col {
                            min_col = col;
                        }
                        if col > max_col {
                            max_col = col;
                        }
                    }
                }
            }
            if min_col < ALIEN_COLS {
                let left_edge = ALIEN_START_X as i16
                    + min_col as i16 * ALIEN_SPACING_X as i16
                    + self.alien_offset_x;
                let right_edge = ALIEN_START_X as i16
                    + max_col as i16 * ALIEN_SPACING_X as i16
                    + self.alien_offset_x
                    + ALIEN_W as i16;

                if right_edge >= SCREEN_W as i16 - 2 || left_edge <= 2 {
                    self.alien_dir = -self.alien_dir;
                    self.alien_offset_y += 4;
                }
            }
        }

        // Collision: player bullets vs aliens.
        for b in &mut self.bullets {
            if !b.active {
                continue;
            }
            let mut hit = false;
            for row in 0..ALIEN_ROWS {
                let ay = ALIEN_START_Y as i16
                    + row as i16 * ALIEN_SPACING_Y as i16
                    + self.alien_offset_y;
                // Skip this row entirely if bullet is above or below it.
                if b.y >= ay + ALIEN_H as i16 || b.y + BULLET_H as i16 <= ay {
                    continue;
                }
                for col in 0..ALIEN_COLS {
                    if !self.aliens[row][col] {
                        continue;
                    }
                    let ax = ALIEN_START_X as i16
                        + col as i16 * ALIEN_SPACING_X as i16
                        + self.alien_offset_x;
                    if b.x >= ax && b.x < ax + ALIEN_W as i16 {
                        self.aliens[row][col] = false;
                        b.active = false;
                        self.score += (ALIEN_ROWS - row) as u32 * 10;
                        self.aliens_remaining -= 1;
                        hit = true;
                        break;
                    }
                }
                if hit {
                    break;
                }
            }
        }

        // Collision: enemy bullets vs player.
        for b in &mut self.enemy_bullets {
            if !b.active {
                continue;
            }
            if b.x >= self.player_x
                && b.x < self.player_x + PLAYER_W as i16
                && b.y >= PLAYER_Y as i16
                && b.y < PLAYER_Y as i16 + PLAYER_H as i16
            {
                self.game_over = true;
                b.active = false;
            }
        }

        // Check if aliens reached player — scan bottom-up, exit early.
        'reached: for row in (0..ALIEN_ROWS).rev() {
            let ay = ALIEN_START_Y as i16
                + row as i16 * ALIEN_SPACING_Y as i16
                + self.alien_offset_y
                + ALIEN_H as i16;
            if ay < PLAYER_Y as i16 {
                break; // All higher rows are above the player too.
            }
            for col in 0..ALIEN_COLS {
                if self.aliens[row][col] {
                    self.game_over = true;
                    break 'reached;
                }
            }
        }

        // All aliens dead -> new wave.
        if self.aliens_remaining == 0 {
            self.reset_aliens();
        }
    }
}

// --- Rendering ---

fn draw_sprite<const W: usize, const H: usize>(
    rows_buf: &mut [[u8; display::BYTES_PER_ROW]],
    sprite: &[[u8; W]; H],
    sx: i16,
    sy: i16,
    buf_start_y: usize,
    color: u8,
) {
    for (py, row) in sprite.iter().enumerate() {
        let screen_y = sy + py as i16;
        if screen_y < buf_start_y as i16 || screen_y >= buf_start_y as i16 + rows_buf.len() as i16 {
            continue;
        }
        let buf_y = (screen_y - buf_start_y as i16) as usize;
        for (px, &on) in row.iter().enumerate() {
            if on != 0 {
                let screen_x = sx + px as i16;
                if screen_x >= 0 && (screen_x as usize) < SCREEN_W {
                    display::set_pixel(&mut rows_buf[buf_y], screen_x as usize, color);
                }
            }
        }
    }
}

const NUM_BANDS: usize = (SCREEN_H + 7) / 8; // 22 bands

/// Mark which 8-row bands a y-range overlaps.
fn mark_bands(dirty: &mut u32, y_top: i16, h: usize) {
    if y_top < 0 {
        return;
    }
    let y0 = y_top as usize;
    let y1 = y0 + h;
    let band0 = y0 / 8;
    let band1 = (y1 + 7) / 8;
    for b in band0..band1.min(NUM_BANDS) {
        *dirty |= 1 << b;
    }
}

/// Compute per-band dirty bitmask for all game objects.
fn dirty_bands(game: &Game) -> u32 {
    let mut dirty: u32 = 0;

    // Player.
    mark_bands(&mut dirty, PLAYER_Y as i16, PLAYER_H);

    // Aliens — mark once per row that has any alive alien (all aliens in
    // a row share the same y, so we only need one mark_bands call per row).
    for row in 0..ALIEN_ROWS {
        let mut row_alive = false;
        for col in 0..ALIEN_COLS {
            if game.aliens[row][col] {
                row_alive = true;
                break;
            }
        }
        if row_alive {
            let ay =
                ALIEN_START_Y as i16 + row as i16 * ALIEN_SPACING_Y as i16 + game.alien_offset_y;
            mark_bands(&mut dirty, ay, ALIEN_H);
        }
    }

    // Bullets.
    for b in &game.bullets {
        if b.active {
            mark_bands(&mut dirty, b.y, BULLET_H);
        }
    }
    for b in &game.enemy_bullets {
        if b.active {
            mark_bands(&mut dirty, b.y, BULLET_H);
        }
    }

    dirty
}

fn render_game(game: &Game, last_score: &mut u32, prev_dirty: &mut u32, vcom: &mut bool) {
    // Score bar — only redraw when score or wave changes.
    if game.score != *last_score {
        *last_score = game.score;
        let mut score_str = [b' '; 22];
        score_str[0] = b'S';
        score_str[1] = b':';
        let mut buf = [0u8; 10];
        let digits = display::format_u32(game.score, &mut buf);
        for (i, &d) in digits.iter().enumerate() {
            if 2 + i < 22 {
                score_str[2 + i] = d;
            }
        }
        score_str[14] = b'W';
        score_str[15] = b':';
        let wave_digits = display::format_u32(game.wave as u32, &mut buf);
        for (i, &d) in wave_digits.iter().enumerate() {
            if 16 + i < 22 {
                score_str[16 + i] = d;
            }
        }
        let s = core::str::from_utf8(&score_str).unwrap_or("");
        display::draw_text_colored(0, 0, s, BLUE, WHITE, vcom);
    }

    // Compute dirty bands: union of current and previous frame.
    let cur_dirty = dirty_bands(game);
    let dirty = cur_dirty | *prev_dirty;
    *prev_dirty = cur_dirty;

    // Skip band 0 (score bar, handled above).
    let mut band_buf = [[0xFFu8; display::BYTES_PER_ROW]; 8];

    for band in 1..NUM_BANDS {
        if dirty & (1 << band) == 0 {
            continue;
        }

        let y = band * 8;
        let band_end = (y + 8).min(SCREEN_H);
        let band_rows = band_end - y;

        // Clear band to white.
        for row in band_buf[..band_rows].iter_mut() {
            for b in row.iter_mut() {
                *b = 0xFF;
            }
        }

        // Draw aliens — check row overlap once, then iterate columns.
        for row in 0..ALIEN_ROWS {
            let ay =
                ALIEN_START_Y as i16 + row as i16 * ALIEN_SPACING_Y as i16 + game.alien_offset_y;
            if ay + ALIEN_H as i16 <= y as i16 || ay >= band_end as i16 {
                continue;
            }
            let color = match row {
                0 => RED,
                1 => MAGENTA,
                2 => CYAN,
                _ => GREEN,
            };
            for col in 0..ALIEN_COLS {
                if !game.aliens[row][col] {
                    continue;
                }
                let ax = ALIEN_START_X as i16
                    + col as i16 * ALIEN_SPACING_X as i16
                    + game.alien_offset_x;
                if row < 2 {
                    draw_sprite(&mut band_buf[..band_rows], &ALIEN_A, ax, ay, y, color);
                } else {
                    draw_sprite(&mut band_buf[..band_rows], &ALIEN_B, ax, ay, y, color);
                }
            }
        }

        // Draw player bullets.
        for b in &game.bullets {
            if !b.active {
                continue;
            }
            if b.y + BULLET_H as i16 <= y as i16 || b.y >= band_end as i16 {
                continue;
            }
            for py in 0..BULLET_H {
                let sy = b.y + py as i16;
                if sy >= y as i16 && sy < band_end as i16 && b.x >= 0 && (b.x as usize) < SCREEN_W {
                    display::set_pixel(
                        &mut band_buf[(sy - y as i16) as usize],
                        b.x as usize,
                        BLACK,
                    );
                }
            }
        }

        // Draw enemy bullets.
        for b in &game.enemy_bullets {
            if !b.active {
                continue;
            }
            if b.y + BULLET_H as i16 <= y as i16 || b.y >= band_end as i16 {
                continue;
            }
            for py in 0..BULLET_H {
                let sy = b.y + py as i16;
                if sy >= y as i16 && sy < band_end as i16 && b.x >= 0 && (b.x as usize) < SCREEN_W {
                    display::set_pixel(&mut band_buf[(sy - y as i16) as usize], b.x as usize, RED);
                }
            }
        }

        // Draw player.
        if PLAYER_Y + PLAYER_H > y && PLAYER_Y < band_end {
            draw_sprite(
                &mut band_buf[..band_rows],
                &PLAYER_SPRITE,
                game.player_x,
                PLAYER_Y as i16,
                y,
                BLUE,
            );
        }

        display::write_rows(y as u8, &band_buf[..band_rows], vcom);
    }
}

fn render_game_over(game: &Game, vcom: &mut bool) {
    display::draw_text_colored(9, 3, "  GAME OVER!  ", RED, WHITE, vcom);
    let mut buf = [0u8; 10];
    let mut line = [b' '; 22];
    line[3] = b'S';
    line[4] = b'c';
    line[5] = b'o';
    line[6] = b'r';
    line[7] = b'e';
    line[8] = b':';
    line[9] = b' ';
    let digits = display::format_u32(game.score, &mut buf);
    for (i, &d) in digits.iter().enumerate() {
        if 10 + i < 22 {
            line[10 + i] = d;
        }
    }
    let s = core::str::from_utf8(&line).unwrap_or("");
    display::draw_text_colored(11, 0, s, BLACK, WHITE, vcom);
    display::draw_text_colored(13, 2, "BTN0+BTN1:Restart", BLUE, WHITE, vcom);
}

// --- Entry point ---

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

    p.wdog0.ctrl().modify(|_, w| w.en().clear_bit());
    p.wdog1.ctrl().modify(|_, w| w.en().clear_bit());

    // Enable GPIO clock.
    p.cmu.hfbusclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfbusclken0().read();

    // --- Display ---
    display::init();
    let mut vcom = false;
    display::clear(&mut vcom);

    // --- Buttons: PC8 (BTN0), PC9 (BTN1) as input with pull-up ---
    p.gpio
        .pc_modeh()
        .modify(|_, w| w.mode8().inputpullfilter().mode9().inputpullfilter());
    p.gpio
        .pc_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 8) | (1 << 9)) });

    // --- SysTick at 1 kHz ---
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(19_000 - 1);
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt();

    defmt::info!("Space Invaders demo started");

    let mut game = Game::new();

    // Title screen.
    display::draw_text_colored(5, 2, " SPACE INVADERS ", RED, WHITE, &mut vcom);
    display::draw_text_colored(8, 2, " BTN0 = Left    ", BLACK, WHITE, &mut vcom);
    display::draw_text_colored(9, 2, " BTN1 = Right   ", BLACK, WHITE, &mut vcom);
    display::draw_text_colored(10, 2, " Auto-Fire      ", BLACK, WHITE, &mut vcom);
    display::draw_text_colored(13, 2, " Press any BTN  ", BLUE, WHITE, &mut vcom);

    // Wait for button press to start.
    let mut last_vcom: u32 = 0;
    loop {
        cortex_m::asm::wfi();
        let now = MILLIS.load(Ordering::Relaxed);
        if now.wrapping_sub(last_vcom) >= 16 {
            last_vcom = now;
            display::toggle_vcom();
        }
        // Poll buttons directly.
        let gpio = unsafe { &*pac::Gpio::ptr() };
        let pins = gpio.pc_din().read().bits();
        if pins & (1 << 8) == 0 || pins & (1 << 9) == 0 {
            cortex_m::asm::delay(3_800_000); // debounce
            break;
        }
    }

    display::clear(&mut vcom);

    // Game loop.
    let mut last_frame: u32 = MILLIS.load(Ordering::Relaxed);
    last_vcom = last_frame;
    let mut last_score: u32 = u32::MAX;
    let mut prev_dirty: u32 = 0xFFFF_FFFF; // first frame redraws all
    loop {
        cortex_m::asm::wfi();

        let now = MILLIS.load(Ordering::Relaxed);

        // Toggle VCOM at ~60 Hz.
        if now.wrapping_sub(last_vcom) >= 16 {
            last_vcom = now;
            display::toggle_vcom();
        }

        // Frame tick at ~15 fps.
        if now.wrapping_sub(last_frame) < FRAME_MS {
            continue;
        }
        last_frame = now;

        // Poll button state (held detection).
        let gpio_reg = unsafe { &*pac::Gpio::ptr() };
        let pins = gpio_reg.pc_din().read().bits();
        BTN0_HELD.store(pins & (1 << 8) == 0, Ordering::Relaxed);
        BTN1_HELD.store(pins & (1 << 9) == 0, Ordering::Relaxed);

        if game.game_over {
            if game.frame != 0 {
                render_game_over(&game, &mut vcom);
                game.frame = 0;
            }
            // Restart on both buttons.
            if pins & (1 << 8) == 0 && pins & (1 << 9) == 0 {
                cortex_m::asm::delay(3_800_000);
                game = Game::new();
                display::clear(&mut vcom);
                last_score = u32::MAX;
                prev_dirty = 0xFFFF_FFFF;
            }
            continue;
        }

        game.update();
        render_game(&game, &mut last_score, &mut prev_dirty, &mut vcom);
    }
}

// --- SysTick handler ---

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}
