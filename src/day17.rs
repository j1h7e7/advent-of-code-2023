use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

fn get_grid(document: &str) -> Vec<Vec<usize>> {
    return document
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}
impl Direction {
    fn opposite(&self) -> Direction {
        return match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        };
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct State {
    pos: (isize, isize),
    direction: Direction,
    steps: usize,
}
impl Ord for State {
    fn cmp(&self, _other: &Self) -> Ordering {
        return Ordering::Equal;
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn get_next_steps(state: State, bounds: (isize, isize)) -> Vec<State> {
    let (x, y) = state.pos;
    let (max_x, max_y) = bounds;

    let mut next_states = vec![];

    for direction in vec![Direction::N, Direction::E, Direction::S, Direction::W] {
        if direction == state.direction && state.steps >= 3 {
            continue;
        }
        if direction == state.direction.opposite() {
            continue;
        }
        let (next_x, next_y) = match direction {
            Direction::N => (x - 1, y),
            Direction::E => (x, y + 1),
            Direction::S => (x + 1, y),
            Direction::W => (x, y - 1),
        };
        if next_x < 0 || next_x >= max_x || next_y < 0 || next_y >= max_y {
            continue;
        }
        next_states.push(State {
            pos: (next_x, next_y),
            direction,
            steps: if direction == state.direction {
                state.steps + 1
            } else {
                1
            },
        });
    }

    return next_states;
}

fn get_next_steps_2(state: State, bounds: (isize, isize)) -> Vec<State> {
    let (x, y) = state.pos;
    let (max_x, max_y) = bounds;

    let mut next_states = vec![];

    for direction in vec![Direction::N, Direction::E, Direction::S, Direction::W] {
        if direction == state.direction && state.steps >= 10 {
            continue;
        }
        if direction != state.direction && state.steps < 4 && state.pos != (0, 0) {
            continue;
        }
        if direction == state.direction.opposite() {
            continue;
        }
        let (next_x, next_y) = match direction {
            Direction::N => (x - 1, y),
            Direction::E => (x, y + 1),
            Direction::S => (x + 1, y),
            Direction::W => (x, y - 1),
        };
        if next_x < 0 || next_x >= max_x || next_y < 0 || next_y >= max_y {
            continue;
        }
        next_states.push(State {
            pos: (next_x, next_y),
            direction,
            steps: if direction == state.direction {
                state.steps + 1
            } else {
                1
            },
        });
    }

    return next_states;
}

fn get_fastest_path(
    document: &str,
    step_generator: &dyn Fn(State, (isize, isize)) -> Vec<State>,
    completion_condition: &dyn Fn(State) -> bool,
) -> String {
    let grid = get_grid(document);
    let bounds = (grid.len() as isize, grid[0].len() as isize);

    let mut queue: BinaryHeap<(isize, State)> = BinaryHeap::new();
    let mut visited: HashSet<State> = HashSet::new();
    queue.push((
        0,
        State {
            pos: (0, 0),
            direction: Direction::S,
            steps: 0,
        },
    ));

    loop {
        let (cost, state) = queue.pop().unwrap();
        if state.pos == (bounds.0 - 1, bounds.1 - 1) && completion_condition(state) {
            return (-cost).to_string();
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        let next_states = step_generator(state, bounds);
        for next_state in next_states {
            queue.push((
                cost - grid[next_state.pos.0 as usize][next_state.pos.1 as usize] as isize,
                next_state,
            ));
        }
    }
}

pub struct Day17Puzzle {}
impl super::solve::Puzzle<String> for Day17Puzzle {
    fn solve(&self, document: &str) -> String {
        return get_fastest_path(document, &get_next_steps, &|_| true);
    }

    fn solve2(&self, document: &str) -> String {
        return get_fastest_path(document, &get_next_steps_2, &|state| state.steps >= 4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_grid() {
        let document = "123\n456\n789";
        let grid = get_grid(document);
        assert_eq!(grid, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_get_next_steps() {
        let bounds = (3, 3);
        let state = State {
            pos: (1, 1),
            direction: Direction::N,
            steps: 0,
        };
        let next_states = get_next_steps(state, bounds);
        assert_eq!(
            next_states,
            vec![
                State {
                    pos: (0, 1),
                    direction: Direction::N,
                    steps: 1
                },
                State {
                    pos: (1, 2),
                    direction: Direction::E,
                    steps: 1
                },
                State {
                    pos: (1, 0),
                    direction: Direction::W,
                    steps: 1
                }
            ]
        );
    }
}
