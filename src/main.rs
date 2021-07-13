/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 23:06:40 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/13 09:48:40 by bccyv            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod maze;
use maze::Maze;

fn main() {
	let width = 88;
	let height = 1000;

	let maze = Maze::generate(height, width);

	// maze.print_blockwise(' ', '█');
	maze.print_linewise();
}
