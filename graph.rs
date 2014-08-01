use std::num::{abs,FloatMath};

pub struct Point (pub f32, pub f32);

pub struct Node {
	pub id: int,
	pub point: Point,
}

impl Point {
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
	pub fn shiftPoint(mut self, dist: f32, angle: f32) {
		let Point(x,y) = self;
		self = Point(x + angle.cos() * dist, y + angle.sin() * dist);
	}
}

impl Node {
	fn new(id: int, pt: Point) -> Node {
		Node { id: id, point: pt }
	}

	fn distance(&self, node: Node) -> f32 {
		return dist(self.point, node.point);
	}
}

pub struct Graph {
	pub node_list: Vec<Node>,
	pub visited_list: Vec<int>,
	pub unvisited_list: Vec<int>,
	pub size: int,
}

impl Graph {
	pub fn new(vec: Vec<Point>) -> Graph {
		let mut list = Vec::new();
		let mut i = 0;
		for pts in vec.iter()
		{
			list.push(Node::new(i, *pts));
			i += 1;
		}

		Graph {node_list: list, visited_list: Vec::new(),
			unvisited_list: Vec::new(), size: i }
	}


	fn add(&mut self, pt:Point) {
		self.unvisited_list.push(self.size);

		self.node_list.push((Node::new(self.size,pt)));
		self.size += 1;
	}

	fn visit(&mut self, node: Node) {
		let mut i = 0;
		loop
		{
			if(self.unvisited_list.get(i) == &node.id)
			{
				self.unvisited_list.remove(i);
				break;
			}
			i += 1;
		}

		self.visited_list.push(i as int)
	}
}

pub fn dist(pt1:Point, pt2:Point) -> f32 {
	let Point(x1, y1) = pt1;
	let Point(x2, y2) = pt2;

	let x_dist = abs(x2-x1);
	let y_dist = abs(y2-y1);

	return (x_dist*x_dist + y_dist*y_dist).sqrt();
}

pub fn convert(pt: &str) -> f32 {
	let convert: Option<f32> = from_str(pt.as_slice().trim());

	let ret = match convert
	{
		Some(num) => num,
		_	=> fail!("improper input"),
	};

	return ret;
}

pub fn createPoint(x_coord: &str, y_coord: &str) -> Point {
	return Point(convert(x_coord),convert(y_coord));
}

pub fn getAngle(pt1: Point, pt2: Point) -> f32 {
	let Point(x1,y1) = pt1;
	let Point(x2,y2) = pt2;

	let ratio = abs(x2-x1) / dist(pt1, pt2);

	return ratio.asin();
}

fn main() {}