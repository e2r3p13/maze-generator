/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 23:06:40 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/10 06:59:50 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod maze;
use maze::Maze;

fn main() {
	let width = 64;
	let height = 64;

	let maze = Maze::generate(height, width);

	maze.print(' ', '█');
	maze.export("mymaze.png", 32, 3);
}
