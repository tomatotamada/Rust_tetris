use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};
mod block;
mod field;
use block::{BLOCKS, BlockKind};
use field::{FIELD, FIELD_HEIGHT, FIELD_WIDTH, Field};

//ポジション
struct Position {
    x: usize,
    y: usize,
}

//ブロックがフィールドに衝突する際はtrueを返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if pos.y + y >= FIELD_HEIGHT {
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

//フィールドを描画する関数
fn draw(field: &Field, pos: &Position, block: BlockKind) {
    //描画用フィールドの生成
    let mut field_buf = field.clone();
    //描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] == 1 {
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
}

fn main() {
    let field = Arc::new(Mutex::new(FIELD));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));
    //画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    //フィールドを描画
    draw(&field.lock().unwrap(), &pos.lock().unwrap(), *block.lock().unwrap());

    //自然落下処理
    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let block = Arc::clone(&block);
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(1000));
                //自然落下
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                } else {
                    //ブロックをフィールドに固定
                    for y in 0..4 {
                        for x in 0..4 {
                            if BLOCKS[*block as usize][y][x] == 1 {
                                field[pos.y + y][pos.x + x] = 1;
                            }
                        }
                    }
                    //新しいブロックを生成
                    *pos = Position { x: 4, y: 0 };
                    *block = rand::random();
                }

                //フィールドを描画
                draw(&field, &pos, *block);
            }
        });
    }

    //キー入力処理
    let g = Getch::new();
    loop {
        //キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => {
                //カーソルを再表示
                println!("\x1b[?25h");
                return;
            }
            _ => (),
        }
    }
}
