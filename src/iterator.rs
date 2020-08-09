
use crate::prelude::*;
use crate::searcher_mod::Merge;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Node{
    pub next : Option<usize>,
    pub prev : Option<usize>,
    pub alive : bool,
}

impl Node{
    pub fn new() -> Self{
        Node{
            prev : None,
            next : None,
            alive : true,
        }
    }

    pub fn from(prev : Option<usize>, next : Option<usize>) -> Self{
        Node{
            prev : prev,
            next : next,
            alive : true,
        }
    }

    pub fn list_with_length(len : usize) -> Vec<Self>{
        let mut vec_of_nodes : Vec<Self> = vec![Self::new(); len];

        match len{
            0 | 1 => {return vec_of_nodes;},
            _ => {
                vec_of_nodes[0].next = Some(1);
                vec_of_nodes[len - 1].prev = Some(len - 2);

                for idx in 1..=len-2{
                    vec_of_nodes[idx].prev = Some(idx - 1);
                    vec_of_nodes[idx].next = Some(idx + 1);
                }

                return vec_of_nodes;
            }
        }
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
pub struct LinkedList<T>{
    pub links : Vec<Node>,
    pub contents : Vec<T>,
    pub head : Option<usize>,
    pub tail : Option<usize>,
    pub current : [Option<usize>; 3],
}

impl<T> LinkedList<T>{
    #[allow(dead_code)]
    pub fn new() -> Self{
        Self{
            links : Vec::<Node>::new(),
            contents : Vec::<T>::new(),
            head : None,
            tail : None,
            current : [None; 3],
        }
    }

    #[allow(dead_code)]
    pub fn from(contents : Vec<T>) -> Self{
        let len : usize = contents.len();
        let head : Option<usize>;
        let tail : Option<usize>;
        match len{
            0 => {
                head = None;
                tail = None;
            },
            _ => {
                head = Some(0);
                tail = Some(len - 1);
            }
        }

        Self{
            links : Node::list_with_length(len),
            contents : contents,
            head : head,
            tail : tail,
            current : [None; 3],
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, content : T){
        let len : usize = self.links.len();
        match self.tail{
            Some(idx) => {
                self.links[idx].next = Some(len);
                self.links.push(Node::from(Some(idx), None));
                self.tail = Some(len);
            },
            None => {
                self.head = Some(len);
                self.tail = Some(len);
                self.links.push(Node::from(None, None));
            },
        }
        self.contents.push(content);
    }

    #[allow(dead_code)]
    pub fn del(&mut self, idx : usize) -> Result<(), Error>{
        if self.links.len() <= idx || self.links[idx].alive == false{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }

        self.links[idx].alive = false;
        let node = self.links[idx];


        match node.prev{
            Some(prev) => {self.links[prev].next = node.next;},
            None => {self.head = node.next;},
        }

        match node.next{
            Some(next) => {self.links[next].prev = node.prev;},
            None => {self.tail = node.prev;},
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn connect_all(&mut self) -> Result<(), Error>{
        let len : usize = self.contents.len();
        match len{
            0 => {
                self.head = None;
                self.tail = None;
                return Ok(());
            }
            1 => {
                self.head = Some(0);
                self.tail = Some(0);
                self.links[0].prev = None;
                self.links[0].next = None;
                return Ok(());
            }
            length =>{
                self.head = Some(0);
                self.tail = Some(length - 1);

                self.links[0].prev = None;
                self.links[0].next = Some(1);

                self.links[length - 1].prev = Some(0);
                self.links[length - 1].next = None;

                for i in 1..=length-2 {
                    self.links[i].prev = Some(i - 1);
                    self.links[i].next = Some(i + 1);
                }
                return Ok(());
            }
        }
    }

    #[allow(dead_code)]
    pub fn into_iter(&mut self){
        self.current[0] = self.head;
    }

    pub fn into_double_iter(&mut self){
        self.current[0] = self.head;
        self.current[1] = self.head.map_or(None, |x| self.links[x].next);
    }

    #[allow(dead_code)]
    pub fn get(&mut self) -> Option<&T>{
        match self.current[0]{
            None => None,
            Some(idx) => {
                self.current[0] = self.links[idx].next;
                Some(&self.contents[idx])
            },
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self) -> Option<&mut T>{
        match self.current[0]{
            None => None,
            Some(idx) => {
                self.current[0] = self.links[idx].next;
                Some(&mut self.contents[idx])
            },
        }
    }

    #[allow(dead_code)]
    pub fn enumerate(&mut self) -> Option<(usize, &T)>{
        match self.current[0]{
            None => None,
            Some(idx) => {
                self.current[0] = self.links[idx].next;
                Some((idx, &self.contents[idx]))
            },
        }
    }

    #[allow(dead_code)]
    pub fn enumerate_mut(&mut self) -> Option<(usize, &mut T)>{
        match self.current[0]{
            None => None,
            Some(idx) => {
                self.current[0] = self.links[idx].next;
                Some((idx, &mut self.contents[idx]))
            },
        }
    }

    #[allow(dead_code)]
    pub fn indicies(&mut self) -> Option<usize>{
        match self.current[0]{
            None => None,
            Some(idx) => {
                self.current[0] = self.links[idx].next;
                Some(idx)
            },
        }
    }

    #[allow(dead_code)]
    pub fn get_double(&mut self) -> Option<(&T, &T)>{
        let idx2 : usize;
        match self.current[1]{
            None => {
                self.current[0] = self.links[self.current[0]?].next;
                idx2 = self.links[self.current[0]?].next?;
                self.current[1] = self.links[idx2].next;
            },
            Some(idx) => {
                self.current[1] = self.links[idx].next;
                idx2 = idx;
            },
        }
        let idx1 = self.current[0]?;

        return Some((&self.contents[idx1], &self.contents[idx2]));
    }

    #[allow(dead_code)]
    pub fn enumerate_double(&mut self) -> Option<(usize, &T, usize, &T)>{
        let idx2 : usize;
        match self.current[1]{
            None => {
                self.current[0] = self.links[self.current[0]?].next;
                idx2 = self.links[self.current[0]?].next?;
                self.current[1] = self.links[idx2].next;
            },
            Some(idx) => {
                self.current[1] = self.links[idx].next;
                idx2 = idx;
            },
        }
        let idx1 = self.current[0]?;

        return Some((idx1, &self.contents[idx1], idx2, &self.contents[idx2]));
    }

    #[allow(dead_code)]
    pub fn indicies_double(&mut self) -> Option<(usize, usize)>{
        let idx2 : usize;
        match self.current[1]{
            None => {
                self.current[0] = self.links[self.current[0]?].next;
                idx2 = self.links[self.current[0]?].next?;
                self.current[1] = self.links[idx2].next;
            },
            Some(idx) => {
                self.current[1] = self.links[idx].next;
                idx2 = idx;
            },
        }
        let idx1 = self.current[0]?;

        return Some((idx1, idx2));
    }
}

impl<M : Merge> LinkedList<M>{
    #[allow(dead_code)]
    pub fn merge(&mut self, idx1 : usize, idx2 : usize) -> Result<(), Error>{
        let size : usize = self.contents[idx2].size();
        self.contents[idx1].add_size(size)?;
        self.del(idx2)?;
        Ok(())
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_new_vec(){
        const LENGTH : usize = 5;
        let vec_of_nodes : Vec<Node> = Node::list_with_length(LENGTH);

        for idx in 0..LENGTH{
            match idx {
                0   => {assert_eq!(vec_of_nodes[idx], Node::from(None, Some(idx + 1)));},
                4   => {assert_eq!(vec_of_nodes[idx], Node::from(Some(idx - 1), None));},
                _   => {assert_eq!(vec_of_nodes[idx], Node::from(Some(idx - 1), Some(idx + 1)));},
            }
        }

        let linkedlist : LinkedList<usize> = LinkedList::new();
        assert_eq!(linkedlist.head, None);
        assert_eq!(linkedlist.tail, None);
        assert_eq!(linkedlist.links, Vec::<Node>::new());
        assert_eq!(linkedlist.contents, Vec::<usize>::new());
    }

    #[test]
    fn test_list_with_0_length(){
        assert_eq!(Node::list_with_length(0), Vec::<Node>::new());
        assert_eq!(Node::list_with_length(1), vec![Node::new()]);
    }

    #[test]
    fn test_add_del() -> Result<(), Error>{
        let mut linkedlist = LinkedList::<usize>::new();

        linkedlist.push(3);
        linkedlist.push(5);
        linkedlist.push(1);

        assert_eq!(linkedlist.head, Some(0));
        assert_eq!(linkedlist.tail, Some(2));
        assert_eq!(linkedlist.contents, vec![3, 5, 1]);
        assert_eq!(linkedlist.links[0], Node::from(None, Some(1)));
        assert_eq!(linkedlist.links[1], Node::from(Some(0), Some(2)));
        assert_eq!(linkedlist.links[2], Node::from(Some(1), None));

        linkedlist.del(1)?;
        assert_eq!(linkedlist.head, Some(0));
        assert_eq!(linkedlist.tail, Some(2));
        assert_eq!(linkedlist.contents, vec![3, 5, 1]);
        assert_eq!(linkedlist.links[0], Node::from(None, Some(2)));
        assert_eq!(linkedlist.links[1], Node{prev : Some(0), next : Some(2), alive: false});
        assert_eq!(linkedlist.links[2], Node::from(Some(0), None));

        linkedlist.del(2)?;
        assert_eq!(linkedlist.head, Some(0));
        assert_eq!(linkedlist.tail, Some(0));
        assert_eq!(linkedlist.contents, vec![3, 5, 1]);
        assert_eq!(linkedlist.links[0], Node::from(None, None));
        assert_eq!(linkedlist.links[1], Node{prev : Some(0), next : Some(2), alive: false});
        assert_eq!(linkedlist.links[2], Node{prev : Some(0), next : None, alive: false});

        linkedlist.push(4);
        assert_eq!(linkedlist.head, Some(0));
        assert_eq!(linkedlist.tail, Some(3));
        assert_eq!(linkedlist.contents, vec![3, 5, 1, 4]);
        assert_eq!(linkedlist.links[0], Node::from(None, Some(3)));
        assert_eq!(linkedlist.links[1], Node{prev : Some(0), next : Some(2), alive: false});
        assert_eq!(linkedlist.links[2], Node{prev : Some(0), next : None, alive: false});
        assert_eq!(linkedlist.links[3], Node::from(Some(0), None));

        linkedlist.del(0)?;
        assert_eq!(linkedlist.head, Some(3));
        assert_eq!(linkedlist.tail, Some(3));
        assert_eq!(linkedlist.contents, vec![3, 5, 1, 4]);
        assert_eq!(linkedlist.links[0], Node{prev : None, next : Some(3), alive: false});
        assert_eq!(linkedlist.links[1], Node{prev : Some(0), next : Some(2), alive: false});
        assert_eq!(linkedlist.links[2], Node{prev : Some(0), next : None, alive: false});
        assert_eq!(linkedlist.links[3], Node::from(None, None));

        Ok(())
    }

    #[test]
    fn test_mutable_reference(){
        let mut linkedlist = LinkedList::<usize>::new();

        linkedlist.push(3);
        linkedlist.push(5);
        linkedlist.push(1);

        for x in &mut linkedlist.contents{
            *x = 4;
        }
        assert_eq!(linkedlist.contents, vec![4, 4, 4]);
    }

    #[test]
    fn test_get() -> Result<(), Error>{
        let mut linkedlist = LinkedList::<usize>::new();

        linkedlist.push(3);
        linkedlist.push(5);
        linkedlist.push(1);

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get(){
            res.push_str(format!("{} ", *content).as_str());
        }
        assert_eq!(res, "3 5 1 ");

        linkedlist.del(1)?;
        linkedlist.push(4);

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get(){
            res.push_str(format!("{} ", *content).as_str());
        }
        assert_eq!(res, "3 1 4 ");

        return Ok(());
    }

    #[test]
    fn test_get_mut() -> Result<(), Error>{
        let mut linkedlist = LinkedList::<usize>::new();

        linkedlist.push(3);
        linkedlist.push(5);
        linkedlist.push(1);

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get_mut(){
            res.push_str(format!("{} ", *content).as_str());
            *content = 4;
        }
        assert_eq!(res, "3 5 1 ");

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get(){
            res.push_str(format!("{} ", *content).as_str());
        }
        assert_eq!(res, "4 4 4 ");

        return Ok(());
    }

    #[test]
    fn test_complex_type() -> Result<(), Error>{
        #[derive(Copy, Clone, Debug)]
        enum TestEnum{
            Var1,
            Var2,
        }

        #[derive(Clone, Debug)]
        struct TestType{
            x : f64,
            n : usize,
            vec : Vec<f64>,
            enu : TestEnum,
        }

        let tt1 = TestType{
            x : 3f64,
            n : 1usize,
            vec : vec![1f64, 2f64, 3f64],
            enu : TestEnum::Var1,
        };
        let tt2 = TestType{
            x : 2f64,
            n : 2usize,
            vec : vec![1f64, 3f64, 5f64],
            enu : TestEnum::Var2,
        };

        let mut linkedlist = LinkedList::<TestType>::new();
        linkedlist.push(tt1);
        linkedlist.push(tt2);

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get_mut(){
            res.push_str(format!("{:?}\n", *content).as_str());
            content.vec[0] = 10f64;
        }
        assert_eq!(res,
            "TestType { x: 3.0, n: 1, vec: [1.0, 2.0, 3.0], enu: Var1 }
TestType { x: 2.0, n: 2, vec: [1.0, 3.0, 5.0], enu: Var2 }\n");


        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(content) = linkedlist.get(){
            res.push_str(format!("{:?}\n", *content).as_str());
        }
        assert_eq!(res,
            "TestType { x: 3.0, n: 1, vec: [10.0, 2.0, 3.0], enu: Var1 }
TestType { x: 2.0, n: 2, vec: [10.0, 3.0, 5.0], enu: Var2 }\n");

        Ok(())
    }

    #[test]
    fn test_from(){
        #[derive(Copy, Clone, Debug)]
        enum TestEnum{
            Var1,
            Var2,
        }

        #[derive(Clone, Debug)]
        struct TestType{
            x : f64,
            n : usize,
            vec : Vec<f64>,
            enu : TestEnum,
        }

        let tt1 = TestType{
            x : 3f64,
            n : 1usize,
            vec : vec![1f64, 2f64, 3f64],
            enu : TestEnum::Var1,
        };
        let tt2 = TestType{
            x : 2f64,
            n : 2usize,
            vec : vec![1f64, 3f64, 5f64],
            enu : TestEnum::Var2,
        };
        let vec : Vec<TestType> = vec![tt1, tt2];
        let mut linkedlist = LinkedList::from(vec);

        linkedlist.into_iter();
        let mut res = String::new();
        while let Some(tt) = linkedlist.get(){
            res.push_str(format!("{:?}\n", *tt).as_str());
        }
        assert_eq!(res,
            "TestType { x: 3.0, n: 1, vec: [1.0, 2.0, 3.0], enu: Var1 }
TestType { x: 2.0, n: 2, vec: [1.0, 3.0, 5.0], enu: Var2 }\n");
    }

    #[test]
    fn test_distance() -> Result<(), Error>{
        use crate::searcher_mod::cont_passive_indep::ContPassiveIndepSearcher;

        // Empty list
        let mut linkedlist = LinkedList::<ContPassiveIndepSearcher>::from(Vec::new());
        linkedlist.into_double_iter();
        let mut res = String::new();
        while let Some((searcher1, searcher2)) = linkedlist.get_double(){
            let d : f64 = searcher1.pos().distance(searcher2.pos())?;
            res.push_str(format!("{} ", d).as_str());
        }
        assert_eq!(res, String::new());


        let s1 = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64), Position::<f64>::new(vec![0.0, 0.0]));
        let s2 = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64), Position::<f64>::new(vec![3.0, 0.0]));
        let s3 = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64), Position::<f64>::new(vec![0.0, 4.0]));


        // Single ptl list
        let mut linkedlist = LinkedList::<ContPassiveIndepSearcher>::from(vec![s1.clone()]);
        let mut res = String::new();
        while let Some((searcher1, searcher2)) = linkedlist.get_double(){
            let d : f64 = searcher1.pos().distance(searcher2.pos())?;
            res.push_str(format!("{} ", d).as_str());
        }
        assert_eq!(res, String::new());

        // More
        let mut linkedlist = LinkedList::<ContPassiveIndepSearcher>::from(vec![s1, s2, s3]);
        linkedlist.into_double_iter();
        let mut res = String::new();
        while let Some((searcher1, searcher2)) = linkedlist.get_double(){
            let d : f64 = searcher1.pos().distance(searcher2.pos())?;
            res.push_str(format!("{} ", d).as_str());
        }
        assert_eq!(res, "3 4 5 ");
        Ok(())
    }
}

