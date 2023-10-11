mod greedy_delta_v2;

use greedy_delta_v2::Queue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut queue: Queue<usize> = Queue::new();
        let pushed_value = 8;
        queue.push(pushed_value);
        let popped_value = queue.pop().unwrap();
        assert_eq!(pushed_value, popped_value);
    }
}
