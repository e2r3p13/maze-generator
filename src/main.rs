/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/09 23:06:40 by lfalkau           #+#    #+#             */
/*   Updated: 2021/07/10 17:25:17 by bccyv            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod maze;
use maze::Maze;

fn main() {
	let width = 16;
	let height = 16;

	let maze = Maze::generate(height, width);

	maze.print_blockwise(' ', '█');
	maze.print_linewise();
}
