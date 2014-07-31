#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate collections;
use regex::Regex;
use std::vec::Vec;
use graph::{Point,Node,Graph, createPoint};
use regex::Regex;
use std::io::fs::File;
use std::io::BufferedReader;
use std::{os,rand};

mod graph;


/*-------------------------------------------------------------------------------------
 |	Purpose:	this is used to read the file provided in the command line
 |
 |	Pre-Cond:	the file exists, is a valid format, and that a command line file was 
 |				provided
 |
 |	Post-Cond:	the file is converted successfully into a Graph
 |
 |	Parameters:	n/a
 |
 |	Returns:	The graph representation of the provided text file
 |
 `-------------------------------------------------------------------------------------*/
fn readFile(file: &str) -> Graph {
	let mut vectors: Vec<Point> = Vec::new();
	let re = regex!(r"\(?P(<t1>-?\d+),(?P<t2>-?\d+)\)");

	for points in re.captures_iter(file)
	{
		let point: Point = createPoint(points.name("t1"), points.name("t2"));
		vectors.push(point);
	}

	let graph = Graph::new(vectors);
	return graph;
}

/*--------------------------------------------------------------------------------------
 |	Method:		euclidean
 |
 |	Purpose:	Finds a euclidean path, if possible
 |
 |	Pre-Cond:	
 |
 |	Post-Cond:
 |
 |	Parameters:
 |
 |	Returns:
 |
 `-------------------------------------------------------------------------------------*
fn euclidean(graph: Graph, node: int) -> Vec<Node> {
	//base case
	if(graph.unvisited_list.len() == 0)
	{
		return Vec::new();
	// case where a path can't be found
	} else if(graph.visited_list.len() == graph.node_list.len()) 
	{
		return null();
	// recursive case
	} else 
	{
		let mut i, j = 0, 0;

		while(i != graph.node_list.len())
		{
			let visit_node = graph.node_list.get(i);
			if(!graph.visited_list.contains(visit_node.nodeID))
			{
				graph.visit(visit_node.nodeID);
				loop 
				{
					let vec = euclidean(graph, visit_node.connected.pop());
				
					if(vec == null())
					{
						if(visit_node.connected.len() == 0)
						{
							return null();
						}

					} else {
						let vec = euclidean(graph, visit_node.nodeID);
						if(vec == null())
						{
							return null();
						}
						return vec.push(visit_node.nodeID);
					}
				}
				
			}
			i++;
		}
	}
}
*/

/*--------------------------------------------------------------------------------------
 |	Method:		spreadGraph
 |
 |	Purpose:	attempting to create a random spread of points
 |
 |	Pre-Cond:	the graph is initialized
 |
 |	Post-Cond:	no side effects, I hope.
 |
 |	Parameters:	graph:	the graph that is being spread
 |
 |	Returns:	Vec<Point>:	a vector of the points, post spread
 |
 `-------------------------------------------------------------------------------------*/

fn spreadGraph(graph: Graph) -> Vec<Point>{
	let points: Vec<Point>;
	let mut max_x: f32 = -0xFFFFFFFF;
	let mut min_x: f32 = 0xFFFFFFFF;
	let mut max_y: f32 = -0xFFFFFFFF;
	let mut min_y: f32 = 0xFFFFFFFF;

	for node in graph.node_list.iter()
	{
		let Point(x,y) = node;
		points.push(node);

		if x > max_x
		{
			max_x = x;
		} else if x < min_x
		{
			min_x = x;
		}

		if y > max_y
		{
			max_y = y;
		} else if y < min_y
		{
			min_y = y;
		}
	}

	let max_radius = SOMETHING;
	for point1 in points.iter()
	{
		let radius = rand::random() * max_radius;
		for point2 in points.iter() 
		{
			let dist = point1.distance(point2);
			if dist
			{

				shiftPoint(point2, dist, getAngle(point1, point2));
			}
		}
	}
}

 /*--------------------------------------------------------------------------------------
 |	Method:		
 |
 |	Purpose:	
 |
 |	Pre-Cond:	
 |
 |	Post-Cond:
 |
 |	Parameters:
 |
 |	Returns:
 |
 `-------------------------------------------------------------------------------------*/
fn shiftPoint(pt: Point, dist: f32) {

}

 /*--------------------------------------------------------------------------------------
 |	Method:		
 |
 |	Purpose:	
 |
 |	Pre-Cond:	
 |
 |	Post-Cond:
 |
 |	Parameters:
 |
 |	Returns:
 |
 `-------------------------------------------------------------------------------------*/
fn main() {
	let arg = os::args();

	if arg.len() != 2
	{
		fail!("Enter a path to load points");
	}
	let fi = arg.get(1).as_slice();
	let path = Path::new(fi);
	let file = match File::open(&path)
	{
		Err(why) => fail!("couldn't open {}: {}", why, why.desc),
		Ok(file) => file,
	};

	let mut file_str = BufferedReader::new(file);
}