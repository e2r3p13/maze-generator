/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   maze.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bccyv <bccyv@student.42.fr>                +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/12 20:37:13 by bccyv             #+#    #+#             */
/*   Updated: 2021/07/13 10:46:44 by bccyv            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

pub struct Maze {
    w: usize,
    h: usize,
    links: Vec<bool>,
}

pub enum Generator {
    Backtrack,
    // Eller,
    // Krusal,
    // Prim,
    // Division,
    // AldousBroder,
    // Wilson,
    // HuntAndKill,
    // GrowingTree,
}

impl Maze {

    fn new(w: usize, h: usize) -> Maze {
        Maze {
            w: w,
            h: h,
            links: vec![false; h * (w - 1) + w * (h - 1)],
        }
    }

    pub fn generate(w: usize, h: usize, generator: Generator) -> Maze {
        let mut maze = Maze::new(w, h);
        generator.run(&mut maze);
        maze
    }

    pub fn print_blockwise(&self) {

    }

    pub fn print_linewise(&self) {

    }

    pub fn export(&self) {

    }
}

impl Generator {

    pub fn run(self, maze: &mut Maze) {
        match self {
            Generator::Backtrack => Generator::genearate_with_backtrack(maze),
            _ => (),
        };
    }

    fn genearate_with_backtrack(maze: &mut Maze) {
        let mut visited_cells: Vec<bool> = vec![false; maze.w * maze.h];
        let starting_cell_id = rand::thread_rng().gen_range(0..visited_cells.len());
        carve_passage(maze, &mut visited_cells, starting_cell_id);
    }

}

fn carve_passage(maze: &mut Maze, visited: &mut Vec<bool>, cell_id: usize) {
    visited[cell_id] = true;
    let mut directions = [0, 1, 2, 3]; // Left Top Right Bottom

    let mut rng = thread_rng();
	directions.shuffle(&mut rng);

    for direction in directions.iter() {

        let (dst_id, link_id);

        match direction {
            0 => {
                if cell_id % maze.w == 0 { continue ; }
                dst_id = cell_id - 1;
                link_id = cell_id - (cell_id / maze.w + 1);
            }
            1 => {
                if cell_id / maze.w == 0 { continue ;}
                dst_id = cell_id - maze.w;
                link_id = (maze.w * (maze.h - 1)) + cell_id - maze.w;
            }
            2 => {
                if cell_id % maze.w == maze.w - 1 { continue ; }
                dst_id = cell_id + 1;
                link_id = cell_id - (cell_id / maze.w);
            }
            _ => {
                if cell_id / maze.w == maze.h - 1 { continue ;}
                dst_id = cell_id + maze.w;
                link_id = (maze.w * (maze.h - 1)) + cell_id;
            }
        }

        if visited[dst_id] == false {
            maze.links[link_id] = true;
            carve_passage(maze, visited, dst_id);
        }
    }
}
