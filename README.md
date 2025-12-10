Here are my solutions for AoC 2025 in Rust. 

I'm back to challenge myself with Rust again. 

Since there are only 15 puzzles this year, I hope it will be a bit more relaxing.

## day02

I attempted to optimize the solution to meet timing constraints, but I realized the brute-force method would have been significantly simpler to implement. Misjudge! Totally overthought.

## day08

Really? How about UnionFind, that's how ai works.

## day09

I spent a long time trying to figure out how to determine if a rectangle is inside a polygon. I tried many methods, but they were either incorrect or simply inefficient.

Thanks to Reddit, I found the best approach so far:

Given the problem's constraints that edges are vertical or horizontal and the rectangle's vertices align with the edges—we can use a specific logic: if a rectangle is not totally inside the polygon, it must contain at least one complete polygon edge.

Therefore, we can simply solve for the largest rectangle area that contains no edges. This approach is much easier to implement.