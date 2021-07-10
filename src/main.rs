/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 23:06:40 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/10 06:19:32 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use std::env;

// fn main() {
//
// 	let args: Vec<String>;
// 	let width: u32;
// 	let height: u32;
//
// 	args = env::args().collect();
//     if args.len() != 3 {
// 		println!("Usage: ./mazegen height width");
// 		return ();
// 	}
// 	if let Ok(h) = args[1].parse() {
// 		height = h;
// 	} else {
// 		height = 32;
// 		println!("Error: '{}' is not a valid height", args[1]);
// 	}
// 	if let Ok(w) = args[2].parse() {
// 		width = w;
// 	} else {
// 		width = 32;
// 		println!("Error: '{}' is not a valid width", args[2]);
// 	}
//
// 	println!("{} {}", height, width);
// 	return ();
// }

mod maze;
use maze::Maze;

fn main() {
	let width = 64;
	let height = 64;

	let maze = Maze::generate(height, width);

	maze.print(' ', '█');
}
