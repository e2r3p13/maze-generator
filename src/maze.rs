/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   maze.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 21:35:15 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/10 07:10:03 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

use image::{ImageBuffer, Rgb};

struct Transition {
	is_reachable: bool,
	destination: Option<u32>,
}

impl Transition {

	fn new(is_reachable: bool, destination: Option<u32>) -> Transition {
		Transition {
			is_reachable,
			destination,
		}
	}

}

struct Cell {
	left: Transition,
	right: Transition,
	top: Transition,
	bottom: Transition,
}

impl Cell {

	fn new(index: u32, w: u32, h: u32) -> Cell {

		Cell {
			left: Transition::new(false, if index % w > 0 {Some(index - 1)} else {None}),
			right: Transition::new(false, if index % w < w - 1 {Some(index + 1)} else {None}),
			top: Transition::new(false, if index / w > 0 {Some(index - w)} else {None}),
			bottom: Transition::new(false, if index / w < h - 1 {Some(index + w)} else {None}),
		}
	}

}

pub struct Maze {
	height: u32,
	width: u32,
	cells: Vec<Cell>,
}

impl Maze {

	// Creates a fully closed maze
	// (each is_reachable transition's attribute of each cell are set to false).
	fn new(h: u32, w: u32) -> Maze {
		let mut maze = Maze {
			height: h,
			width: w,
			cells: Vec::new(),
		};

		for i in 0 .. h * w {
			let cell = Cell::new(i, w, h);
			maze.cells.push(cell);
		}
		maze
	}

}

impl Maze {

	pub fn generate(h: u32, w: u32) -> Maze {

		let mut maze = Maze::new(h, w);

		let mut visited: Vec<bool> = vec![false; (w * h) as usize];

		let mut rng = rand::thread_rng();
		let cell_id = rng.gen_range(0..maze.cells.len());

		carve_passage(&mut maze, &mut visited, cell_id);

		return maze;
	}

	pub fn print(&self, free_char: char, wall_char: char) {

		let mut tmp: Vec<char> = Vec::new();

		for cell in &self.cells {
			tmp.push(free_char);
			tmp.push(if cell.right.is_reachable {free_char} else {wall_char});
			tmp.push(if cell.bottom.is_reachable {free_char} else {wall_char});
			tmp.push(wall_char);
		}


		let mut lines: Vec<String> = Vec::new();

		for _ in 0..self.height {
			let mut line1 = String::from(wall_char);
			let mut line2 = String::from(wall_char);
			for _ in 0 .. self.width {
				line1.push(tmp.remove(0));
				line1.push(tmp.remove(0));
				line2.push(tmp.remove(0));
				line2.push(tmp.remove(0));
			}
			lines.push(line1);
			lines.push(line2);
		}
		println!("{}", String::from(wall_char).repeat((self.width * 2 + 1) as usize));
		for line in lines.iter() {
			println!("{}", line);
		}
	}

	pub fn export(&self, filename: &str, ppc: u32, linewidth: u32) {

		if ppc < 5 || linewidth >= ppc / 2 {
			println!("Error: Resolution too low");
			return ;
		}
		let w = self.width * ppc;
		let h = self.height * ppc;

		let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(w, h);

		for cell in &self.cells {
			// TODO
		}

		image.save(filename).unwrap();
	}

}

fn carve_passage(maze: &mut Maze, visited: &mut Vec<bool>, cell_id: usize) {

	visited[cell_id] = true;

	let mut directions = [0, 1, 2, 3]; // TOP RIGHT BOTTOM LEFT

	let mut rng = thread_rng();
	directions.shuffle(&mut rng);

	for direction in directions.iter() {
		let transition: &mut Transition = match direction {
			0 => { &mut maze.cells[cell_id].top },
			1 => { &mut maze.cells[cell_id].right },
			2 => { &mut maze.cells[cell_id].bottom },
			_ => { &mut maze.cells[cell_id].left },
		};
		if let Some(id) = transition.destination {
			if visited[id as usize] == false {
				transition.is_reachable = true;
				match direction {
					0 => {maze.cells[id as usize].bottom.is_reachable = true},
					1 => {maze.cells[id as usize].left.is_reachable = true},
					2 => {maze.cells[id as usize].top.is_reachable = true},
					_ => {maze.cells[id as usize].right.is_reachable = true},
				}
				carve_passage(maze, visited, id as usize);
			}
		}
	}
}
