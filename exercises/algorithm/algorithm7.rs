/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

use std::collections::VecDeque;

struct BracketMatcher {
    stack: VecDeque<char>,
}

impl BracketMatcher {
    // 创建一个新的 BracketMatcher 实例
    fn new() -> Self {
        BracketMatcher {
            stack: VecDeque::new(),
        }
    }

    // 检查给定的字符串中的括号是否匹配
    fn is_matched(&mut self, expr: &str) -> bool {
        for ch in expr.chars() {
            match ch {
                '(' | '[' | '{' => self.stack.push_back(ch),
                ')' => {
                    if self.stack.is_empty() || self.stack.pop_back() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if self.stack.is_empty() || self.stack.pop_back() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if self.stack.is_empty() || self.stack.pop_back() != Some('{') {
                        return false;
                    }
                }
                _ => {} // 忽略非括号字符
            }
        }
        self.stack.is_empty()
    }
}

// 提供一个全局函数 `bracket_match` 供测试使用
fn bracket_match(expr: &str) -> bool {
    let mut matcher = BracketMatcher::new();
    matcher.is_matched(expr)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}