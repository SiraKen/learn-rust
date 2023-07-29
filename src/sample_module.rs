pub mod greet {
    //! モジュールに対してもこのようにコメントを記述できる
    
    /// 関数単位には /// スラッシュ3つでコメントする
    pub fn english(name: &str) -> String {
        format!("Hello, {}", name)
    }

    #[test]
    fn test_english() {
        assert_eq!(english("John"), "Hello, John")
    }
}
