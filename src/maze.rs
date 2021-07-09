/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   maze.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 21:35:15 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/09 22:30:19 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct Maze {
	height: u32,
	width: u32,
	data: String,
}

impl Maze {

	pub fn generate(h: u32, w: u32, wall_chr: char, blank_chr: char) -> Maze {
		let maze = Maze {
			height: h,
			width: w,
			data: String::new(),
		};
		maze
	}

	pub fn print(&self) {

	}

}
