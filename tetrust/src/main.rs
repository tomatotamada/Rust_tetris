use getch_rs::{Getch, Key};
use std::{thread, time};

//フィールドサイズ
const FIELD_WIDTH: usize = 11 + 2; //左右の壁分を追加
const FIELD_HEIGHT: usize = 20 + 1; //底の壁分を追加
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

//ブロックの種類
#[derive(Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

//ブロックの形状
type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
    //I
    [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    //O
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    //S
    [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    //Z
    [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    //J
    [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    //L
    [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
    //T
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
];

//ポジション
struct Position {
    x: usize,
    y: usize,
}

//ブロックがフィールドに衝突する際はtrueを返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if pos.y + y + 1 >= FIELD_HEIGHT {
                if BLOCKS[block as usize][y][x] == 1 {
                    return true;
                }
                continue;
            }
            if pos.x + x >= FIELD_WIDTH {
                if BLOCKS[block as usize][y][x] == 1 {
                    return true;
                }
                continue;
            }
            if (field[pos.y + y][pos.x + x] & BLOCKS[block as usize][y][x]) == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    //フィールドの定義
    let field = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();

    //画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    //30マス下に移動
    loop {
        //描画用フィールドの生成
        let mut field_buf = field;

        //自然落下
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };

        //衝突判定
        if !is_collision(&field, &pos, BlockKind::I) {
            //衝突しなければ1マス下に移動
            pos = new_pos;
        }

        //ブロックをフィールドに配置
        for y in 0..4 {
            for x in 0..4 {
                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                    field_buf[pos.y + y][pos.x + x] = 1;
                }
            }
        }

        //フィールドを描画
        println!("\x1b[H"); //カーソルを先頭に移動
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if field_buf[y][x] == 1 {
                    print!("[]");
                } else {
                    print!(" .");
                }
            }
            println!();
        }
        //1秒間スリープする
        thread::sleep(time::Duration::from_millis(1000));

        //キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    //カーソルを再表示
    println!("\x1b[?25h");
}
