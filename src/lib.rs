mod insertion_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::insertion_sort;
        let vec = vec![3,2,4];
        let sorted = insertion_sort::sort(vec);
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 2);
        assert_eq!(sorted[1], 3);
        assert_eq!(sorted[2], 4);
    }
}
