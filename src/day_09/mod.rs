use std::fs;

#[derive(Debug)]
struct Block {
    id: Option<usize>,
    size: usize,
}

pub fn part_1() {
    let contents = fs::read_to_string("src/day_09/input.txt").unwrap();

    let mut next_block_is_full = true;
    let mut next_block_id: usize = 0;
    let mut blocks: Vec<Block> = vec![];

    for char in contents.chars() {
        let size: usize = char.to_string().parse().unwrap();
        if next_block_is_full {
            blocks.push(Block {
                id: Some(next_block_id),
                size,
            });
            next_block_is_full = false;
            next_block_id += 1;
        } else {
            blocks.push(Block { id: None, size });
            next_block_is_full = true;
        }
    }
    // println!("{:?}", blocks);
    let mut forward_index = 0usize;
    let mut backward_index = blocks.len() - 1;
    while backward_index > forward_index {
        // iterate indices forwards/backwards to find some values that can be moved
        while blocks[forward_index].id.is_some() || blocks[forward_index].size == 0 {
            forward_index += 1;
        }
        while blocks[backward_index].id.is_none() || blocks[backward_index].size == 0 {
            backward_index -= 1;
        }
        let elements_to_move =
            std::cmp::min(blocks[forward_index].size, blocks[backward_index].size);
        blocks[forward_index].size -= elements_to_move;
        blocks[backward_index].size -= elements_to_move;
        blocks.insert(
            forward_index,
            Block {
                id: blocks[backward_index].id,
                size: elements_to_move,
            },
        );
        forward_index += 1;
        backward_index += 1;
        // println!("{:?},{:?},{:?}", blocks, forward_index, backward_index );
        // if forward_index >2 {
        //     break;
        // }
    }
    // filter out empty blocks for convenience
    blocks.retain(|block| block.size > 0);
    // println!("{:?}", blocks);
    let mut current_index = 0usize;
    let mut total_score = 0usize;
    for block in blocks.iter() {
        for _ in 0..block.size {
            if let Some(b_id) = block.id {
                total_score += b_id * current_index
            }
            current_index += 1;
        }
    }
    println!("{:?}", total_score);
}

pub fn part_2() {
    let contents = fs::read_to_string("src/day_09/input.txt").unwrap();

    let mut next_block_is_full = true;
    let mut next_block_id: usize = 0;
    let mut blocks: Vec<Block> = vec![];

    for char in contents.chars() {
        let size: usize = char.to_string().parse().unwrap();
        if next_block_is_full {
            blocks.push(Block {
                id: Some(next_block_id),
                size,
            });
            next_block_is_full = false;
            next_block_id += 1;
        } else {
            blocks.push(Block { id: None, size });
            next_block_is_full = true;
        }
    }
    // println!("{:?}", blocks);
    let mut backward_index = blocks.len() - 1;
    'main_loop: while backward_index > 0 {
        // look for a movable block
        while blocks[backward_index].id.is_none() || blocks[backward_index].size == 0 {
            backward_index -= 1;
        }
        let current_size = blocks[backward_index].size;
        let mut forward_index = 0usize;
        while blocks[forward_index].id.is_some() || blocks[forward_index].size < current_size || forward_index >= backward_index {
            if forward_index >= backward_index {
                // Not possible to move this block!
                backward_index -= 1;
                continue 'main_loop;
            }
            forward_index += 1;
        }
        blocks[forward_index].size -= current_size;
        blocks.insert(
            forward_index,
            Block {
                id: blocks[backward_index].id,
                size: current_size,
            },
        );
        backward_index += 1;
        blocks[backward_index].id = None;
        // println!("{:?},{:?},{:?}", blocks, forward_index, backward_index );
    }
    // filter out empty blocks for convenience
    blocks.retain(|block| block.size > 0);
    println!("{:?}", blocks);
    let mut current_index = 0usize;
    let mut total_score = 0usize;
    for block in blocks.iter() {
        for _ in 0..block.size {
            if let Some(b_id) = block.id {
                total_score += b_id * current_index
            }
            current_index += 1;
        }
    }
    println!("{:?}", total_score);
}
