#[derive(Debug, Default, PartialEq)]
pub struct StateAutomata {
    pub state: u8
}

// Graph of the state automata:
// _______________________
// | from | to   | digit |
// | -----|----- |------ |
// | S000 | S001 | 1     |
// | S001 | S011 | 1     |
// | S010 | S101 | 1     |
// | S100 | S001 | 1     |
// | S101 | S011 | 1     |
// | S011 | S111 | 1     |
// | S110 | S101 | 1     |
// | S111 | S111 | 1     |
// | S000 | S000 | 0,2   |
// | S001 | S010 | 0,2   |
// | S010 | S100 | 0,2   |
// | S100 | S000 | 0,2   |
// | S101 | S010 | 0,2   |
// | S011 | S110 | 0,2   |
// | S110 | S100 | 0,2   |
// | S111 | S110 | 0,2   |
// -----------------------
// Accepts only words where 1 is 3rd from last
// Final state is S3
impl StateAutomata {
    pub fn next(&mut self, digit: u8) {
        self.state = (self.state << 1) & 0b111 | ( digit == 1 ) as u8;
    }

    pub fn is_final(&self) -> bool {
        self.state & 0b100 != 0
    }

    pub fn process_input(input: Vec<u8>) -> bool {
        let mut state_automata = StateAutomata::default();
        for digit in input {
            state_automata.next(digit);
        }
        state_automata.is_final()
    }
}

fn main() {
    let mut state_automata = StateAutomata::default();
    let input = vec![1, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 1, 0, 2];
    for digit in input {
        state_automata.next(digit);
        if state_automata.is_final() {
            println!("Final state reached");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_words_where_1_is_3rd_from_last() {
        assert_eq!(StateAutomata::process_input(vec![1, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 1, 0, 2]), true);
        assert_eq!(StateAutomata::process_input(vec![1, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 2, 0, 2]), false);
        assert_eq!(StateAutomata::process_input(vec![1, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 0, 2]), false);
        assert_eq!(StateAutomata::process_input(vec![1, 1, 0, 1, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0]), false);
        assert_eq!(StateAutomata::process_input(vec![0]), false);
        assert_eq!(StateAutomata::process_input(vec![1]), false);
        assert_eq!(StateAutomata::process_input(vec![2]), false);

    }
}