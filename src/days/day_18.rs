use super::Day;

pub struct Day18 {
    input: Vec<String>,
}

impl Day18 {
    pub fn load(input: &str) -> Self {
        let input = input.lines().map(|e| e.to_string()).collect::<Vec<_>>();

        Self { input }
    }

    fn solve<T>(expression: &String) -> usize
    where
        T: OpPriority,
    {
        let (val, _) = Self::solve_rec::<T>(expression, 0);
        val
    }

    fn solve_rec<T>(expression: &String, index: usize) -> (usize, usize)
    where
        T: OpPriority,
    {
        let mut stack = vec![Parse::Open];
        let mut delta = 0;
        while index + delta < expression.len() {
            let token = expression.as_bytes()[index + delta] as char;
            delta += 1;

            match token {
                '(' => {
                    let (val, len) = Self::solve_rec::<T>(expression, index + delta);
                    Self::parse_expr(&mut stack, Expr::Num(val));
                    delta += len + 1;
                }

                ')' => {
                    break;
                }

                '+' => {
                    Self::parse_op::<T>(&mut stack, Op::Add);
                }

                '*' => {
                    Self::parse_op::<T>(&mut stack, Op::Mul);
                }

                n if n.is_numeric() => {
                    let expr = Expr::Num(n.to_digit(10).unwrap() as usize);
                    Self::parse_expr(&mut stack, expr);
                }

                _ => {}
            }
        }

        Self::parse_close(&mut stack);

        match stack.pop().unwrap() {
            Parse::Done(expr) => (expr.value(), delta - 1),
            _ => panic!("aaaah!"),
        }
    }

    fn parse_close(stack: &mut Vec<Parse>) {
        let mut expr = match stack.pop().unwrap() {
            Parse::Done(expr) => expr,
            _ => panic!("aaaah!"),
        };

        while let Parse::Partial(op, left) = stack.pop().unwrap() {
            expr = Expr::Op(op, Box::new(left), Box::new(expr));
        }

        stack.push(Parse::Done(expr));
    }

    fn parse_op<T>(stack: &mut Vec<Parse>, op: Op)
    where
        T: OpPriority,
    {
        let parse = match stack.pop().unwrap() {
            Parse::Done(Expr::Op(prev, left, right)) if T::priority(&op) > T::priority(&prev) => {
                stack.push(Parse::Partial(prev, *left));
                Parse::Partial(op, *right)
            }

            Parse::Done(left) => Parse::Partial(op, left),

            _ => panic!("aaaah!"),
        };
        stack.push(parse);
    }

    fn parse_expr(stack: &mut Vec<Parse>, expr: Expr) {
        let expr = match stack.pop().unwrap() {
            Parse::Open => {
                stack.push(Parse::Open);
                expr
            }

            Parse::Partial(op, left) => Expr::Op(op, Box::new(left), Box::new(expr)),

            _ => panic!("aaaah!"),
        };
        stack.push(Parse::Done(expr));
    }
}

impl Day for Day18 {
    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .map(|s| Self::solve::<SamePriority>(s))
            .sum::<usize>()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        self.input
            .iter()
            .map(|s| Self::solve::<AddFirst>(s))
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Op(Op, Box<Expr>, Box<Expr>),
    Num(usize),
}

impl Expr {
    fn value(&self) -> usize {
        match self {
            Expr::Op(op, left, right) => op.exec(left.value(), right.value()),
            Expr::Num(num) => *num,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn exec(&self, left: usize, right: usize) -> usize {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

#[derive(Debug)]
enum Parse {
    Open,
    Partial(Op, Expr),
    Done(Expr),
}

trait OpPriority {
    fn priority(op: &Op) -> u8;
}

struct SamePriority;

impl OpPriority for SamePriority {
    fn priority(op: &Op) -> u8 {
        match op {
            Op::Add => 1,
            Op::Mul => 1,
        }
    }
}

struct AddFirst;

impl OpPriority for AddFirst {
    fn priority(op: &Op) -> u8 {
        match op {
            Op::Add => 2,
            Op::Mul => 1,
        }
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let day = Day18::load(input);
        assert_eq!(
            day.first_challenge(),
            (26 + 437 + 12240 + 13632).to_string()
        );
    }

    #[test]
    fn test_second_challenge() {
        let input = "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let day = Day18::load(input);
        assert_eq!(
            day.second_challenge(),
            (46 + 1445 + 669060 + 23340).to_string()
        );
    }
}
