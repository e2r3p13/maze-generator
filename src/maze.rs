/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   maze.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 21:35:15 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/12 19:32:09 by bccyv            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

pub struct Maze {
	height: u32,
	width: u32,
	cells: Vec<Cell>,
}

struct Cell {
	left: Link,
	right: Link,
	top: Link,
	bottom: Link,
}

struct Link {
	is_connected: bool,
	destination: Option<u32>,
}

impl Cell {
	fn new(index: u32, w: u32, h: u32) -> Cell {
		Cell {
			left: Link::new(false, if index % w > 0 {Some(index - 1)} else {None}),
			right: Link::new(false, if index % w < w - 1 {Some(index + 1)} else {None}),
			top: Link::new(false, if index / w > 0 {Some(index - w)} else {None}),
			bottom: Link::new(false, if index / w < h - 1 {Some(index + w)} else {None}),
		}
	}

	fn top_left_wall(&self, maze: &Maze) -> char {
		let left: bool;
		let top: bool;
		let top_left: bool;
		let left_top: bool;

		left = self.left.is_connected;
		top = self.top.is_connected;
		top_left = match self.top.destination {
			Some(id) => {
				maze.cells[id as usize].left.is_connected
			},
			None => true,
		};
		left_top = match self.left.destination {
			Some(id) => {
				maze.cells[id as usize].top.is_connected
			},
			None => true,
		};
		return if left && top_left { '━' }
		else if top && left_top { '┃' }
		else if left && top { '┛' }
		else if top && top_left { '┓' }
		else if left && left_top { '┗' }
		else if top_left && left_top { '┏' }
		else if left { '┻' }
		else if top { '┫' }
		else if left_top  { '┣' }
		else if top_left { '┳' }
		else { '╋' };
	}

}

impl Link {
	fn new(is_connected: bool, destination: Option<u32>) -> Link {
		Link {
			is_connected,
			destination,
		}
	}
}

impl Maze {
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

	pub fn print_linewise(&self) {

		let mut lines: Vec<String> = Vec::new();

		let mut line = String::new();

		for i in 0..self.cells.len() {

			let cell = &self.cells[i];

			line.push(cell.top_left_wall(&self));
			line.push(if cell.top.is_connected {' '} else {'━'});

			if  i > 0 && i as u32 % self.width == self.width - 1 {
				line.push( if i as u32 / self.width == 0 { '┓' } else if line.chars().last() == Some('━') {'┫'} else { '┃' });
				lines.push(line);
				line = String::new();
			}

		}

		line = String::from("┗━");
		for i in 1 .. self.width {
			if let Some(id) = self.cells[((self.height - 2) * self.width + i) as usize].bottom.destination {
				if self.cells[id as usize].left.is_connected {
					line.push('━');
				} else {
					line.push('┻');
				}
			}
			// line.push(if self.cells[((self.height - 1) * self.width + i) as usize].bottom.destination { '━' } else { '┻' });
			line.push('━');
		}
		line.push('┛');
		lines.push(line);

		for line in lines.iter() {
			println!("{}", line);
		}

	}

	pub fn print_blockwise(&self, free_char: char, wall_char: char) {

		let mut tmp: Vec<char> = Vec::new();

		for cell in &self.cells {
			tmp.push(free_char);
			tmp.push(if cell.right.is_connected {free_char} else {wall_char});
			tmp.push(if cell.bottom.is_connected {free_char} else {wall_char});
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

	// TODO
	// pub fn export(&self, filename: &str, ppc: u32, linewidth: u32) {
	//
	// 	if ppc < 5 || linewidth >= ppc / 2 {
	// 		println!("Error: Resolution too low");
	// 		return ;
	// 	}
	// 	let w = self.width * ppc;
	// 	let h = self.height * ppc;
	//
	// 	let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(w, h);
	//
	// 	for cell in &self.cells {
	// 	}
	//
	// 	image.save(filename).unwrap();
	// }

}

fn carve_passage(maze: &mut Maze, visited: &mut Vec<bool>, cell_id: usize) {

	visited[cell_id] = true;

	let mut directions = [0, 1, 2, 3]; // TOP RIGHT BOTTOM LEFT

	let mut rng = thread_rng();
	directions.shuffle(&mut rng);

	for direction in directions.iter() {
		let Link: &mut Link = match direction {
			0 => { &mut maze.cells[cell_id].top },
			1 => { &mut maze.cells[cell_id].right },
			2 => { &mut maze.cells[cell_id].bottom },
			_ => { &mut maze.cells[cell_id].left },
		};
		if let Some(id) = Link.destination {
			if visited[id as usize] == false {
				Link.is_connected = true;
				match direction {
					0 => {maze.cells[id as usize].bottom.is_connected = true},
					1 => {maze.cells[id as usize].left.is_connected = true},
					2 => {maze.cells[id as usize].top.is_connected = true},
					_ => {maze.cells[id as usize].right.is_connected = true},
				}
				carve_passage(maze, visited, id as usize);
			}
		}
	}
}
