mod binary_tree;

use std::{cell::RefCell, rc::Rc, vec};

use binary_tree::TreeNode;

// 1332. https://leetcode.com/problems/remove-palindromic-subsequences/
pub fn remove_palindrome_sub(s: String) -> i32 {
    let reverse: String = s.chars().rev().collect();

    match s == reverse {
        true => 1,
        _ => 2,
    }
}

// 94. https://leetcode.com/problems/binary-tree-inorder-traversal/
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    let mut stack = vec![];
    let mut new_root = root;

    while !stack.is_empty() || new_root.is_some() {
        while let Some(node) = new_root {
            new_root = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            result.push(node.borrow().val);
            new_root = node.borrow_mut().right.take();
        }
    }

    result
}

// 144. https://leetcode.com/problems/binary-tree-preorder-traversal/
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    let mut stack = vec![];
    let mut new_root = root;

    while !stack.is_empty() || new_root.is_some() {
        while let Some(node) = new_root {
            new_root = node.borrow_mut().left.take();
            result.push(node.borrow_mut().val);
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            new_root = node.borrow_mut().right.take();
        }
    }

    result
}

// 145. https://leetcode.com/problems/binary-tree-postorder-traversal/
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    let mut stack = vec![];
    let mut new_root = root;

    while !stack.is_empty() || new_root.is_some() {
        while let Some(node) = new_root {
            new_root = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            if node.borrow().right.is_some() {
                new_root = node.borrow_mut().right.take();
                stack.push(node);
            } else {
                result.push(node.borrow().val);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_1332_works() {
        let mut result = remove_palindrome_sub("ababa".to_owned());
        assert_eq!(result, 1);

        result = remove_palindrome_sub("abb".to_owned());
        assert_eq!(result, 2);

        result = remove_palindrome_sub("baabb".to_owned());
        assert_eq!(result, 2);
    }
}
