/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 23:06:40 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/13 10:47:29 by bccyv            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod maze;
use maze::Maze;

fn main() {
	let width = 10;
	let height = 10;

	let maze = Maze::generate(height, width, maze::Generator::Backtrack);

	// maze.print_blockwise(' ', '█');
	maze.print_linewise();
}
