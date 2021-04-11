use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use tetra::audio::{Sound};
use tetra::graphics::text::{Font, Text};


const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_SPEED: f32 = 5.0;
const BALL_ACC: f32 = 0.05;
const TEXT_POS: Vec2<f32> = Vec2::new(16.0, 16.0);

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    sound: Sound,
    win_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);


        let player1 = Entity::new(player1_texture, player1_position);
        let player2 = Entity::new(player2_texture, player2_position);
        let ball =Entity::with_velocity(ball_texture, ball_position, ball_velocity);
        let sound = Sound::new("./resources/pong.wav")?;
        let win_text = Text::new("lorem ipsum",
            Font::vector(ctx, "./resources/LEMONMILK-Regular.ttf", 16.0)?,
        );


        Ok(GameState {
            player1,
            player2,
            ball,
            sound,
            win_text,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::Z) && self.player1.position.y>0.0 {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) && self.player1.position.y<WINDOW_HEIGHT - self.player1.texture.height() as f32   {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up)  && self.player2.position.y>0.0{
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) && self.player2.position.y<  WINDOW_HEIGHT - self.player2.texture.height() as f32 {
            self.player2.position.y += PADDLE_SPEED;
        }
        
        if input::is_key_down(ctx, Key::Enter){
            self.player1.position = Vec2::new(
                16.0,
                (WINDOW_HEIGHT - self.player1.texture.height() as f32) / 2.0,
            );
            self.player2.position = Vec2::new(
                WINDOW_WIDTH - self.player2.texture.width() as f32 - 16.0,
                (WINDOW_HEIGHT - self.player2.texture.height() as f32) / 2.0,
            );
            self.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - self.ball.texture.width() as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - self.ball.texture.height() as f32 / 2.0,
            );
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        }


        self.ball.position += self.ball.velocity;

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            //pong sound
            self.sound.play(ctx)?;
            // Increase the ball's velocity, then flip it.
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
            // Calculate the offset between the paddle and the ball, number in [-1,1]
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        if self.ball.position.x < 0.0 {
            self.win_text.set_content("Player 2 win ! \nPress enter to play again");
            self.win_text.draw(ctx, TEXT_POS);
        }

        if self.ball.position.x > WINDOW_WIDTH {
            self.win_text.set_content("Player 1 win ! \nPress enter to play again");
            self.win_text.draw(ctx, TEXT_POS);
        }
        Ok(())
    }
}
