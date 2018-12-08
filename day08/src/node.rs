use nom::*;
use nom::types::CompleteStr;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    pub fn parse(input: &str) -> Node {
        parse_node(CompleteStr(input))
            .expect("Could not parse Node")
            .1
    }

    pub fn metadata_sum(&self) -> i32 {
        let mut sum = 0;
        
        sum += self.metadata.iter()
            .sum::<i32>();
        
        sum += self.children.iter()
            .map(|node| node.metadata_sum())
            .sum::<i32>();

        sum
    }

    pub fn value(&self) -> i32 {
        if self.children.is_empty() {
            return self.metadata_sum();
        }

        let mut value = 0;

        for entry in &self.metadata {
            let index = (*entry - 1) as usize;

            if let Some(child) = self.children.get(index) {
                value += child.value()
            }
        }
        value
    }
}

named!(parse_i32<CompleteStr, i32>,
    do_parse!(
        opt!(tag!(" "))                     >>
        int: take_while!(char::is_numeric)  >>

        (int.parse::<i32>().expect("Could not parse i32"))
    )
);

named!(parse_usize<CompleteStr, usize>,
    do_parse!(
        opt!(tag!(" "))                     >>
        int: take_while!(char::is_numeric)  >>

        (int.parse::<usize>().expect("Could not parse i32"))
    )
);

named!(parse_node<CompleteStr, Node>,
    do_parse!(
        amt_children: parse_usize                   >>
        amt_metadata: parse_usize                   >>

        children: count!(parse_node, amt_children)  >>
        metadata: count!(parse_i32, amt_metadata)   >>

        (Node { children, metadata })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_only() {
        let node_a = Node::parse("0 1 1");

        assert_eq!(node_a.children.len(), 0);
        assert_eq!(node_a.metadata.len(), 1);
        assert_eq!(node_a.metadata, vec![1]);

        let node_b = Node::parse("0 3 50 25 100");

        assert_eq!(node_b.children.len(), 0);
        assert_eq!(node_b.metadata.len(), 3);
        assert_eq!(node_b.metadata, vec![50, 25, 100]);
    }

    #[test]
    fn test_parse_nested() {
        let node_a = Node::parse("1 1 0 1 50 100");

        assert_eq!(node_a.children.len(), 1);
        assert_eq!(node_a.metadata, vec![100]);

        let node_b = node_a.children.get(0).unwrap();

        assert_eq!(node_b.children.len(), 0);
        assert_eq!(node_b.metadata, vec![50]);
    }

    #[test]
    fn test_parse_example() {
        let node_a = Node::parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");

        assert_eq!(node_a.children.len(), 2);
        assert_eq!(node_a.metadata, vec![1, 1, 2]);

        let node_b = node_a.children.get(0).unwrap();

        assert_eq!(node_b.children.len(), 0);
        assert_eq!(node_b.metadata, vec![10, 11, 12]);

        let node_c = node_a.children.get(1).unwrap();

        assert_eq!(node_c.children.len(), 1);
        assert_eq!(node_c.metadata, vec![2]);

        let node_d = node_c.children.get(0).unwrap();

        assert_eq!(node_d.children.len(), 0);
        assert_eq!(node_d.metadata, vec![99]);
    }

    #[test]
    fn test_metadata_sum() {
        let node_a = Node {
            children: vec![
                Node { children: vec![], metadata: vec![100, 50] }
            ],
            metadata: vec![200]
        };

        assert_eq!(node_a.metadata_sum(), 350);
    }

    #[test]
    fn test_value() {
        let node_b = Node { children: vec![], metadata: vec![10, 11, 12] };
        let node_d = Node { children: vec![], metadata: vec![99] };

        assert_eq!(node_b.value(), 33);
        assert_eq!(node_d.value(), 99);

        let node_c = Node { children: vec![node_d], metadata: vec![2] };

        assert_eq!(node_c.value(), 0);

        let node_a = Node { children: vec![node_b, node_c], metadata: vec![1, 1, 2] };

        assert_eq!(node_a.value(), 66);

    }
}
